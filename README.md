## A Heatmap Generator CLI for Any Github Repository You Want To Access (Include Ranking)
Contributor Checker
🚀 一个 Rust CLI 工具，可以获取 GitHub 项目的贡献情况，并显示类似 GitHub 主页贡献图的彩色网格。

# Contributor Checker

🚀 一个 Rust CLI 工具，可以获取 GitHub 项目的贡献情况，并显示类似 GitHub 主页贡献图的彩色网格。

## 📦 安装

### 1. 克隆代码 Clone
```sh
git clone https://github.com/eR3R3/contributor-checker.git
cd contributor_checker

cargo build --release

sudo mv ./target/release/cchecker /usr/local/bin/cchecker

sudo mkdir -p /usr/local/bin
```

### 2. 使用方法 Usage
```
cchecker <GitHub用户名>

cchecker apple

cargo run --release -- <GitHub用户名>

sudo rm /usr/local/bin/cchecker
```

## 欢迎 PR 或 Issue！🎉

📧 联系方式：er1r1@qq.com