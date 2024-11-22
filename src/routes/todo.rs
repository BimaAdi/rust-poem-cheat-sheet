use std::sync::Arc;

use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi, Tags,
};

use crate::{
    schema::{
        common::{InternalServerErrorResponse, NotFoundResponse, OkResponse, PaginateResponse},
        todo::{
            TodoCreateOk, TodoCreateRequest, TodoCreateResponses, TodoDeleteResponses,
            TodoDetailFound, TodoDetailResponses, TodoPaginateDetail, TodoPaginateResponses,
            TodoUpdateRequest, TodoUpdateResponse, TodoUpdateResponses,
        },
    },
    utils::div_ceil,
    AppState,
};

#[derive(Tags)]
enum ApiTodoTags {
    /// CRUD Todo implementation
    Todo,
}

pub struct ApiTodo;

#[OpenApi]
impl ApiTodo {
    #[oai(path = "/todo", method = "get", tag = "ApiTodoTags::Todo")]
    async fn get_paginate_todo(
        &self,
        page: Query<Option<i32>>,
        page_size: Query<Option<i32>>,
        state: Data<&Arc<AppState>>,
    ) -> TodoPaginateResponses {
        let page = page.0.unwrap_or(1);
        let page_size = page_size.0.unwrap_or(5);
        let limit = page_size;
        let offset = (page - 1) * page_size;

        let num_data: (i32,) = match sqlx::query_as("SELECT count(id) FROM todo")
            .fetch_one(&state.db)
            .await
        {
            Ok(x) => x,
            Err(err) => {
                return TodoPaginateResponses::InternalServerError(Json(
                    InternalServerErrorResponse::new(
                        "routes/todo.rs",
                        "get_paginate_todo",
                        "count todo on db",
                        err.to_string().as_str(),
                    ),
                ))
            }
        };
        let num_data = num_data.0;

        let data: Vec<(i32, String, i32)> = match sqlx::query_as(
            "SELECT id, todo, is_done FROM todo ORDER BY id DESC LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        {
            Ok(x) => x,
            Err(err) => {
                return TodoPaginateResponses::InternalServerError(Json(
                    InternalServerErrorResponse::new(
                        "routes/todo.rs",
                        "get_paginate_todo",
                        "find todo on db",
                        err.to_string().as_str(),
                    ),
                ))
            }
        };
        let num_page = div_ceil(num_data, page_size);
        TodoPaginateResponses::Ok(Json(PaginateResponse {
            page,
            page_size,
            num_data,
            num_page,
            results: data
                .iter()
                .map(|f| TodoPaginateDetail {
                    id: f.0,
                    todo: f.1.clone(),
                    is_done: f.2 == 1,
                })
                .collect(),
        }))
    }

    #[oai(path = "/todo/:id", method = "get", tag = "ApiTodoTags::Todo")]
    async fn get_detail_todo(
        &self,
        id: Path<i32>,
        state: Data<&Arc<AppState>>,
    ) -> TodoDetailResponses {
        let data: Option<(i32, String, i32)> =
            match sqlx::query_as("SELECT id, todo, is_done FROM todo WHERE id = ?")
                .bind(id.0)
                .fetch_optional(&state.db)
                .await
            {
                Ok(x) => x,
                Err(err) => {
                    return TodoDetailResponses::InternalServerError(Json(
                        InternalServerErrorResponse::new(
                            "routes/todo.rs",
                            "get_detail_todo",
                            "find todo on db",
                            err.to_string().as_str(),
                        ),
                    ))
                }
            };

        if let Some(x) = data {
            TodoDetailResponses::Ok(Json(TodoDetailFound {
                id: x.0,
                todo: x.1,
                is_done: x.2 == 1,
            }))
        } else {
            TodoDetailResponses::NotFound(Json(NotFoundResponse {
                message: format!("todo with id {} not found", id.0),
            }))
        }
    }

