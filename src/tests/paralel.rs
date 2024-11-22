use std::time::Duration;

use tokio::time::sleep;

#[ignore = "this is just an example"]
#[tokio::test]
async fn test_1() {
    println!("test 1 start");
    sleep(Duration::from_millis(1000)).await;
    assert!(true);
    println!("test 1 end");
}

#[ignore = "this is just an example"]
#[tokio::test]
async fn test_2() {
    println!("test 2 start");
    sleep(Duration::from_millis(2000)).await;
    assert!(true);
    println!("test 2 end");
}

#[ignore = "this is just an example"]
#[tokio::test]
async fn test_3() {
    println!("test 3 start");
    sleep(Duration::from_millis(3000)).await;
    assert!(true);
    println!("test 3 end");
}

#[ignore = "this is just an example"]
#[tokio::test]
async fn test_4() {
    println!("test 4 start");
    sleep(Duration::from_millis(4000)).await;
    assert!(true);
    println!("test 4 end");
}

#[ignore = "this is just an example"]
#[tokio::test]
async fn test_5() {
    println!("test 5 start");
    sleep(Duration::from_millis(5000)).await;
    assert!(true);
    println!("test 5 end");
}
