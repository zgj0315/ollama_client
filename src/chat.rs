#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_line_number(true).init();
    let client = reqwest::ClientBuilder::new().build()?;
    let url = "http://127.0.0.1:11434/api/chat";
    let req_body = serde_json::json!({
      "model": "gemma2:27b",
      "messages": [
        {
          "role": "system",
          "content": "你是一个专业的漏洞研究专家、计算机安全专家、中英文翻译专家；
          根据输入内容进行分析，给出分析思路和过程，并输出中英文漏洞标题；
          标题中包含CVE信息；
          输出内容格式：**分析思路**\nxxx\n**分析过程**\nxxx\n**中文标题**\nxxx\n**英文标题**\nxxx\n"
        },
        {
          "role": "user",
          // "content": "
          // CVE: CVE-2024-21733
          // Description: Generation of Error Message Containing Sensitive Information vulnerability in Apache Tomcat.This issue affects Apache Tomcat: from 8.5.7 through 8.5.63, from 9.0.0-M11 through 9.0.43. Users are recommended to upgrade to version 8.5.64 onwards or 9.0.44 onwards, which contain a fix for the issue.
          // "
          // "content": "
          // CVE: CVE-2019-17558
          // Description: Apache Solr 5.0.0 to Apache Solr 8.3.1 are vulnerable to a Remote Code Execution through the VelocityResponseWriter. A Velocity template can be provided through Velocity templates in a configset `velocity/` directory or as a parameter. A user defined configset could contain renderable, potentially malicious, templates. Parameter provided templates are disabled by default, but can be enabled by setting `params.resource.loader.enabled` by defining a response writer with that setting set to `true`. Defining a response writer requires configuration API access. Solr 8.4 removed the params resource loader entirely, and only enables the configset-provided template rendering when the configset is `trusted` (has been uploaded by an authenticated user).
          // "
          "content": "
          CVE: CVE-2017-3066
          Description: Adobe ColdFusion 2016 Update 3 and earlier, ColdFusion 11 update 11 and earlier, ColdFusion 10 Update 22 and earlier have a Java deserialization vulnerability in the Apache BlazeDS library. Successful exploitation could lead to arbitrary code execution.
          "
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
