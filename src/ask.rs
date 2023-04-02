use std::error::Error;
use reqwest;
use serde::{Serialize, Deserialize};

use crate::credential::Credential;

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionMessage {
  role: String,
  content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionsPostParams {
  model: String,
  messages: Vec<ChatCompletionMessage>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ChatCompletionsResponseUsage {
  prompt_tokens: u32,
  completion_tokens: u32,
  total_tokens: u32
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ChatCompletionsResponseChoice {
  message: ChatCompletionMessage,
  finish_reason: String,
  index: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ChatCompletionsResponseParams {
  id: String,
  object: String,
  created: u32,
  model: String,
  usage: ChatCompletionsResponseUsage,
  choices: Vec<ChatCompletionsResponseChoice>,
}

pub struct Answer {
  pub role: String,
  pub content: String,
}
pub async fn ask (content: &str, credential: Credential) -> Result<Answer, Box<dyn Error>> {
  let client = reqwest::Client::new();

  let post_message = ChatCompletionMessage {
    role: "user".to_string(),
    content: content.to_string(),
  };
  let mut messages = Vec::new();
  messages.push(post_message);
  let post_params = ChatCompletionsPostParams {
    model: credential.openai_chatgpt_model,
    messages: messages,
  };

  let post_body = serde_json::to_string(&post_params).unwrap();

  let response_body = client.post("https://api.openai.com/v1/chat/completions")
    // .header("OpenAI-Organization", credential.openai_organization_id)
    .header("Content-Type", "application/json")
    .header("Authorization",  format!("Bearer {}", credential.openai_secret_key))
    .body(post_body)
    .send()
    .await?
    .text()
    .await?
  ;

  let chat_completion: ChatCompletionsResponseParams = serde_json::from_str(response_body.as_str()).unwrap();
  let role = chat_completion.choices[0].message.role.clone();
  let content = chat_completion.choices[0].message.content.clone();

  Ok(Answer { role, content })
}
