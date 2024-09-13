use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_line_number(true).init();
    let ollama = Ollama::new("http://127.0.0.1".to_string(), 11434);
    let model = "llama3.1".to_string();
    let prompt = "Why is the sky blue?".to_string();
    // let prompt =
    //     "将这段文字翻译成英文：Apache Tomcat存在生成包含敏感信息的错误消息的漏洞。".to_string();
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;
    if let Ok(res) = res {
        log::info!("{}", res.response);
    }
    Ok(())
}
