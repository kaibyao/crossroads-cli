/// The name of the config file
pub static CONFIG_FILE_NAME: &str = "xr.toml";

/// This struct is used to generate the CONFIG_FILE_NAME config file
#[derive(Serialize)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String
}
