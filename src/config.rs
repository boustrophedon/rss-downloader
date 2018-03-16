use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use toml;

const DEFAULT_DB_PATH: &str = "/usr/local/share/rss-torrent/feeds.csv";

const CONFIG_ENV_VAR: &str = "RSS_TORRENT_CONFIG";

const HOME_CONFIG_RELPATH: &str = ".config/rss_torrent.toml";
const USR_LOCAL_CONFIG_PATH: &str = "/usr/local/etc/rss_torrent.toml";
const ETC_CONFIG_PATH: &str = "/etc/rss_torrent.toml";

// TODO use failure crate instead of a boxed error

#[derive(Debug, Clone)]
pub struct RTConfig {
    pub db_location: PathBuf,
    pub torrent_add_cmd: String,
    pub torrent_add_args: Vec<String>,
    pub torrent_file_cache_dir: Option<PathBuf>,
}

impl Default for RTConfig {
    fn default() -> RTConfig {
        RTConfig {
            db_location: PathBuf::from(DEFAULT_DB_PATH),
            torrent_add_cmd: "transmission-remote".to_string(),
            torrent_add_args: vec!["-a".to_string(), "_TORRENT_PATH".to_string(), "-sr".to_string(), 50.to_string()],
            torrent_file_cache_dir: None,
        }
    }
}

// Deserialize to this struct, then convert to actual RTConfig struct
#[derive(Debug, Clone, Deserialize)]
pub struct RTConfigValues {
    db_location: String,
    torrent_add_cmd: String,
    torrent_add_args: Vec<String>,
    torrent_file_cache_dir: Option<String>,
}

impl RTConfigValues {
    pub fn to_config(self) -> RTConfig {
        RTConfig {
            db_location: PathBuf::from(self.db_location),
            torrent_add_cmd: self.torrent_add_cmd,
            torrent_add_args: self.torrent_add_args,
            torrent_file_cache_dir: self.torrent_file_cache_dir.map(|v| PathBuf::from(v)),
        }
    }
}

impl RTConfig {
    pub fn from_file(mut f: File) -> Result<RTConfig, Box<Error>> {
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err)),
        }

        // I kind of prefer to write returns like this explictly
        // but it's easier to do it like this
        match toml::from_str::<RTConfigValues>(&contents) {
            Ok(config_raw) => return Ok(config_raw.to_config()),
            Err(err) => return Err(Box::new(err)),
        }
    }

    pub fn new(config_arg: Option<String>) -> RTConfig {
        // first check if arg was passed
        // else, check $RSS_TORRENT_CONFIG
        // else, check $HOME/.config/rss_torrent.toml
        //      this should actually be $XDG_CONFIG_DIR or something
        // else, check /usr/local/etc/rss_torrent.toml
        // else, check /etc/rss_torrent.toml
        // there probably is a crate for this (maybe config-rs does this? I couldn't tell)
        

        let mut conf_files = Vec::new();
        if let Some(config_path) = config_arg {
            conf_files.push(PathBuf::from(config_path));
        }

        let config_var = env::var(CONFIG_ENV_VAR);
        if let Ok(var_file) = config_var {
            conf_files.push(PathBuf::from(var_file));
        }
        else {
            trace!("No config env var found");
        }

        let home = env::home_dir();
        if let Some(home_dir) = home {
            let mut f = home_dir;
            f.push(HOME_CONFIG_RELPATH);
            conf_files.push(PathBuf::from(f));
        }
        else {
            trace!("No home directory found")
        }

        conf_files.push(PathBuf::from(USR_LOCAL_CONFIG_PATH));
        conf_files.push(PathBuf::from(ETC_CONFIG_PATH));
   
        for path in conf_files {
            let f = File::open(&path);
            match f {
                Ok(file) => {
                    match RTConfig::from_file(file) {
                        Ok(config) => {
                            info!("Found valid config file: {}", path.to_string_lossy());
                            return config;
                        },
                        Err(err) => {
                            warn!("Invalid config file {}: {}", path.to_string_lossy(), err);
                        }
                    }
                },
                Err(err) => {
                    debug!("Error trying to open config file {}: {}", path.to_string_lossy(), err);
                }
            }
        }

        // if none work, use default values
        warn!("No config files found, using default values");
        return Default::default();
    }

}
