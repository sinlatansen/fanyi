# fanyi 翻译 CLI 教学计划

## 项目简介
`fanyi` 是一个 Rust 编写的命令行翻译工具，支持中英文互译。

**使用方法：**
```bash
fanyi hello      # 输出中文翻译
fanyi 你好       # 输出英文翻译
```

## 项目状态

### 已完成 ✓
1. **命令行参数解析** (clap)
   - 使用 derive 宏定义 CLI 接口
   - 支持接收单个字符串参数
   
2. **语言检测**
   - 检测输入文本的第一个字符
   - 判断是否为中文 (Unicode 范围: `\u{4e00}` - `\u{9fff}`)
   - 决定翻译方向（中文→英文 / 英文→中文）

3. **依赖配置**
   - `clap`: 命令行参数解析
   - `reqwest`: HTTP 客户端（异步）
   - `tokio`: 异步运行时
   - `serde`: 序列化/反序列化
   - `serde_json`: JSON 处理

4. **API 响应结构定义**
   - `ApiResponse`: 顶层响应
   - `Choice`: 选择列表
   - `Message`: 消息内容

5. **异步主函数**
   - 使用 `#[tokio::main]`
   - 返回 `Result` 类型处理错误

### 待完成 ⏳
1. **实现 translate 函数**
   - 从环境变量读取 `DEEPSEEK_API_KEY`
   - 根据语言检测结果构建 system prompt
   - 发送 HTTP POST 请求到 DeepSeek API
   - 解析 JSON 响应并返回翻译结果

2. **错误处理**
   - API 调用失败
   - 网络错误
   - 无效 API Key

3. **测试与优化**
   - 实际调用 API 测试
   - 处理边界情况（空字符串、特殊字符等）

## 技术要点

### 核心概念
- **clap derive macro**: 声明式 CLI 定义
- **Rust 异步编程**: async/await + tokio 运行时
- **HTTP 请求**: reqwest 发送 POST 请求
- **JSON 处理**: serde 序列化/反序列化
- **环境变量**: 安全存储 API Key

### 关键代码片段

```rust
// 语言检测
fn is_chinese(text: &str) -> bool {
    text.chars().next()
        .map(|c| c >= '\u{4e00}' && c <= '\u{9fff}')
        .unwrap_or(false)
}

// 异步主函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...
}
```

## 下一步行动

### 任务：实现 translate 函数

**步骤：**
1. 获取 API Key: `std::env::var("DEEPSEEK_API_KEY")?`
2. 构建 system prompt（根据输入语言决定目标语言）
3. 创建 HTTP 客户端: `reqwest::Client::new()`
4. 构建请求体（JSON）
5. 发送 POST 请求
6. 解析响应并提取翻译文本

**API Endpoint:**
```
POST https://api.deepseek.com/chat/completions
```

**请求示例：**
```json
{
  "model": "deepseek-chat",
  "messages": [
    {"role": "system", "content": "将用户输入翻译成中文。只输出翻译结果。"},
    {"role": "user", "content": "hello"}
  ]
}
```

---

**最后更新：** 2026-02-19
