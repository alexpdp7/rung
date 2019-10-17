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
    if config.slack_url == "" {
        return Err(From::from("no slack url in config"));
    }
    let resp = ureq::post(&config.slack_url)
        .set("X-My-Header", "Secret")
        .send_json(json!({
            "text": message,
        }));
    if resp.status() == 200 {
        Ok(())
    } else {
        Err(From::from(resp.status_text()))
    }
}

fn main() {
    let config: RungConfig = confy::load("rung").unwrap();
    send_to_slack(config, "ping").unwrap();
}
