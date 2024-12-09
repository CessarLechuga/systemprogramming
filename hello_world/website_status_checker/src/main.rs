use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH, Instant},
};
use serde::{Deserialize, Serialize};
use ureq;  // Add this import

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
    timestamp: u64,
}

#[derive(Debug)]
pub enum MonitorError {
    RequestError(String),
    AgentError(String),
}

impl std::fmt::Display for MonitorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonitorError::RequestError(e) => write!(f, "Request failed: {}", e),
            MonitorError::AgentError(e) => write!(f, "Agent error: {}", e),
        }
    }
}

impl std::error::Error for MonitorError {}

pub struct Monitor {
    config: Config,
    agent: Arc<ureq::Agent>,
}

impl Monitor {
    pub fn new(config: Config) -> Result<Self, MonitorError> {
        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build();

        Ok(Monitor {
            config,
            agent: Arc::new(agent),
        })
    }

    pub fn check_websites(&self, urls: Vec<String>) -> Result<Vec<WebsiteStatus>, MonitorError> {
        let pool = thread::scope(|s| {
            let mut handles = Vec::new();
            
            for url in urls {
                let agent = Arc::clone(&self.agent);
                let max_retries = self.config.max_retries;
                
                let handle = s.spawn(move || {
                    Self::check_single_website(&agent, &url, max_retries)
                });
                handles.push(handle);
            }

            handles.into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        });

        Ok(pool)
    }

    fn check_single_website(agent: &ureq::Agent, url: &str, max_retries: u32) -> WebsiteStatus {
        let start = Instant::now();
        let mut retries = 0;
        let mut last_error = None;

        while retries < max_retries {
            match agent.get(url).call() {
                Ok(response) => {
                    return WebsiteStatus {
                        url: url.to_string(),
                        status: Ok(response.status()),
                        response_time: start.elapsed(),
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
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
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
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
    use std::net::TcpListener;
    use std::io::Write;
    use std::thread;

    fn create_test_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        });

        format!("http://127.0.0.1:{}", port)
    }

    #[test]
    fn test_successful_request() {
        let server_url = create_test_server();
        
        let config = Config {
            worker_threads: 4,
            timeout_seconds: 5,
            max_retries: 3,
        };

        let monitor = Monitor::new(config).unwrap();
        let results = monitor.check_websites(vec![server_url]).unwrap();

        assert_eq!(results.len(), 1);
        assert!(matches!(results[0].status, Ok(200)));
    }

    #[test]
    fn test_timeout_handling() {
        let config = Config {
            worker_threads: 4,
            timeout_seconds: 1,
            max_retries: 1,
        };

        let monitor = Monitor::new(config).unwrap();
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