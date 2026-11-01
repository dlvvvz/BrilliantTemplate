pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub bind: String,
}
impl Config {
    pub fn load() -> Config {
        Config {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            bind: std::env::var("BIND").unwrap_or("127.0.0.1:8080".to_string()),
        }
    }
}