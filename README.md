
# Contributor Checker
## A Heatmap Generator CLI for Any Github Repository You Want To Access (Include Ranking)
🚀 一个 Rust CLI 工具，可以获取 GitHub 项目的贡献情况，并显示类似 GitHub 主页贡献图的彩色网格。


## Demo 
### Use Example

<img width="966" alt="image" src="https://github.com/user-attachments/assets/11f6fc24-88e3-4299-8f55-f5398b27f51d" />

### Rejection Example

<img width="942" alt="image" src="https://github.com/user-attachments/assets/602b4576-ca10-4913-8a98-6d2165ed2fa3" />


## 📦 安装

### 1. 克隆代码 Clone
```
git clone https://github.com/eR3R3/contributor-checker.git

cd contributor-checker

cargo build --release

sudo mv ./target/release/cchecker /usr/local/bin/cchecker

sudo mkdir -p /usr/local/bin
```

### 2. 使用方法 Usage
```
cchecker <Repo Owner>/<GitHub Repo> <Github User>
#Basic Syntax

cchecker <Repo Owner>/<Github Repo>
#See All the Contributors with Commit Ranking

cchecker 
#Check the Current Remote Repo
```

## 欢迎 PR 或 Issue！🎉

📧 电子邮箱 Email：er1r1@qq.com
