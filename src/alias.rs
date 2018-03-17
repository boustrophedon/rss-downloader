use std::collections::HashMap;
use std::error::Error;

use reqwest::Url;

use commands::RTAlias;
use config::RTConfig;

use alias_util;

pub type Aliases = HashMap<String, Alias>;

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub url: Url,
}

impl Alias {
    pub fn to_record(self) -> AliasRecord {
        AliasRecord {
            name: self.name,
            url: self.url.to_string(),
        }
    }
}

// Only pub so we can use it both here and in alias_util. Ideally alias_util would just be a
// submodule of alias but it's kind of annoying to do that.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AliasRecord {
    pub name: String,
    pub url: String,
}

impl AliasRecord {
    pub fn to_alias(self) -> Result<Alias, Box<Error>> {
        let url = Url::parse(&self.url)?;

        Ok(Alias {
            name: self.name,
            url: url,
        })
    }
}
pub fn add_alias(cmd: RTAlias, config: &RTConfig) {
    let result = Url::parse(&cmd.url);
    let url: Url;
    match result {
        Ok(u) => url = u,
        Err(err) => {
            error!("Invalid url: {}. Not updating aliases.", err);
            return;
        }
    }

    let result = alias_util::read_alias_db(config.data_dir.as_path());

    let mut aliases: Aliases;
    match result {
        Ok(a) => aliases = a,
        Err(err) => {
            error!("Could not read alias db: {}. Not updating aliases.", err);
            return;
        },
    }

    if aliases.contains_key(&cmd.alias) {
        info!("Updating alias {}", cmd.alias);
    }
    else {
        info!("Adding new alias {}", cmd.alias);
    }

    // I wish there were a csv::ReaderWriter
    let new_alias = Alias {name: cmd.alias.clone(), url: url};
    aliases.insert(cmd.alias.clone(), new_alias);

    match alias_util::write_alias_db(config.data_dir.as_path(), aliases) {
        Ok(()) => info!("Sucessfully updated alias db."),
        Err(err) => error!("Could not update alias db: {}", err),
    }
}
