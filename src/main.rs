#[macro_use]
extern crate ureq;
#[macro_use]
extern crate serde_derive;

use std::error::Error;

#[derive(Serialize, Deserialize)]
struct RungConfig {
    slack_url: String,
}

impl ::std::default::Default for RungConfig {
    fn default() -> Self {
        Self {
            slack_url: "".to_string(),
        }
    }
}

fn send_to_slack(config: RungConfig, message: &str) -> Result<(), Box<dyn Error>> {
    if config.slack_url.is_empty() {
        return Err(From::from("no slack url in config"));
    }
    ureq::post(&config.slack_url).send_json(json!({
        "text": message,
    }))?;
    Ok(())
}

fn main() {
    let config: RungConfig = confy::load("rung").unwrap();
    send_to_slack(
        config,
        &::std::env::args()
            .skip(1)
            .collect::<Vec<String>>()
            .join(" "),
    )
    .unwrap();
}
