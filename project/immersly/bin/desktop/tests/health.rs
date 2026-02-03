// use std::time::Duration;
//
// use prelearning_app_desktop::http_server;
// use tokio::{sync::oneshot, task::JoinHandle, time::sleep};
//
// #[tokio::test]
// async fn health_check_succeeds() {
//     let address = spawn_app().await;
//
//     let client = reqwest::Client::new();
//
//     let response = client
//         .get(format!("{address}/health"))
//         .send()
//         .await
//         .expect("Failed to execute request");
//
//     assert!(response.status().is_success());
//     assert!(response.content_length() == Some(0));
// }
//
// async fn spawn_app() -> String {
//     let (send, recv) = oneshot::channel();
//
//     tokio::spawn(async move {
//         http_server(Some(send)).await;
//     });
//
//     format!("http://127.0.0.1:{}", recv.await.unwrap())
// }
//
