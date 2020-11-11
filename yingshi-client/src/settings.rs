use config::{ConfigError, Config, File, Environment};
use std::env::var;
use tokio::fs;
use tokio::prelude::*;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref GLOBAL_CONFIG_PATH: String = {
        let config_path = var("XDG_CONFIG_HOME").or_else(|_| var("HOME").map(|home|format!("{}/.config", home))).unwrap();
        format!("{}/.ys.toml", config_path)
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub key: String,
    pub secret: String,
    pub token: Option<String>,
    pub expire_time: Option<i64>,
    pub default_device_serial: Option<String>,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(File::with_name(GLOBAL_CONFIG_PATH.as_str()).required(false))?;
        config.merge(Environment::with_prefix("YINGSHI"))?;

        config.try_into()
    }

    pub fn new(key: String, secret: String) -> Settings {
        Settings {
            key,
            secret,
            token: None,
            expire_time: None,
            default_device_serial: None
        }
    }

    /// 将配置文件写入文件系统，配置文件保存在 $HOME/.config/.ys.toml中
    pub async fn save(&self) -> std::io::Result<()> {
        let content = toml::to_string(self).expect("fail to serialize");
        let mut f = fs::File::create(GLOBAL_CONFIG_PATH.as_str()).await?;
        f.write_all(content.as_bytes()).await?;

        Ok(())
    }
}