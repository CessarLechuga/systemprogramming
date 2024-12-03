use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    worker_threads: usize,
    timeout_seconds: u64,
    max_retries: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Channel error: {0}")]
    ChannelError(String),
}

pub struct Monitor {
    config: Config,
    client: Arc<Client>,
}

impl Monitor {
    pub fn new(config: Config) -> Result<Self, MonitorError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()?;

        Ok(Monitor {
            config,
            client: Arc::new(client),
        })
    }

    pub fn check_websites(&self, urls: Vec<String>) -> Result<Vec<WebsiteStatus>, MonitorError> {
        let pool = thread::scope(|s| {
            let mut handles = Vec::new();
            
            for url in urls {
                let client = Arc::clone(&self.client);
                let max_retries = self.config.max_retries;
                
                let handle = s.spawn(move || {
                    Self::check_single_website(&client, &url, max_retries)
                });
                handles.push(handle);
            }

            handles.into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        });

        Ok(pool)
    }

    fn check_single_website(client: &Client, url: &str, max_retries: u32) -> WebsiteStatus {
        let start = Instant::now();
        let mut retries = 0;
        let mut last_error = None;

        while retries < max_retries {
            match client.get(url).send() {
                Ok(response) => {
                    return WebsiteStatus {
                        url: url.to_string(),
                        status: Ok(response.status().as_u16()),
                        response_time: start.elapsed(),
                        timestamp: Utc::now(),
                    };
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    retries += 1;
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }

        WebsiteStatus {
            url: url.to_string(),
            status: Err(last_error.unwrap_or_else(|| "Unknown error".to_string())),
            response_time: start.elapsed(),
            timestamp: Utc::now(),
        }
    }
}

fn main() -> Result<(), MonitorError> {
    let config = Config {
        worker_threads: 10,
        timeout_seconds: 5,
        max_retries: 3,
    };

    let monitor = Monitor::new(config)?;
    let urls = vec![
        "https://www.google.com".to_string(),
        "https://www.github.com".to_string(),
    ];

    let results = monitor.check_websites(urls)?;
    for result in results {
        println!("{:#?}", result);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_successful_request() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .create();

        let config = Config {
            worker_threads: 4,
            timeout_seconds: 5,
            max_retries: 3,
        };

        let monitor = Monitor::new(config).unwrap();
        let results = monitor.check_websites(vec![server.url()]).unwrap();

        assert_eq!(results.len(), 1);
        assert!(matches!(results[0].status, Ok(200)));
        mock.assert();
    }

    #[test]
    fn test_timeout_handling() {
        // Use a non-existent server that should timeout
        let config = Config {
            worker_threads: 4,
            timeout_seconds: 1,
            max_retries: 1,
        };

        let monitor = Monitor::new(config).unwrap();
        // Use a non-routable IP address that should timeout quickly
        let results = monitor.check_websites(vec!["http://198.51.100.1:12345".to_string()]).unwrap();

        assert_eq!(results.len(), 1);
        assert!(matches!(results[0].status, Err(_)));
    }

    #[test]
    fn test_invalid_url() {
        let config = Config {
            worker_threads: 4,
            timeout_seconds: 1,
            max_retries: 1,
        };

        let monitor = Monitor::new(config).unwrap();
        let results = monitor.check_websites(vec!["not_a_valid_url".to_string()]).unwrap();

        assert_eq!(results.len(), 1);
        assert!(matches!(results[0].status, Err(_)));
    }
}