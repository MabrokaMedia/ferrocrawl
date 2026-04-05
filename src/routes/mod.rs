mod extract;
mod health;
mod scrape;

pub use extract::extract_data;
pub use health::health_check;
pub use scrape::scrape_url;
