use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn get_query_plan(query: &str, database_url: &str) -> Result<String, Box<dyn Error>> {
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

    Ok(String::from_utf8(output)?)
}
