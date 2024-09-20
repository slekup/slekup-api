fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} is not set", var_name))
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server_address: String,
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        let server_address = get_env_var("SERVER_ADDRESS");
        let database_url = get_env_var("DATABASE_URL");

        Config {
            server_address,
            database_url,
        }
    }
}
