use std::error::Error;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use clap::Parser;
use yaml_rust::YamlLoader;
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
struct Args {
  initial_query_text: String,
}

struct Credential {
  openai_organization_id: String,
  openai_secret_key: String,
}

#[tokio::main]
async fn main() {

  let credential = parse_credentials();

  let args = Args::parse();
  let initial_query_text = args.initial_query_text.as_str();

  let result = ask(initial_query_text, credential).await;

  match result {
    Ok(answer) => println!("{}", answer),
    Err(err) => {},
  }

}

/** Parse OpenAI credential from environment or .chatgpt-cli.yaml */
fn parse_credentials() -> Credential {
    // 環境変数を確認
    let mut openai_organization_id = match env::var("OPENAI_ORGANIZATION_ID") {
    Ok(val) => val,
    Err(_err) => "".to_string(),
      };
    let mut openai_secret_key = match env::var("OPENAI_SECRET_KEY") {
    Ok(val) => val,
    Err(_err) => "".to_string(),
      };

    // .chatgpt-cli.yaml を再帰的探す
    if openai_organization_id == "" || openai_secret_key == "" {
    let mut config_dir = fs::canonicalize(".").unwrap();
    let config_filename = ".chatgpt-cli.yaml";

    let config_path = loop {
      let mut current_config_path = config_dir.clone();
      current_config_path.push(config_filename);
      if !current_config_path.exists() {
        if config_dir == Path::new("/") {
          eprintln!("Application error. Credential can not be loaded.");
          process::exit(1);
        } else {
          config_dir.pop();
        }
      } else {
        // Found
        break current_config_path;
      }
    };

    // yaml のパース
    let content = fs::read_to_string(&config_path).unwrap();
    let yaml_docs = YamlLoader::load_from_str(&content);

    match yaml_docs {
      Ok(content) => {
        if content.len() == 0 {
          eprintln!("Application error: Invalid yaml format for {:?}.", config_path.to_str());
          process::exit(2);
        } else {
          let yaml_doc = &content[0];
          let current_org_id = &yaml_doc["openai_organization_id"].as_str();
          let current_secret_key = &yaml_doc["openai_secret_key"].as_str();

          if let (Some(_organization_id), Some(_secret_key)) = (current_org_id, current_secret_key) {
            if _organization_id.is_empty() && _secret_key.is_empty() {
                eprintln!("Application error: Invalid yaml format for {:?}.", config_path.to_str().unwrap());
                process::exit(3)
            } else {
              if openai_organization_id == "" {
                openai_organization_id = _organization_id.to_string();
              }
              if openai_secret_key == "" {
                openai_secret_key = _secret_key.to_string();
              }
            };
          } else {
            eprintln!("Application error: Missing openai_organization_id or openai_secret_key in the yaml file for {:?}.", config_path.to_str().unwrap());
            process::exit(4);
          };
        }
      }
      Err(_err) => {
        eprintln!("{:?}", _err);
        process::exit(5);
      }
    };
  }
  return Credential {
    openai_organization_id: openai_organization_id,
    openai_secret_key: openai_secret_key,
  };
}

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
struct ChatCompletionsResponseUsage {
  prompt_tokens: i8,
  completion_tokens: i8,
  total_tokens: i8
}

#[derive(Debug, Deserialize)]
struct ChatCompletionsResponseChoice {
  message: ChatCompletionMessage,
  finish_reason: String,
  index: i8,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionsResponseParams {
  id: String,
  object: String,
  created: i32,
  model: String,
  usage: ChatCompletionsResponseUsage,
  choices: Vec<ChatCompletionsResponseChoice>,
}

async fn ask (content: &str, credential: Credential) -> Result<String, Box<dyn Error>> {
  let client = reqwest::Client::new();

  let post_message = ChatCompletionMessage {
    role: "user".to_string(),
    content: content.to_string(),
  };
  let mut messages = Vec::new();
  messages.push(post_message);
  let post_params = ChatCompletionsPostParams {
    model: "gpt-3.5-turbo".to_string(),
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
  let response_params: ChatCompletionsResponseParams = serde_json::from_str(response_body.as_str()).unwrap();
  let answer = &response_params.choices[0].message.content;

  Ok(answer.to_string())
}



