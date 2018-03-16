use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::path::PathBuf;

use toml;

const DEFAULT_DATA_DIR: &str = "/usr/local/share/rss-torrent/";

const CONFIG_ENV_VAR: &str = "RSS_TORRENT_CONFIG";

const HOME_CONFIG_RELPATH: &str = ".config/rss_torrent.toml";
const USR_LOCAL_CONFIG_PATH: &str = "/usr/local/etc/rss_torrent.toml";
const ETC_CONFIG_PATH: &str = "/etc/rss_torrent.toml";

// TODO use failure crate instead of a boxed error

/// `data_dir` is the directory where the databases are stored. `torrent_file_cache_dir` is an
/// optional directory where downloaded torrent files will be stored.
#[derive(Debug, Clone)]
pub struct RTConfig {
    pub data_dir: PathBuf,
    pub torrent_add_command: String,
    pub torrent_add_args: Vec<String>,
    pub torrent_file_cache_dir: Option<PathBuf>,
}

impl Default for RTConfig {
    fn default() -> RTConfig {
        RTConfig {
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            torrent_add_command: "transmission-remote".to_string(),
            torrent_add_args: vec!["-a".to_string(), "_TORRENT_PATH".to_string(), "-sr".to_string(), 50.to_string()],
            torrent_file_cache_dir: None,
        }
    }
}

// Deserialize to this struct, then convert to actual RTConfig struct
#[derive(Debug, Clone, Deserialize)]
pub struct RTConfigValues {
    data_dir: String,
    torrent_add_command: String,
    torrent_add_args: Vec<String>,
    torrent_file_cache_dir: Option<String>,
}

impl RTConfigValues {
    // Not sure if I should really be using io::Error here but these are io errors...
    pub fn to_config(self) -> Result<RTConfig, io::Error> {
        let data_dir = PathBuf::from(&self.data_dir);
        let torrent_file_cache_dir = self.torrent_file_cache_dir.map(|v| PathBuf::from(v));

        if !data_dir.exists() {
            return Err(io::Error::new(ErrorKind::NotFound,
                                      format!("Data directory not found: {}", data_dir.to_string_lossy())));
        }
        if !data_dir.is_dir() {
            return Err(io::Error::new(ErrorKind::InvalidData,
                                      format!("Data directory is not a directory: {}",
                                              data_dir.to_string_lossy())));
        }

        if let Some(ref cache_dir) = torrent_file_cache_dir {
            if !cache_dir.exists() {
                return Err(io::Error::new(ErrorKind::NotFound,
                                          format!("Torrent cache directory not found: {}",
                                                  cache_dir.to_string_lossy())));
            }

            if !cache_dir.is_dir() {
                return Err(io::Error::new(ErrorKind::InvalidData,
                                          format!("Torrent cache directory is not a directory: {}",
                                                  cache_dir.to_string_lossy())));
            }
        }

        // TODO could check that torrent command is also valid, but that there isn't anything in
        // stdlib that searches the path for you. So we'll leave handling that error to the actual
        // std::process::Command result.

        Ok(RTConfig {
            data_dir: data_dir,
            torrent_add_command: self.torrent_add_command,
            torrent_add_args: self.torrent_add_args,
            torrent_file_cache_dir: torrent_file_cache_dir,
        })
    }
}

impl RTConfig {
    pub fn from_file(mut f: File) -> Result<RTConfig, Box<Error>> {
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        // I'm not really sure why I have to do the `as Box<Error>` here but not above.
        toml::from_str::<RTConfigValues>(&contents)?.to_config().map_err(|e| Box::new(e) as Box<Error>)
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
