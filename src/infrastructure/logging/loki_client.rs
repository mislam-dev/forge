use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::modules::projects::build_worker::log_stream::LogItem;

#[derive(Debug)]
pub enum LokiError {
    FailedToPushLogs(String),
    FailedToQueryLogs(String),
    FailedToParseLogs(String),
    FailedToParseLogLine(String),
    FailedToParseLogTimestamp(String),
}
#[derive(Deserialize)]
struct QueryResponse {
    data: QueryData,
}

#[derive(Deserialize)]
struct QueryData {
    result: Vec<StreamResult>,
}

#[derive(Deserialize)]
struct StreamResult {
    #[allow(dead_code)]
    stream: HashMap<String, String>,
    values: Vec<(String, String)>, // [timestamp_ns, line]
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Info,
    Error,
    Warn,
}

pub struct LokiFetchQuery {
    pub deployment_id: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Debug)]
pub struct LokiClient {
    http: reqwest::Client,
    base_url: String,
}

impl LokiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn push(&self, items: &[LogItem]) -> Result<(), LokiError> {
        // TODO: implement push
        let mut streams: HashMap<(String, String), Vec<[String; 2]>> = HashMap::new();

        for item in items {
            let ts_ns = DateTime::parse_from_rfc3339(&item.timestamp)
                .unwrap_or_default()
                .timestamp_nanos_opt()
                .unwrap_or_default()
                .to_string();

            let line = serde_json::to_string(item).unwrap_or_default();
            let level = serde_json::json!(&&item.level)
                .as_str()
                .unwrap_or("info")
                .to_string();
            streams
                .entry(("forge".to_string(), level))
                .or_default()
                .push([ts_ns, line]);
        }
        let payload = json!({
          "streams": streams.into_iter().map(|((app, level), values)| {
            json!({
              "stream": {
                "app": app,
                "level": level,
              },
              "values": values
            })
          }).collect::<Vec<_>>()
        });

        let url = format!("{}/push", &self.base_url);

        let _a = self
            .http
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| LokiError::FailedToPushLogs(e.to_string()))?
            .error_for_status()
            .map_err(|e| LokiError::FailedToPushLogs(e.to_string()))?;

        Ok(())
    }

    pub async fn find_by_deployment_id(
        &self,
        dto: LokiFetchQuery,
    ) -> Result<Vec<LogItem>, LokiError> {
        //  validate deployment id
        let url = format!("{}/loki/api/v1/query_range", self.base_url);
        println!("url {:#?}", url);

        let query = format!(
            r#"{{app="forge"}} |= "{id}" | json | deployment_id="{id}""#,
            id = dto.deployment_id
        );

        const PAGE: usize = 5000;
        const WINDOW_NS: i64 = 720 * 3_600 * 1_000_000_000; // 30 days, under Loki's 721h default
        let from_ns = dto.from.timestamp_nanos_opt().unwrap_or_default();
        let to_ns = dto.to.timestamp_nanos_opt().unwrap_or_default();

        let mut items = Vec::new();
        let mut win_start = from_ns;

        while win_start < to_ns {
            let win_end = (win_start + WINDOW_NS).min(to_ns);
            let mut start = win_start;

            loop {
                let resp: QueryResponse = self
                    .http
                    .get(&url)
                    .query(&[
                        ("query", query.as_str()),
                        ("start", &start.to_string()),
                        ("end", &win_end.to_string()),
                        ("limit", &PAGE.to_string()),
                        ("direction", "forward"),
                    ])
                    .send()
                    .await
                    .map_err(|e| {
                        println!("{e:#?}");
                        LokiError::FailedToQueryLogs(e.to_string())
                    })?
                    .error_for_status()
                    .map_err(|e| LokiError::FailedToQueryLogs(e.to_string()))?
                    .json()
                    .await
                    .map_err(|e| LokiError::FailedToQueryLogs(e.to_string()))?;

                let mut rows: Vec<(i32, String)> = resp
                    .data
                    .result
                    .into_iter()
                    .flat_map(|s| s.values)
                    .map(|(ts, line)| (ts.parse().unwrap_or(0), line))
                    .collect();
                rows.sort_by_key(|(ts, _)| *ts);
                let fetched = rows.len();

                let Some(last_ts) = rows.last().map(|(ts, _)| *ts) else {
                    break;
                };
                items.extend(
                    rows.into_iter()
                        .filter_map(|(_, line)| serde_json::from_str::<LogItem>(&line).ok()),
                );

                if fetched < PAGE {
                    break;
                }
                start = (last_ts as i64) + 1;
            }
            win_start = win_end;
        }
        Ok(items)
    }
}
