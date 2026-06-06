// LM Studio streaming client for Zelfi
// Calls http://localhost:1234/v1/chat/completions with stream: true
// Emits "zelfi://token" events to the frontend as each token arrives
// Emits "zelfi://done" event when streaming completes
// Emits "zelfi://error" event on failure

use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Window};

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct DeltaContent {
    content: Option<String>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: DeltaContent,
}

#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Serialize, Clone)]
struct TokenPayload {
    token: String,
}

#[derive(Serialize, Clone)]
struct ErrorPayload {
    message: String,
}

pub async fn stream_completion(window: Window, prompt: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let request_body = ChatRequest {
        model: "gemma-4-e4b-it".to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        stream: true,
        max_tokens: 2048,
    };

    let response = client
        .post("http://localhost:1234/v1/chat/completions")
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            let _ = window.emit("zelfi://error", ErrorPayload {
                message: format!("Cannot connect to LM Studio: {}", e),
            });
            e
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        let _ = window.emit("zelfi://error", ErrorPayload {
            message: format!("LM Studio returned {}: {}", status, body),
        });
        return Ok(());
    }

    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(e) => {
                let _ = window.emit("zelfi://error", ErrorPayload {
                    message: format!("Stream error: {}", e),
                });
                break;
            }
        };

        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            if line.starts_with("data: ") {
                let data = &line["data: ".len()..];

                if data.trim() == "[DONE]" {
                    let _ = window.emit("zelfi://done", ());
                    return Ok(());
                }

                if let Ok(parsed) = serde_json::from_str::<StreamChunk>(data) {
                    if let Some(choice) = parsed.choices.first() {
                        if let Some(ref content) = choice.delta.content {
                            if !content.is_empty() {
                                let _ = window.emit("zelfi://token", TokenPayload {
                                    token: content.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let _ = window.emit("zelfi://done", ());
    Ok(())
}
