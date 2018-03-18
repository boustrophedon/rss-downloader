use reqwest::Url;

use commands::RTAdd;
use config::RTConfig;

use feed_util::{self, Feeds, Feed};
use alias::Aliases;
use alias_util;

pub fn add_feed(cmd: RTAdd, config: &RTConfig) {
    let result = Url::parse(&cmd.url_or_alias);
    let url: Url;
    match result {
        Ok(u) => url = u,
        Err(_) => {
            trace!("url_or_alias is not a valid url, checking if it is an alias...");
            let aliases_result = alias_util::read_alias_db(config.data_dir.as_path());
            let aliases: Aliases;
            match aliases_result {
                Ok(a) => aliases = a,
                Err(alias_err) => {
                    error!("Could not read alias db: {}", alias_err);
                    error!("Not adding feed because url_or_alias could not be understood.");
                    return;
                }
            }
            match aliases.get(&cmd.url_or_alias) {
                Some(alias) => url = alias.url.clone(),
                None => {
                    error!("Not adding feed because url_or_alias {} was not a url or a valid alias.", &cmd.url_or_alias);
                    return;
                }
            }
        }
    }

    let result = feed_util::read_feed_db(config.data_dir.as_path());

    let mut feeds: Feeds;
    match result {
        Ok(read_feeds) => feeds = read_feeds,
        Err(err) => {
            error!("Could not read feed db: {}. Not adding feed.", err);
            return;
        },
    }

    trace!("Read feeds db with {} entries.", feeds.len());

    // TODO add option to set time to None or to current time
    let new_feed = Feed {url: url, last_update: None, filters: cmd.filters};

    if feeds.contains(&new_feed) {
        error!("Feed with url {} and filters \"{}\" already exists in db. Not adding feed.",
               new_feed.url.as_str(),
               new_feed.filters.join(",")
        );
        return;
    }
    else {
        info!("Adding new feed {} with filters \"{}\"", new_feed.url.as_str(),
              new_feed.filters.join(",")
        );
    }

    feeds.insert(new_feed);

    match feed_util::write_feed_db(config.data_dir.as_path(), feeds) {
        Ok(()) => info!("Sucessfully added feed to db."),
        Err(err) => error!("Could not add feed to db: {}", err),
    }
}
