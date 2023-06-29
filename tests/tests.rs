use lovense_rust::*;

// Write individual test functions
#[tokio::test]
async fn test_vibrate() {
    let client = LovenseClient { local_ip: String::from("localhost"), http_port: 8000 };
    client.single_vibrate(13, 20.0, None, true).await;
}
