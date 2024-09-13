# ollama_client
ollama http client with rust


```
curl http://localhost:11434/api/create -d '{
  "name": "llama_for_vul",
  "modelfile": "FROM llama3.1\nSYSTEM 你是一个专业的漏洞研究专家、计算机安全专家、中英文翻译专家；根据输入内容进行分析，给出分析思路和过程，并输出中英文漏洞标题；标题中包含CVE信息"
}'
```