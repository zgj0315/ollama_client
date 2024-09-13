#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_line_number(true).init();
    let client = reqwest::ClientBuilder::new().build()?;
    let url = "http://127.0.0.1:11434/api/chat";
    let req_body = serde_json::json!({
      "model": "llama3.1",
      "messages": [
        {
          "role": "user",
          "content": "why is the sky blue?"
        }
      ],
      "stream": false
    });
    log::info!("\nurl: {}\nreq_body: {}", url, req_body);
    let rsp = client.post(url).body(req_body.to_string()).send().await?;
    let text = rsp.text().await?;
    let json: serde_json::Value = serde_json::from_str(&text)?;
    let str_pretty = serde_json::to_string_pretty(&json)?;
    log::info!("{}", str_pretty);
    Ok(())
}
