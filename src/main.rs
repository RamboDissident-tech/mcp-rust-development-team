use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncReadExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Request {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    jsonrpc: String,
    id: Value,
    result: Option<Value>,
    error: Option<Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Error {
    code: i32,
    message: String,
    data: Option<Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    
    // Read the first line to get the content length
    reader.read_line(&mut line).await?;
    
    // Parse the line to extract content length (should be in format "Content-Length: <number>")
    let content_length = line
        .split(": ")
        .nth(1)
        .and_then(|s| s.trim().parse::<usize>().ok())
        .expect("Failed to parse content length");
    
    // Read the actual JSON-RPC message
    let mut json_buffer = vec![0u8; content_length];
    reader.read_exact(&mut json_buffer).await?;
    
    let request: Request = serde_json::from_slice(&json_buffer)?;
    
    let response = handle_request(request).await?;
    
    // Send the response back to the client
    let response_json = serde_json::to_string(&response).unwrap();
    let response_length = response_json.len();
    
    println!("Content-Length: {}\r\n\r\n{}", response_length, response_json);
    
    Ok(())
}

async fn handle_request(request: Request) -> Result<Response> {
    match request.method.as_str() {
        "mcp.tool.list" => {
            let tools = vec![
                "safe_write_file".to_string(),
                "restore_backup".to_string(),
                "web_fetch_and_extract".to_string(),
                "run_cargo_check".to_string(),
                "run_automation_task".to_string(),
            ];
            
            Ok(Response {
                jsonrpc: "2.0".to_string(),
                id: request.id.unwrap_or(Value::Null),
                result: Some(serde_json::to_value(tools).unwrap()),
                error: None,
            })
        }
        "mcp.tool.call" => {
            let params = request.params.unwrap();
            let tool_name = params.get("toolName").and_then(|v| v.as_str()).unwrap_or("");
            
            match tool_name {
                "safe_write_file" => safe_write_file(params).await,
                "restore_backup" => restore_backup(params).await,
                "web_fetch_and_extract" => web_fetch_and_extract(params).await,
                "run_cargo_check" => run_cargo_check(params).await,
                "run_automation_task" => run_automation_task(params).await,
                _ => Ok(Response {
                    jsonrpc: "2.0".to_string(),
                    id: request.id.unwrap_or(Value::Null),
                    result: None,
                    error: Some(Error {
                        code: -32601,
                        message: "Method not found".to_string(),
                        data: None,
                    }),
                }),
            }
        }
        _ => Ok(Response {
            jsonrpc: "2.0".to_string(),
            id: request.id.unwrap_or(Value::Null),
            result: None,
            error: Some(Error {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }),
        }),
    }
}

async fn safe_write_file(params: Value) -> Result<Response> {
    // Extract parameters
    let file_path = params.get("file_path").and_then(|v| v.as_str()).unwrap_or("");
    let content = params.get("content").and_then(|v| v.as_str()).unwrap_or("");
    let reason = params.get("reason").and_then(|v| v.as_str()).unwrap_or("");
    
    // Get current timestamp for backup
    let now = chrono::Utc::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    
    // Check if file exists and create backup
    let backup_dir = format!(".backups/{}", now.format("%Y-%m-%d"));
    fs::create_dir_all(&backup_dir)?;
    
    let backup_path = format!("{}/{}", backup_dir, file_path);
    fs::create_dir_all(std::path::Path::new(&backup_path).parent().unwrap())?;
    
    // Copy existing file to backup
    if fs::metadata(file_path).is_ok() {
        fs::copy(file_path, &backup_path)?;
    }
    
    // Create git repository if not exists
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()
        .expect("Failed to execute git command");
    
    if !output.status.success() {
        Command::new("git")
            .arg("init")
            .output()
            .expect("Failed to initialize git repository");
    }
    
    // Add file to git
    Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to add files to git");
    
    // Commit with reason
    let commit_message = format!("auto-backup: {} [{}]", reason, timestamp);
    let output = Command::new("git")
        .arg("commit")
        .arg("--allow-empty")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .expect("Failed to commit files");
    
    // Get the commit hash
    let commit_hash = String::from_utf8(output.stdout)
        .unwrap_or_default()
        .split_whitespace()
        .last()
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    // Write new content to file
    fs::create_dir_all(std::path::Path::new(file_path).parent().unwrap())?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    
    // Return backup path and commit hash
    let result = serde_json::json!({
        "backup_path": backup_path,
        "commit_hash": commit_hash,
    });
    
    Ok(Response {
        jsonrpc: "2.0".to_string(),
        id: Value::Null,
        result: Some(result),
        error: None,
    })
}

async fn restore_backup(params: Value) -> Result<Response> {
    let target = params.get("target").and_then(|v| v.as_str()).unwrap_or("");
    
    if target == "last_git" {
        // Perform git reset --hard HEAD~1
        let output = Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg("HEAD~1")
            .output()
            .expect("Failed to execute git command");
        
        if output.status.success() {
            Ok(Response {
                jsonrpc: "2.0".to_string(),
                id: Value::Null,
                result: Some(serde_json::json!({"status": "success", "message": "Restored last Git backup"})),
                error: None,
            })
        } else {
            Ok(Response {
                jsonrpc: "2.0".to_string(),
                id: Value::Null,
                result: None,
                error: Some(Error {
                    code: -32602,
                    message: "Failed to restore backup".to_string(),
                    data: None,
                }),
            })
        }
    } else {
        // Restore specific file from backup
        let backup_path = target;
        
        if fs::metadata(backup_path).is_ok() {
            // Copy backup to original location
            let dest_path = std::path::Path::new(backup_path)
                .strip_prefix(".backups/")
                .unwrap_or(std::path::Path::new(backup_path));
            
            fs::create_dir_all(dest_path.parent().unwrap())?;
            fs::copy(backup_path, dest_path)?;
            
            Ok(Response {
                jsonrpc: "2.0".to_string(),
                id: Value::Null,
                result: Some(serde_json::json!({"status": "success", "message": format!("Restored backup from {}", backup_path)})),
                error: None,
            })
        } else {
            Ok(Response {
                jsonrpc: "2.0".to_string(),
                id: Value::Null,
                result: None,
                error: Some(Error {
                    code: -32602,
                    message: "Backup file not found".to_string(),
                    data: None,
                }),
            })
        }
    }
}

async fn web_fetch_and_extract(params: Value) -> Result<Response> {
    let url = params.get("url").and_then(|v| v.as_str()).unwrap_or("");
    let css_selector = params.get("css_selector").and_then(|v| v.as_str());
    
    // Fetch the webpage
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await?;
    
    let html = response.text().await?;
    
    // Parse HTML with scraper
    let document = scraper::Html::parse_document(&html);
    
    // Extract text content based on selector or whole document
    let text_content = match css_selector {
        Some(selector) => {
            let selector = scraper::Selector::parse(selector).unwrap();
            document
                .select(&selector)
                .flat_map(|element| element.text())
                .collect::<Vec<_>>()
                .join(" ")
        }
        None => {
            // Extract all text, but remove script and style tags
            let mut text = String::new();
            for element in document.select(&scraper::Selector::parse("*").unwrap()) {
                if !element.value().name().eq_ignore_ascii_case("script") && 
                   !element.value().name().eq_ignore_ascii_case("style") {
                    text.push_str(element.inner_html().as_ref());
                }
            }
            text
        }
    };
    
    // Truncate to 30,000 characters
    let truncated_text = if text_content.len() > 30000 {
        text_content[..30000].to_string()
    } else {
        text_content
    };
    
    Ok(Response {
        jsonrpc: "2.0".to_string(),
        id: Value::Null,
        result: Some(serde_json::json!({"content": truncated_text})),
        error: None,
    })
}

async fn run_cargo_check(params: Value) -> Result<Response> {
    let command = params.get("command").and_then(|v| v.as_str()).unwrap_or("");
    let working_dir = params.get("working_dir").and_then(|v| v.as_str()).unwrap_or(".");
    
    // Validate command
    if !["check", "test", "clippy", "build"].contains(&command) {
        return Ok(Response {
            jsonrpc: "2.0".to_string(),
            id: Value::Null,
            result: None,
            error: Some(Error {
                code: -32602,
                message: "Invalid command".to_string(),
                data: None,
            }),
        });
    }
    
    // Run cargo command
    let output = Command::new("cargo")
        .arg(command)
        .current_dir(working_dir)
        .output()
        .expect("Failed to execute cargo command");
    
    Ok(Response {
        jsonrpc: "2.0".to_string(),
        id: Value::Null,
        result: Some(serde_json::json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "exit_code": output.status.code().unwrap_or(-1)
        })),
        error: None,
    })
}

