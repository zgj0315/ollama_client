#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_line_number(true).init();
    let client = reqwest::ClientBuilder::new().build()?;
    let url = "http://127.0.0.1:11434/api/chat";
    // let req_body = serde_json::json!({
    //   "model": "llama3.1",
    //   "messages": [
    //     {
    //       "role": "user",
    //       "content": "why is the sky blue?"
    //     }
    //   ],
    //   "stream": false
    // });
    let req_body = serde_json::json!({
      // "model": "llama3.1",
      "model": "gemma2:27b",
      "messages": [
        {
          "role": "system",
          // "content": "你是一个专业的漏洞研究工程师，你是一个专业的计算机安全工程师，你是一个专业的中英文翻译工程师，你将会把输入的内容翻译成中文"
          "content": "You are a professional vulnerability research engineer, you are a professional computer security engineer, you are a professional Chinese-English translation engineer, you will translate the input into Chinese"
        },
        {
          "role": "user",
          "content": "Generation of Error Message Containing Sensitive Information vulnerability in Apache Tomcat.This issue affects Apache Tomcat: from 8.5.7 through 8.5.63, from 9.0.0-M11 through 9.0.43. Users are recommended to upgrade to version 8.5.64 onwards or 9.0.44 onwards, which contain a fix for the issue."
        }
      ],
      "stream": false
    });
    log::info!("\nurl: {}\nreq_body: {}", url, req_body);
    let rsp = client.post(url).body(req_body.to_string()).send().await?;
    let text = rsp.text().await?;
    let json: serde_json::Value = serde_json::from_str(&text)?;
    // let str_pretty = serde_json::to_string_pretty(&json)?;
    // log::info!("{}", str_pretty);
    let content = json["message"]["content"].as_str().unwrap_or_default();
    log::info!("{}", content);
    Ok(())
}
