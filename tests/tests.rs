use lovense_rust::client::LovenseClient;
use lovense_rust::server::create_qr_code;

// Write individual test functions
#[tokio::test]
async fn test_vibrate() {
    let client = LovenseClient::new("192.168.0.92", 20010);
    client.single_vibrate(13, 20.0, None, true).await;
}

#[tokio::test]
async fn test_create_qr_code() {
    match create_qr_code("eJFamEJC_aIzOFF7L-jANpY2XW2_RiwU8jboteQw-kWDYaYrON_vu7uMMPxEZ2gW", "123456789", "test").await {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e)
    }
}