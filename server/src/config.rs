use confique::Config as ConfigTrait;
use lazy_static::lazy_static;

#[derive(ConfigTrait)]
pub struct Config {
    #[config(default = "0.0.0.0:3000")]
    pub bind_address: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::builder()
        .env()
        .file("config.toml")
        .file("/etc/vexwireless/config.toml")
        .load()
        .unwrap();
}
