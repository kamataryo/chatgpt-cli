use std::env;
use std::fs;
use std::path::Path;
use std::process;
use clap::Parser;
use yaml_rust::YamlLoader;

#[derive(Parser, Debug)]
struct Args {
  initial_query_text: String,
}

struct Credential {
  openai_organization_id: String,
  openai_secret_key: String,
}

fn main() {

  let credential = parse_credentials();

  let args = Args::parse();
  let initial_query_text = args.initial_query_text;
  println!("{}", initial_query_text);


}

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

// TODO:
// - login コマンド (org と API キーを保存)
// - モデルは GPT-4 固定
// -
