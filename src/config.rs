pub struct Config {
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 3000,
            host: "127.0.0.1".to_string(),
        }
    }
    
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
} 