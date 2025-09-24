use reqwest::StatusCode;
use rstest::rstest;
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

#[rstest]
#[case("panacea")]
#[tokio::test]
async fn test_modeinfo_returns_200(#[case] mode: &str) -> anyhow::Result<()> {
    // Arrange
    let app = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();
    // Act
    let response = client.get(app.modeinfo(mode)).send().await?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let data: JsonValue = response.json().await?;
    insta::assert_json_snapshot!(format!("test_modeinfo_returns_200-{mode}"), data);
    Ok(())
}
