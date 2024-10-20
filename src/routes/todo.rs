use poem_openapi::{param::Query, payload::PlainText, OpenApi, Tags};

#[derive(Tags)]
enum ApiTodoTags {
    /// CRUD Todo implementation
    Todo,
}

pub struct ApiTodo;

#[OpenApi]
impl ApiTodo {
    #[oai(path = "/todo", method = "get", tag = "ApiTodoTags::Todo")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}
