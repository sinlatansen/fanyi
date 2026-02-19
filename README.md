# fanyi - 命令行翻译工具

一个轻量级的中英文命令行翻译工具，基于 Rust 开发，使用 MyMemory 免费翻译 API。

## 特性

- **自动语言检测** - 自动识别中英文，无需手动指定源语言
- **支持多词翻译** - 可以翻译完整句子，不只是单个单词
- **零配置** - 无需 API Key，开箱即用
- **跨平台** - 支持 Windows、macOS、Linux

## 安装

### 从源码编译

```bash
git clone https://github.com/sinlatansen/fanyi.git
cd fanyi
cargo install --path .
```

### 要求

- Rust 1.70+

## 用法

```bash
# 英文转中文
fy hello
# 输出: 你好

# 中文转英文
fy 你好
# 输出: Hello

# 翻译句子
fy how are you today
# 输出: 你今天好吗

fy 今天天气真好
# 输出: The weather is really nice today
```

## 技术栈

- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP 客户端
- [tokio](https://github.com/tokio-rs/tokio) - 异步运行时
- [serde](https://github.com/serde-rs/serde) - 序列化/反序列化

## 致谢

- 翻译服务由 [MyMemory](https://mymemory.translated.net/) 提供

## License

MIT
