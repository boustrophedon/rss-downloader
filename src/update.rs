use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::path::PathBuf;

use rayon::prelude::*;

use chrono::{DateTime, OldDuration, TimeZone};

use reqwest::{self, Url, Client};
use rss;

use config::RTConfig;

use feed_util::{self, Feed};

pub fn run_update(config: &RTConfig) {
   let feeds_result = feed_util::read_feed_db(config.data_dir.as_path());

   let feeds = match feeds_result {
       Ok(f) => f,
       Err(err) => {
           error!("Could not read feed db: {}. Not updating feeds.", err);
           return;
       }
   };

   let mut urls = HashSet::new();

   for feed in &feeds {
       urls.insert(feed.url.clone());
   }

   info!("Downloading rss feeds.");
    let client = Client::new();
   let feed_data = get_feeds(&client, urls);
   let mut rss_feeds = HashMap::new();

   for (feed_url, res) in feed_data {
       match res {
           Ok(text) => {
               trace!("Sucessfully downloaded feed {}", feed_url);
               match text.parse::<rss::Channel>() {
                   Ok(channel) => {rss_feeds.insert(feed_url, channel);},
                   Err(err) => warn!("Feed at {} could not be parsed as rss feed: {}. Skipping.",
                                     feed_url.as_str(), err)
               };
           }
           Err(err) => {
               warn!("Feed at {} could not be downloaded: {}. Skipping.", feed_url.as_str(), err);
           }
       }
   }

   feeds.par_iter()
       .flat_map(|feed| get_update_urls(feed, &rss_feeds))
       .map(|url| download_feed_item(&client, &config, url))
       .forEach(|torrent_file| run_item_command(&config, torrent_file);
}


// not a boxed error because I couldn't figure out a way to turn reqwest::Error into
// Box<Error+Send>. Send is required for returning from the parallel iterator.
fn get_feeds(client: &Client, urls: HashSet<Url>) -> HashMap<Url, Result<String, reqwest::Error>> {
    urls.into_par_iter()
        .map(|url| {
            let data = client.get(url.as_str()).send()
                .and_then(|mut resp| resp.text());

            (url, data)
        })
        .collect()
}

// TODO this ideally would return a Vec<Result<Url, Box<Error>>> but I had trouble getting rayon to
// work nicely with boxed errors so for now it logs the errors internally and just returns a vector
// of the good cases.


/// Get the newest links from an rss feed relative to the last time we checked it.
fn get_update_urls(feed: &Feed, rss_feeds: &HashMap<Url, rss::Channel>) -> Vec<Url> {
    info!("Getting latest updates from feeds.");
    let updates = Vec::new();
    let rss_feed = match rss_feeds.get(feed.url) {
        Some(r) => r,
        None => {
            warn!("Feed {} not found in downloaded rss feeds. Not updating.", feed.url.as_str());
            return updates;
        }
    }
    info!("Finding updates for feed {}.", feed.url.as_str());

    for item in rss_feed {
        let date_result = match item.pub_date() {
            Some(date_str) => DateTime::parse_from_rfc2822(date_str),
            None => {
                warn!("Feed item in feed {} does not have pub date, skipping item.", rss_feed.title());
                continue;
            }
        }

        let pub_date = match date_result {
            Ok(d) => d,
            Err(e) => { 
                warn!("Could not parse date from feed {}: {}. Skipping item.", feed.url.as_str(), e);
                continue;
            }
        }

        let title = match item.title() {
            Some(t) => t,
            None => {
                warn!("Item in feed {} does not have title, skipping item.", feed.url.as_str());
                continue;
            }
        }

        let link = match item.link() {
            Some(l) => {
                match Url::parse(l) {
                    Ok(u) => u,
                    Err(e) => {
                        warn!("Item in feed has invalid link {}: {}. Skipping item.", l, e);
                        continue;
                    }
                }
            },
            None => {
                warn!("Item in feed {} does not have link, skipping item.", feed.url.as_str());
                continue;
            }
        }

        // there should be a nicer way to do this. we check if there is a last update, and if so
        // return once we find an older update.
        match feed.last_update {
            Some(last_update) => {
                // stop when the last update is more recent (or the same) as the current pub_date
                if last_update >= pub_date {
                    info!("Reached older update, finished reading feed {} items.", feed.url.as_str());
                    return updates;
                }
            }
            None => (),
        }

        if title_matches_filters(title, &feed.filters) {
           updates.push(link);
        }
    }

    // if the last item in the feed is still newer than the last_update, we can reach this
    return updates;
}

/// Download the content at the provided url to a file at a location dependent on the config
fn download_feed_item(client: &Client, config: &RTConfig, url: Url) -> Result<String, Box<Error>> {
    let contents = client.get(url).send()?.text()?;

    let filename: String;
    if let Some(segments) = url.path_segments() {
    
    }
    else {
        filename = "_".join([url.as_str().to_string(), Utc::now().timestamp().to_string()]);
    }

    let download_dir = get_download_dir(config.torrent_file_cache_dir);

}

fn find_download_dir(config_dir: Option<PathBuf>) {

}

fn title_matches_filters(title: &str, filters: &[String]) -> bool {
    let title_lower = title.to_lower();
    return filters.par_iter().all(|filter| {
        let filter_lower = filter.to_lower();
        title_lower.contains(filter_lower)
    });
}
