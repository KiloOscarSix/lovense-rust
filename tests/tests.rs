use lovense_rust::*;

// Write individual test functions
#[tokio::test]
async fn test_vibrate() {
    let client = LovenseClient::new("192.168.0.92", 20010);
    client.single_vibrate(13, 20.0, None, true).await;
}
