use reqwest::StatusCode;
use serde_json::Value as JsonValue;

use crate::conftest::TestApp;

#[tokio::test]
async fn test_modes_returns_200() -> anyhow::Result<()> {
    // Arrange
    let app = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();
    // Act
    let response = client
        // Use the returned application address
        .get(app.modes())
        .send()
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let data: JsonValue = response.json().await?;
    insta::assert_json_snapshot!(data);
    Ok(())
}
