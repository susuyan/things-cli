use std::io::{self, Read};

use crate::cli::args::BatchCommand;
use crate::cli::GlobalOpts;
use crate::core::executor::{Executor, OpenExecutor};
use crate::core::url_builder::{Command, ThingsUrl};

pub fn handle(cmd: BatchCommand, global: &GlobalOpts) -> anyhow::Result<()> {
    match cmd {
        BatchCommand::Import { file, reveal } => {
            handle_import(&file, reveal, global)?;
        }
        BatchCommand::Template => {
            handle_template()?;
        }
    }

    Ok(())
}

fn handle_import(file: &str, reveal: bool, global: &GlobalOpts) -> anyhow::Result<()> {
    // 读取 JSON 数据
    let json_data = if file == "-" {
        // 从 stdin 读取
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        // 从文件读取
        let path = std::path::Path::new(file);
        if !path.exists() {
            return Err(anyhow::anyhow!("File not found: {}", file));
        }
        std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file, e))?
    };

    // 验证 JSON 格式
    let _: serde_json::Value = serde_json::from_str(&json_data)
        .map_err(|e| anyhow::anyhow!("Invalid JSON format: {}", e))?;

    // 构建 URL
    let mut url = ThingsUrl::new(Command::Json)
        .param("data", &json_data);

    if reveal {
        url = url.param_bool("reveal", true);
    }

    let url = url.build();

    if global.debug {
        eprintln!("URL: {}", url);
    }

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if result.success {
        println!("✓ Batch import sent to Things successfully");
    }

    Ok(())
}

fn handle_template() -> anyhow::Result<()> {
    let template = r#"[
  {
    "type": "project",
    "attributes": {
      "title": "Go Shopping",
      "items": [
        {
          "type": "to-do",
          "attributes": {
            "title": "Buy milk"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Buy bread"
          }
        }
      ]
    }
  },
  {
    "type": "to-do",
    "attributes": {
      "title": "Pick up dry cleaning",
      "when": "evening",
      "tags": ["Errand"]
    }
  }
]"#;
    println!("{}", template);
    Ok(())
}
