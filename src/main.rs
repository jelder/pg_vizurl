mod get_query_plan;
use chrono::prelude::*;
use clap::Parser;
use get_query_plan::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json as sjson;
use std::error::Error;
use std::io::Read;
use std::process::Command;
use std::result::Result as StdResult;
use std::{env, io};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// CLI frontend for explain.dalibo.com, a PostgreSQL execution plan visualizer
///
/// Usage: pbpaste | pg_vizurl
/// Not affiliated with Dalibo!
struct Args {
    #[clap(short, long)]
    /// Title of query, defaults to $USER and timestamp
    title: Option<String>,

    #[clap(short, long, env = "DATABASE_URL")]
    /// Defaults to $DATABASE_URL
    database_url: String,

    #[clap(short, long, default_value_t = String::from("https://explain.dalibo.com/new.json"))]
    url: String,
}

fn main() -> StdResult<(), Box<dyn Error + 'static>> {
    let args = Args::parse();

    let mut query = String::new();
    io::stdin().read_to_string(&mut query).unwrap();

    let title = match args.title {
        Some(str) => str,
        None => current_time_and_user(),
    };

    let params = [
        ("plan", get_query_plan(&query, &args.database_url)?),
        ("query", query),
        ("title", title),
    ];

    let client = Client::new();
    let response = client.post(args.url).form(&params).send()?;

    if response.status().is_success() {
        let body = response.text()?;
        let parsed_response: DaliboResult = sjson::from_str(&body)?;
        parsed_response.open();
    } else {
        println!("Error: {}", response.status());
    }

    Ok(())
}

fn current_time_and_user() -> String {
    let now = Local::now();
    let username = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    format!("{username} {}", now.format("%Y-%m-%d %H:%M:%S"))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct DaliboResult {
    id: String,
    // TODO figure out how to use it and expose it
    delete_key: String,
}

impl DaliboResult {
    fn view_url(&self) -> String {
        format!("https://explain.dalibo.com/plan/{}", self.id)
    }

    #[allow(dead_code)]
    fn delete_url(&self) -> String {
        format!("https://explain.dalibo.com/plan/{}", self.delete_key)
    }

    fn open(&self) {
        match Command::new("open").arg(&self.view_url()).spawn() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("{}", serde_json::to_string_pretty(&self).unwrap());
            }
        }
    }
}
