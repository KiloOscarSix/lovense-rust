use std::env;
use lovense_rust::client::LovenseClient;
use lovense_rust::server::create_qr_code;
use lovense_rust::strength::Strength;

// Write individual test functions
#[tokio::test]
async fn test_vibrate() {
    let client = LovenseClient::new("192.168.0.92", 20010);
    client.single_vibrate(Strength::Eight, 20.0, None, true).await.expect("Failed to vibrate");
}

#[tokio::test]
async fn test_create_qr_code() {
    dotenvy::dotenv().ok();

    match create_qr_code(
        &env::var("LOVENSE_TOKEN").expect("Expected a lovense token"),
        "123456789",
        "test"
    ).await {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e)
    }
}