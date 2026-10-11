//! nagi：TODO アプリの TUI と CLI。データは、この端末だけに置くか（ローカル）、ログインして Worker（Cloudflare）と
//! 同期する（そのときは、手元に控えの SQLite を持つ）

mod auth;
mod cli;
mod config;
mod data;
mod dates;
mod log;
mod model;
mod rank;
mod ui;

use std::process::ExitCode;

use cli::Command;
use config::{Config, DEFAULT_SERVER};

/// 版の正本は Cargo.toml の `version`（Release のタグ `v<版>` と、ワークフロー（tui.yml）の check で突き合わせる）
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let invocation = match cli::parse(&args) {
        Ok(invocation) => invocation,
        Err(message) => {
            eprintln!("{message}\n\n使い方は `nagi --help` で見られます");
            return ExitCode::from(2);
        }
    };
    match invocation.command {
        Command::Help => {
            cli::emit(&format!("{}\n\n本番：{DEFAULT_SERVER}", cli::HELP));
            ExitCode::SUCCESS
        }
        Command::Version => {
            cli::emit(&format!("nagi {VERSION}"));
            ExitCode::SUCCESS
        }
        Command::Tui => {
            let config = Config::new(invocation.server);
            log::init(&config.log_path());
            match ui::run(config) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("起動できませんでした：{error}");
                    ExitCode::FAILURE
                }
            }
        }
        command => {
            let config = Config::new(invocation.server);
            log::init(&config.log_path());
            cli::run(&config, command)
        }
    }
}
