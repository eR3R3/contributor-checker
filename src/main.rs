use std::collections::HashMap;
use clap::{Arg, Command};
use reqwest;
use serde_json::Value;
use std::error::Error;
use std::io::{self, Write};
use std::process::Command as Cmd;
use chrono::{DateTime, Utc, TimeZone, Datelike, Duration, Timelike};
use colored::*;

const GITHUB_API: &str = "https://api.github.com/repos";

fn get_git_remote() -> Option<String> {
    let output = Cmd::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .ok()?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Some(url)
    } else {
        None
    }
}

fn parse_github_repo(url: &str) -> Option<String> {
    if let Some(pos) = url.find("github.com") {
        let repo = &url[pos + 11..];
        let repo = repo.trim_end_matches(".git");
        Some(repo.to_string())
    } else {
        None
    }
}

async fn fetch_contributors(repo: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = format!("{}/{}/contributors", GITHUB_API, repo);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", "Rust-GitHub-CLI")
        .send()
        .await?;

    if response.status().is_success() {
        let contributors: Vec<Value> = response.json().await?;
        println!("贡献者列表：");
        for contributor in &contributors {
            let name = contributor["login"].as_str().unwrap_or("Unknown");
            let contributions = contributor["contributions"].as_i64().unwrap_or(0);
            println!("{}: {} commits", name, contributions);
        }
        Ok(contributors)
    } else {
        println!("无法获取数据，状态码: {}", response.status());
        Err("Failed to fetch contributors".into())
    }
}

async fn fetch_contributor_activity(repo: &str, username: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut all_commits = Vec::new();
    let current_year = Utc::now().year();

    for year in (2000..=current_year).rev() {
        let start_date = format!("{}-01-01T00:00:00Z", year);
        let end_date = if year == current_year {
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
        } else {
            format!("{}-12-31T23:59:59Z", year)
        };

        let url = format!("{}/{}/commits", GITHUB_API, repo);
        let response = client
            .get(&url)
            .header("User-Agent", "Rust-GitHub-CLI")
            .query(&[
                ("author", username),
                ("per_page", "100"),
                ("since", &start_date),
                ("until", &end_date),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            let year_commits: Vec<Value> = response.json().await?;
            if !year_commits.is_empty() {
                all_commits.extend(year_commits);
            }
        } else if response.status() != reqwest::StatusCode::NOT_FOUND {
            println!("无法获取{}年的用户活动数据，状态码: {}", year, response.status());
            return Err("Failed to fetch contributor activity".into());
        }
    }

    if !all_commits.is_empty() {
        println!("\n{}的贡献热力图：", username);
        display_heatmap(&all_commits)?;
        Ok(())
    } else {
        println!("未找到该用户的贡献记录");
        Ok(())
    }
}

fn display_heatmap(commits: &[Value]) -> Result<(), Box<dyn Error>> {
    let mut commits_by_year: HashMap<i32, HashMap<String, i32>> = HashMap::new();

    for commit in commits {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                let year = date.year();
                let date_key = date.format("%Y-%m-%d").to_string();
                commits_by_year
                    .entry(year)
                    .or_insert_with(HashMap::new)
                    .entry(date_key)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let mut years: Vec<i32> = commits_by_year.keys().cloned().collect();
    years.sort_unstable_by(|a, b| b.cmp(a));

    println!("Current Date and Time (UTC): {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
    println!("Current User's Login: eR3R3\n");

    for &year in &years {
        println!("\nContributions for {}:", year);

        let mut contribution_matrix = vec![vec![0i32; 53]; 7];
        let year_start = Utc::now()
            .with_year(year)
            .unwrap()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();

        let year_end = if year == Utc::now().year() {
            Utc::now()
        } else {
            year_start + Duration::days(364)
        };

        let mut max_week: usize = 0;

        let mut current_date = year_start;
        while current_date <= year_end {
            let week_number = current_date.iso_week().week() as usize - 1;
            let weekday = current_date.weekday().num_days_from_monday() as usize;
            let date_str = current_date.format("%Y-%m-%d").to_string();

            if week_number < 53 {
                contribution_matrix[weekday][week_number] =
                    commits_by_year.get(&year)
                        .and_then(|year_commits| year_commits.get(&date_str))
                        .copied()
                        .unwrap_or(0);
                max_week = max_week.max(week_number);
            }

            current_date = current_date + Duration::days(1);
        }

        // Print the matrix for this year
        for day in 0..7 {
            print!("    ");
            for week in 0..=max_week {
                let count = contribution_matrix[day][week];
                let block = "■ ";
                let colored_block = match count {
                    0 => block.truecolor(250, 250, 210),
                    1 => block.truecolor(152, 251, 152),
                    2..=3 => block.truecolor(127, 255, 0),
                    4..=6 => block.truecolor(0, 255, 0),
                    7..=9 => block.truecolor(50, 205, 50),
                    _ => block.truecolor(34, 139, 34),
                };
                print!("{}", colored_block);
            }
            println!();
        }
    }

    println!("\nContribution Legend:");
    print!("{} No contributions    ", "■ ".truecolor(250, 250, 210));
    print!("{} 1 contribution    ", "■ ".truecolor(152, 251, 152));
    print!("{} 2-3 contributions    ", "■ ".truecolor(127, 255, 0));
    print!("{} 4-6 contributions    ", "■ ".truecolor(0, 255, 0));
    print!("{} 7-9 contributions    ", "■ ".truecolor(50, 205, 50));
    println!("{} 10+ contributions", "■ ".truecolor(34, 139, 34));

    Ok(())
}

async fn prompt_for_contributor(contributors: &[Value]) -> Option<String> {
    println!("\n是否要查看特定贡献者的贡献热力图？(y/n)");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok()?;

    if input.trim().to_lowercase() == "y" {
        println!("\n请输入贡献者用户名：");
        let mut username = String::new();
        io::stdin().read_line(&mut username).ok()?;
        Some(username.trim().to_string())
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("cchecker")
        .version("1.0")
        .about("查看 GitHub 项目贡献者")
        .arg(
            Arg::new("repo")
                .help("GitHub 仓库名，例如 'rust-lang/rust'")
                .required(false),
        )
        .arg(
            Arg::new("contributor")
                .help("特定贡献者的用户名")
                .required(false),
        )
        .get_matches();

    let repo = if let Some(repo) = matches.get_one::<String>("repo") {
        repo.clone()
    } else if let Some(url) = get_git_remote() {
        match parse_github_repo(&url) {
            Some(repo) => repo,
            None => {
                eprintln!("无法解析 GitHub 仓库名，请手动提供");
                return;
            }
        }
    } else {
        eprintln!("无法获取远程仓库，请在 Git 仓库内运行");
        return;
    };

    println!("查询 GitHub 项目: {}", repo);

    match fetch_contributors(&repo).await {
        Ok(contributors) => {
            let contributor = if let Some(contributor) = matches.get_one::<String>("contributor") {
                Some(contributor.clone())
            } else {
                prompt_for_contributor(&contributors).await
            };

            if let Some(username) = contributor {
                if let Err(e) = fetch_contributor_activity(&repo, &username).await {
                    eprintln!("获取贡献者活动失败: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("请求失败: {}", e);
        }
    }
}