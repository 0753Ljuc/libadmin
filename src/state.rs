pub struct AppState {
    pub secret: Vec<u8>,
}

impl AppState {
    pub fn new() -> Self {
        let secret = std::env::var("SECRET_KEY").unwrap_or_default().into_bytes();
        Self { secret }
    }
}
