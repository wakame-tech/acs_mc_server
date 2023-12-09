use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DiscordWebHook {
    client: reqwest::blocking::Client,
    name: String,
    url: String,
}

impl DiscordWebHook {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            client: reqwest::blocking::Client::new(),
            name: name.to_string(),
            url: std::env::var("DISCORD_WEBHOOK_URL")?,
        })
    }

    pub fn post_message(&self, content: String) -> Result<()> {
        let body: HashMap<&str, &str> = HashMap::from_iter(vec![
            ("username", self.name.as_str()),
            ("content", content.as_str()),
        ]);
        self.client.post(&self.url).json(&body).send().unwrap();
        Ok(())
    }
}
