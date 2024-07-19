use slack_webhook::*;
use anyhow::Result;

#[tokio::test]
async fn test_fake_send_ok() -> Result<()> {
    let slack = SlackWebhook::fake();
    let v = slack.send_text("fuck you").await?;
    let txt = v.get("text").unwrap();
    assert_eq!(txt, &serde_json::to_value("fuck you").unwrap());
    Ok(())
}

#[tokio::test]
async fn test_prefix() -> Result<()> {
    let slack = SlackWebhook::fake().prefix("[staging]");
    let v = slack.send_text("fuck you").await?;
    let txt = v.get("text").unwrap();
    assert_eq!(txt, &serde_json::to_value("[staging] fuck you").unwrap());
    Ok(())
}

#[test]
fn test_url_error() {
    let slack = SlackWebhook::new("fucking.url.com");
    assert!(slack.is_err());
}

#[tokio::test]
async fn test_send_error() -> Result<()> {
    let non_existing_url = "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX";
    let slack = SlackWebhook::new(non_existing_url)?;
    let resp = slack.send_text("fuck you").await;
    assert!(resp.is_err());
    Ok(())
}