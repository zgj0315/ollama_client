use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_line_number(true).init();
    let client = reqwest::ClientBuilder::new().build()?;
    // let url = "http://127.0.0.1:11434/api/tags";
    // let rsp = client.get(url).send().await?;
    // let text = rsp.text().await?;
    // let json: serde_json::Value = serde_json::from_str(&text)?;
    // let str_pretty = serde_json::to_string_pretty(&json)?;
    // log::info!("{}", str_pretty);

    // let url = "http://127.0.0.1:11434/api/generate";
    // let req_body = serde_json::json!({
    //   "model": "llama3.1",
    //   "prompt": "Why is the sky blue?"
    // });
    // log::info!("\nurl: {}\nreq_body: {}", url, req_body);
    // let rsp = client.post(url).body(req_body.to_string()).send().await?;
    // if rsp.status().is_success() {
    //     let mut stream = rsp.bytes_stream();
    //     while let Some(chunk) = stream.next().await {
    //         match chunk {
    //             Ok(bytes) => {
    //                 let json: serde_json::Value = serde_json::from_slice(&bytes)?;
    //                 let content = json["response"].as_str().unwrap_or_default();
    //                 print!("{}", content);
    //                 // log::info!("{}", content);
    //             }
    //             Err(e) => {
    //                 eprintln!("Error while reading chunk: {:?}", e);
    //             }
    //         }
    //     }
    // } else {
    //     log::error!("Request failed with status: {}", rsp.status());
    // }

    // let req_body = serde_json::json!({
    //   "model": "llama3.1",
    //   "prompt": "Why is the sky blue?",
    //   "stream": false
    // });
    // let req_body = serde_json::json!({
    //   "model": "codellama:code",
    //   "prompt": "def compute_gcd(a, b):",
    //   "suffix": "    return result",
    //   "options": {
    //     "temperature": 0
    //   },
    //   "stream": false
    // });
    // log::info!("\nurl: {}\nreq_body: {}", url, req_body);
    // let rsp = client.post(url).body(req_body.to_string()).send().await?;
    // let text = rsp.text().await?;
    // let json: serde_json::Value = serde_json::from_str(&text)?;
    // let str_pretty = serde_json::to_string_pretty(&json)?;
    // log::info!("{}", str_pretty);

    // let url = "http://127.0.0.1:11434/api/chat";
    // let req_body = serde_json::json!({
    // //   "model": "gemma2:27b",
    // "model": "llama3.1",
    //   "messages": [
    //     {
    //       "role": "user",
    //       "content": "why is the sky blue?"
    //     }
    //   ]
    // });
    // let req_body = serde_json::json!({
    //   "model": "llama3.1",
    //   "messages": [
    //     {
    //       "role": "user",
    //       "content": "why is the sky blue?"
    //     }
    //   ]
    // });
    // log::info!("\nurl: {}\nreq_body: {}", url, req_body);
    // let rsp = client.post(url).body(req_body.to_string()).send().await?;
    // if rsp.status().is_success() {
    //     let mut stream = rsp.bytes_stream();
    //     while let Some(chunk) = stream.next().await {
    //         match chunk {
    //             Ok(bytes) => {
    //                 let json: serde_json::Value = serde_json::from_slice(&bytes)?;
    //                 let content = json["message"]["content"].as_str().unwrap_or_default();
    //                 print!("{}", content);
    //                 // log::info!("{}", content);
    //             }
    //             Err(e) => {
    //                 eprintln!("Error while reading chunk: {:?}", e);
    //             }
    //         }
    //     }
    // } else {
    //     log::error!("Request failed with status: {}", rsp.status());
    // }
    let url = "http://127.0.0.1:11434/api/chat";
    let req_body = serde_json::json!({
      "model": "gemma2:27b",
      // "model": "llama3.1",
      // "prompt": "我是一个漏洞安全工程师，针对漏洞信息进行中向英文的转换",
      "messages": [
        {
          "role": "user",
          "content": "请将这段文字翻译成英文：Apache OFBiz中的服务器端请求伪造（SSRF）和代码生成控制不当（'代码注入'）漏洞。"
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
