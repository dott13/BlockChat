pub fn get_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}