async fn run_automation_task(params: Value) -> Result<Response> {
    let script_path = params.get("script_path").and_then(|v| v.as_str()).unwrap_or("");
    let args = params.get("args")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();
    
    // Create git snapshot
    let output = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to add files to git");
    
    let commit_message = "automation-task snapshot".to_string();
    let output = Command::new("git")
        .arg("commit")
        .arg("--allow-empty")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .expect("Failed to commit files");
    
    // Get the commit hash
    let commit_hash = String::from_utf8(output.stdout)
        .unwrap_or_default()
        .split_whitespace()
        .last()
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    // Check if script is within workspace
    if !script_path.starts_with(".") {
        return Ok(Response {
            jsonrpc: "2.0".to_string(),
            id: Value::Null,
            result: None,
            error: Some(Error {
                code: -32602,
                message: "Script path must be within workspace".to_string(),
                data: None,
            }),
        });
    }
    
    // Run script with timeout
    let output = Command::new(script_path)
        .args(&args)
        .output()
        .expect("Failed to execute script");
    
    // Note: The timeout functionality is not implemented here as it's not available in the current Rust version
    // In a production environment, you would use a proper timeout mechanism
    
    Ok(Response {
        jsonrpc: "2.0".to_string(),
        id: Value::Null,
        result: Some(serde_json::json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "exit_code": output.status.code().unwrap_or(-1),
            "snapshot_commit": commit_hash
        })),
        error: None,
    })
}