    #[oai(path = "/todo", method = "post", tag = "ApiTodoTags::Todo")]
    async fn create_todo(
        &self,
        json: Json<TodoCreateRequest>,
        state: Data<&Arc<AppState>>,
    ) -> TodoCreateResponses {
        let id = sqlx::query("INSERT INTO todo (todo, is_done) VALUES (?, ?)")
            .bind(json.todo.clone())
            .bind(json.is_done)
            .execute(&state.db)
            .await
            .unwrap()
            .last_insert_rowid();

        let data: (i32, String, i32) =
            sqlx::query_as("SELECT id, todo, is_done FROM todo WHERE id = ?")
                .bind(id)
                .fetch_one(&state.db)
                .await
                .unwrap();

        TodoCreateResponses::Ok(Json(TodoCreateOk {
            id: data.0,
            todo: data.1,
            is_done: data.2 == 1,
        }))
    }

    #[oai(path = "/todo/:id", method = "put", tag = "ApiTodoTags::Todo")]
    async fn update_todo(
        &self,
        id: Path<i32>,
        json: Json<TodoUpdateRequest>,
        state: Data<&Arc<AppState>>,
    ) -> TodoUpdateResponses {
        let data: Option<(i32, String, i32)> =
            match sqlx::query_as("SELECT id, todo, is_done FROM todo WHERE id = ?")
                .bind(id.0)
                .fetch_optional(&state.db)
                .await
            {
                Ok(x) => x,
                Err(err) => {
                    return TodoUpdateResponses::InternalServerError(Json(
                        InternalServerErrorResponse::new(
                            "routes/todo.rs",
                            "update_todo",
                            "find todo on db",
                            err.to_string().as_str(),
                        ),
                    ))
                }
            };

        if data.is_none() {
            return TodoUpdateResponses::NotFound(Json(NotFoundResponse {
                message: format!("todo with id {} not found", id.0),
            }));
        }

        if let Err(err) = sqlx::query("UPDATE todo SET todo = ?, is_done = ? WHERE id = ?")
            .bind(&json.todo)
            .bind(json.is_done)
            .bind(id.0)
            .execute(&state.db)
            .await
        {
            return TodoUpdateResponses::InternalServerError(Json(
                InternalServerErrorResponse::new(
                    "routes/todo.rs",
                    "update_todo",
                    format!("update todo on db with id = {}", id.0).as_str(),
                    err.to_string().as_str(),
                ),
            ));
        }

        TodoUpdateResponses::Ok(Json(TodoUpdateResponse {
            id: id.0,
            todo: json.todo.clone(),
            is_done: json.is_done,
        }))
    }

    #[oai(path = "/todo/:id", method = "delete", tag = "ApiTodoTags::Todo")]
    async fn delete_todo(&self, id: Path<i32>, state: Data<&Arc<AppState>>) -> TodoDeleteResponses {
        let data: Option<(i32, String, i32)> =
            match sqlx::query_as("SELECT id, todo, is_done FROM todo WHERE id = ?")
                .bind(id.0)
                .fetch_optional(&state.db)
                .await
            {
                Ok(x) => x,
                Err(err) => {
                    return TodoDeleteResponses::InternalServerError(Json(
                        InternalServerErrorResponse::new(
                            "routes/todo.rs",
                            "delete_todo",
                            "find todo on db",
                            err.to_string().as_str(),
                        ),
                    ))
                }
            };

        if data.is_none() {
            return TodoDeleteResponses::NotFound(Json(NotFoundResponse {
                message: format!("todo with id {} not found", id.0),
            }));
        }

        if let Err(err) = sqlx::query("DELETE FROM todo WHERE id = ?")
            .bind(id.0)
            .execute(&state.db)
            .await
        {
            return TodoDeleteResponses::InternalServerError(Json(
                InternalServerErrorResponse::new(
                    "routes/todo.rs",
                    "delete_todo",
                    "delete todo",
                    err.to_string().as_str(),
                ),
            ));
        }

        TodoDeleteResponses::Ok(Json(OkResponse {
            message: "todo deleted successfully".to_string(),
        }))
    }
}
