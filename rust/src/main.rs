/*
This code translates the Python code to Rust using similar libraries and constructs. 
It uses the reqwest crate for making HTTP requests, serde_json for JSON serialization/deserialization, 
chrono for datetime operations, and url for URL manipulation. 
Note that async/await syntax is used for asynchronous HTTP requests.
*/

use chrono::{DateTime, TimeZone, Utc};
use reqwest::{Client, Error};
use serde_json::Value;
use url::Url;

const CMR_OPS: &str = "https://cmr.earthdata.nasa.gov/search/";
const CMR_UAT: &str = "https://cmr.uat.earthdata.nasa.gov/search/";
const CMR_SIT: &str = "https://cmr.sit.earthdata.nasa.gov/search/";

pub struct Query {
    base_url: String,
    route: String,
    format: String,
    valid_formats_regex: Vec<&'static str>,
    params: serde_json::Map<String, Value>,
    options: serde_json::Map<String, Value>,
    concept_id_chars: Vec<char>,
    headers: Option<reqwest::header::HeaderMap>,
}

impl Query {
    pub fn new(route: &str, mode: &str) -> Self {
        let base_url = match mode {
            "CMR_UAT" => String::from(CMR_UAT),
            "CMR_SIT" => String::from(CMR_SIT),
            _ => String::from(CMR_OPS),
        };

        let valid_formats_regex = vec![
            "json", "xml", "echo10", "iso", "iso19115", "csv", "atom", "kml", "native",
        ];

        Query {
            base_url,
            route: String::from(route),
            format: String::from("json"),
            valid_formats_regex,
            params: serde_json::Map::new(),
            options: serde_json::Map::new(),
            concept_id_chars: Vec::new(),
            headers: None,
        }
    }

    pub async fn get(&mut self, limit: usize) -> Result<Vec<Value>, Error> {
        let mut results = Vec::new();
        let mut page_size = limit.min(2000);
        let mut more_results = true;

        while more_results {
            page_size = limit.saturating_sub(results.len()).min(page_size);
            let url = self.build_url()?;

            let client = Client::new();
            let response = client
                .get(&url)
                .headers(self.headers.clone().unwrap_or_default())
                .query(&[("page_size", page_size.to_string())])
                .send()
                .await?;

            let headers = response.headers().clone();
            self.headers = Some(headers);

            let text = response.text().await?;
            let latest = if self.format == "json" {
                let json: Value = serde_json::from_str(&text)?;
                json["feed"]["entry"].as_array().unwrap_or_default().to_vec()
            } else {
                vec![serde_json::json!(text)]
            };

            results.extend(latest);

            if page_size > results.len() || results.len() >= limit {
                more_results = false;
            }
        }

        if let Some(headers) = &mut self.headers {
            headers.remove("cmr-search-after");
        }

        Ok(results)
    }

    fn build_url(&self) -> Result<String, url::ParseError> {
        let mut url = Url::parse(&self.base_url)?;
        url.path_segments_mut()
            .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
            .push(&self.route);

        for (key, value) in &self.params {
            url.query_pairs_mut().append_pair(key, &value.to_string());
        }

        Ok(url.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut query = Query::new("route", "CMR_OPS");
    let results = query.get(2000).await?;
    println!("{:?}", results);
    Ok(())
}