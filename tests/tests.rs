use lovense_rust::vibrate;

// Write individual test functions
#[tokio::test]
async fn test_vibrate() {
    // Test case 1
    vibrate(2, 3.0, true).await;
}
