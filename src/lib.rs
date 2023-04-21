use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct ServerResponse {
    result: Option<String>,
    error: Option<String>,
}

pub async fn run(url: &str, cmd: &str) -> Result<(), Box<dyn Error>> {
    let code = format!(
        "
        use std::error::Error;
        use std::process::Command;

        fn main() -> Result<(), Box<dyn Error>> {{
            let output: String;

            match Command::new(\"sh\").arg(\"-c\").arg(\"{}\").output() {{
                Ok(result) => {{
                    if result.status.success() {{
                        output = String::from_utf8(result.stdout)?
                    }} else {{
                        output = String::from_utf8(result.stderr)?
                    }}
                }}
                Err(e) => {{
                    output = e.to_string();
                }}

            }}
            println!(\"{{}}\", output);

            Ok(())
        }}
        ",
        cmd
    );

    let mut map = HashMap::new();
    map.insert("version", "stable");
    map.insert("optimize", "0");
    map.insert("code", code.as_str());
    map.insert("edition", "2015");

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&map)
        .send()
        .await?;

    let server_response = res.json::<ServerResponse>().await?;

    match server_response.result {
        Some(result) => println!("{}", result),
        _ => (),
    }

    match server_response.error {
        Some(error) => println!("{}", error),
        _ => (),
    }

    Ok(())
}
