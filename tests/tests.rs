use lovense_rust::client::LovenseClient;
use lovense_rust::server::LovenseServer;

// Write individual test functions
#[tokio::test]
async fn test_vibrate() {
    let client = LovenseClient::new("192.168.0.92", 20010);
    client.single_vibrate(13, 20.0, None, true).await;
}

#[tokio::test]
async fn test_create_qr_code() {
    let client = LovenseServer::new(String::from("eJFamEJC_aIzOFF7L-jANpY2XW2_RiwU8jboteQw-kWDYaYrON_vu7uMMPxEZ2gW"));
    if let Ok(res) = client.create_qr_code("123456789", "test").await {
        println!("{:?}", res);
    };
}