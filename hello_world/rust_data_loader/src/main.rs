use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

// Bitcoin data structures
#[derive(Debug, Serialize, Deserialize)]
struct BitcoinPrice {
    #[serde(rename = "bitcoin")]
    data: BitcoinData,
}

#[derive(Debug, Serialize, Deserialize)]
struct BitcoinData {
    usd: f64,
}

// Ethereum data structures
#[derive(Debug, Serialize, Deserialize)]
struct EthereumPrice {
    #[serde(rename = "ethereum")]
    data: EthereumData,
}

#[derive(Debug, Serialize, Deserialize)]
struct EthereumData {
    usd: f64,
}

// SP500 data structures
#[derive(Debug, Serialize, Deserialize)]
struct SP500Response {
    chart: SP500Chart,
}

#[derive(Debug, Serialize, Deserialize)]
struct SP500Chart {
    result: Vec<SP500Result>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SP500Result {
    meta: SP500Meta,
    timestamp: Vec<i64>,
    indicators: SP500Indicators,
}

#[derive(Debug, Serialize, Deserialize)]
struct SP500Meta {
    regularMarketPrice: f64,
    regularMarketTime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SP500Indicators {
    quote: Vec<SP500Quote>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SP500Quote {
    close: Vec<Option<f64>>,
}

// Main asset structs
struct Bitcoin {
    price: Option<f64>,
    filename: String,
    last_request: std::time::Instant,
    last_saved_price: Option<f64>,
}

struct Ethereum {
    price: Option<f64>,
    filename: String,
    last_request: std::time::Instant,
    last_saved_price: Option<f64>,
}

struct SP500 {
    price: Option<f64>,
    filename: String,
    last_request: std::time::Instant,
    last_saved_price: Option<f64>,
}

// Pricing trait
trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_to_file(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn should_fetch(&self) -> bool;
    fn price_changed(&self) -> bool;
}

// Implementation for Bitcoin
impl Bitcoin {
    fn new(filename: String) -> Self {
        Bitcoin {
            price: None,
            filename,
            last_request: std::time::Instant::now() - Duration::from_secs(11),
            last_saved_price: None,
        }
    }
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.should_fetch() {
            return Ok(());
        }

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response = ureq::get(url)
            .set("User-Agent", "Mozilla/5.0")
            .call()?;
        
        let price_data: BitcoinPrice = response.into_json()?;
        self.price = Some(price_data.data.usd);
        self.last_request = std::time::Instant::now();
        Ok(())
    }

    fn save_to_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(price) = self.price {
            if self.price_changed() {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let data = format!("{},{}\n", timestamp, price);
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.filename)?;
                file.write_all(data.as_bytes())?;
                self.last_saved_price = Some(price);
                println!("Bitcoin price updated to: ${:.2}", price);
            }
        }
        Ok(())
    }

    fn should_fetch(&self) -> bool {
        self.last_request.elapsed() >= Duration::from_secs(10)
    }

    fn price_changed(&self) -> bool {
        match (self.price, self.last_saved_price) {
            (Some(current), Some(last)) => (current - last).abs() > 0.01,
            (Some(_), None) => true,
            _ => false,
        }
    }
}

// Implementation for Ethereum
impl Ethereum {
    fn new(filename: String) -> Self {
        Ethereum {
            price: None,
            filename,
            last_request: std::time::Instant::now() - Duration::from_secs(11),
            last_saved_price: None,
        }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.should_fetch() {
            return Ok(());
        }

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url)
            .set("User-Agent", "Mozilla/5.0")
            .call()?;
        
        let price_data: EthereumPrice = response.into_json()?;
        self.price = Some(price_data.data.usd);
        self.last_request = std::time::Instant::now();
        Ok(())
    }

    fn save_to_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(price) = self.price {
            if self.price_changed() {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let data = format!("{},{}\n", timestamp, price);
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.filename)?;
                file.write_all(data.as_bytes())?;
                self.last_saved_price = Some(price);
                println!("Ethereum price updated to: ${:.2}", price);
            }
        }
        Ok(())
    }

    fn should_fetch(&self) -> bool {
        self.last_request.elapsed() >= Duration::from_secs(10)
    }

    fn price_changed(&self) -> bool {
        match (self.price, self.last_saved_price) {
            (Some(current), Some(last)) => (current - last).abs() > 0.01,
            (Some(_), None) => true,
            _ => false,
        }
    }
}

// Implementation for SP500
impl SP500 {
    fn new(filename: String) -> Self {
        SP500 {
            price: None,
            filename,
            last_request: std::time::Instant::now() - Duration::from_secs(11),
            last_saved_price: None,
        }
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.should_fetch() {
            return Ok(());
        }

        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response = ureq::get(url)
            .set("User-Agent", "Mozilla/5.0")
            .call()?;
        
        let sp500_data: SP500Response = response.into_json()?;
        if let Some(result) = sp500_data.chart.result.get(0) {
            self.price = Some(result.meta.regularMarketPrice);
        }
        self.last_request = std::time::Instant::now();
        Ok(())
    }

    fn save_to_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(price) = self.price {
            if self.price_changed() {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let data = format!("{},{}\n", timestamp, price);
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.filename)?;
                file.write_all(data.as_bytes())?;
                self.last_saved_price = Some(price);
                println!("S&P 500 price updated to: ${:.2}", price);
            }
        }
        Ok(())
    }

    fn should_fetch(&self) -> bool {
        self.last_request.elapsed() >= Duration::from_secs(10)
    }

    fn price_changed(&self) -> bool {
        match (self.price, self.last_saved_price) {
            (Some(current), Some(last)) => (current - last).abs() > 0.01,
            (Some(_), None) => true,
            _ => false,
        }
    }
}

fn main() {
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new("bitcoin_prices.csv".to_string())),
        Box::new(Ethereum::new("ethereum_prices.csv".to_string())),
        Box::new(SP500::new("sp500_prices.csv".to_string())),
    ];

    println!("Starting financial data fetcher...");
    println!("Press Ctrl+C to stop the program");
    println!("Prices will only be saved when they change");
    println!("Fetching data every 10 seconds");

    loop {
        for asset in &mut assets {
            match asset.fetch_price() {
                Ok(_) => match asset.save_to_file() {
                    Ok(_) => (),  // Only print when price actually changes
                    Err(e) => eprintln!("Error saving to file: {}", e),
                },
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
        }

        thread::sleep(Duration::from_secs(10));
    }
}