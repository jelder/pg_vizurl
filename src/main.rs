use chrono::prelude::*;
use regex::Regex;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json as sjson;
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::result::Result as StdResult;
use std::str::FromStr;
use std::{env, io};

fn main() -> StdResult<(), Box<dyn Error + 'static>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let mut query = String::new();
    io::stdin()
        .read_to_string(&mut query)
        .expect("Failed to read input");

    // Check if input matches regex
    let select_regex = Regex::new(r#"^select\s"#).unwrap();
    if !select_regex.is_match(&query) {
        eprintln!("Input must begin with SELECT'");
        std::process::exit(1);
    }

    let explain_query = format!(
        "EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) {}",
        query
    );

    let mut child = Command::new("psql")
        .arg("--no-psqlrc")
        .arg("--quiet")
        .arg("--no-align")
        .arg("--tuples-only")
        .arg(database_url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(explain_query.as_bytes())?;

    let output = child.wait_with_output()?.stdout;

    let params = [
        ("plan", String::from_utf8(output)?),
        ("query", query),
        ("title", get_current_time_and_user()),
    ];

    let client = Client::new();
    let response = client
        .post("https://explain.dalibo.com/new.json")
        .form(&params)
        .send()?;

    if response.status().is_success() {
        // Get the response body as text
        let body = response.text()?;
        let parsed_response: DaliboResult = sjson::from_str(&body)?;
        parsed_response.open();
    } else {
        println!("Error: {}", response.status());
    }

    Ok(())
}

fn get_current_time_and_user() -> String {
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
