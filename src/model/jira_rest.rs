use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthParams {
    pub(crate) email: String,
    #[serde(rename = "apiKey")]
    pub(crate) api_key: String,
    pub(crate) url: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Status {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct JiraError {
    message: String,
}

impl JiraError {
    fn new(msg: &str) -> Self {
        let message = String::from(msg);
        Self { message }
    }
}

impl Display for JiraError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jira Error: {}", self.message)
    }
}

impl Error for JiraError {}

pub async fn request_statuses(auth: &AuthParams, statuses: &Vec<String>) -> Result<HashMap<String, i32>, JiraError> {
    eprintln!("Requesting status IDs from Jira REST API...");
    let client = Client::new();
    let url = format!("{}/rest/api/3/status", auth.url);
    let response = client.get(&url)
        .basic_auth(&auth.email, Some(&auth.api_key))
        .send()
        .await
        .map_err(|e| JiraError::new(&format!("HTTP request failed: {e}")))?;
    if !response.status().is_success() {
        return Err(JiraError::new(&format!("Call to Jira rest api failed with status {}", response.status())));
    }

    let statuses_from_api = response.json::<Vec<Status>>()
        .await
        .map_err(|e| JiraError::new(&format!("Failed to deserialize response from Jira rest api: {e}")))?
        .iter()
        .map(|s| Status { id: s.id.clone(), name: s.name.to_uppercase() })
        .collect::<Vec<_>>();

    let mut status_map = HashMap::new();
    for status in statuses_from_api {
        if statuses.contains(&status.name.to_uppercase()) {
            let status_id = status.id.parse::<i32>().map_err(|_| JiraError::new(&format!(
                "Invalid status id: {}", &status.id
            )))?;
            status_map.insert(status.name.clone(), status_id);
        }
    }

    Ok(status_map)
}
