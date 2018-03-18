use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::fs::{File, OpenOptions};

use std::io::Write;

use chrono::{DateTime, FixedOffset};

use csv;

use reqwest::Url;


pub type Feeds = HashSet<Feed>;

// TODO here and in alias I did this feed/feedrecord thing because it seemed easier than
// impl'ing Serialize and Deserialize but since Url is the only thing that doesn't have a
// ser/de impl already it might not be that difficult. 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Feed {
    pub url: Url,
    pub last_update: Option<DateTime<FixedOffset>>,
    pub filters: Vec<String>,
}

impl Feed {
    pub fn to_record(self) -> FeedRecord {
   
        // workaround for https://github.com/BurntSushi/rust-csv/issues/110
        let filters: Option<Vec<String>>;
        if self.filters.len() > 0 {filters = Some(self.filters);}
        else { filters = None; }

        FeedRecord {
            url: self.url.to_string(),
            last_update: self.last_update,
            filters: filters,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedRecord {
    pub url: String,
    pub last_update: Option<DateTime<FixedOffset>>,
    pub filters: Option<Vec<String>>,
}

impl FeedRecord {
    pub fn to_feed(self) -> Result<Feed, Box<Error>> {
        let url = Url::parse(&self.url)?;

        // workaround for https://github.com/BurntSushi/rust-csv/issues/110
        let filters: Vec<String>;
        match self.filters {
            Some(f) => filters = f,
            None => filters = Vec::new(),
        }

        Ok(Feed {
            url: url,
            last_update: self.last_update,
            filters: filters,
        })
    }
}


// So yeah this is pretty much copied and pasted from alias_utils and they could be combined but it
// would be a bit annoying because of the logging. It wouldn't be that hard but since there's only
// two files it's fine.

const FEED_DB_FILENAME: &str = "feeds.csv";

/// Opens for read write and create because it's simpler
fn open_or_create_feed_db(data_dir: &Path) -> Result<File, Box<Error>> {
    trace!("Opening feeds file.");

    let mut db_path = data_dir.to_path_buf();
    db_path.push(FEED_DB_FILENAME);

    if !db_path.exists() {
        warn!("Feeds db not found at {}, creating.", db_path.to_string_lossy());
    }
    else {
        trace!("Feeds db found at {}.", db_path.to_string_lossy());
    }

    Ok(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(db_path)?)
}

/// Read feed db or create if it does not exist.
pub fn read_feed_db(data_dir: &Path) -> Result<Feeds, Box<Error>> {
    debug!("Reading feed db.");

    let mut feeds = Feeds::new();

    let db_file = open_or_create_feed_db(data_dir)?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true) // may have any number of filters
        .from_reader(db_file);

    for line in reader.deserialize() {
        let record: FeedRecord = line?;
        let feed = record.to_feed()?;
        trace!("Deserialized feed with url {}", feed.url.as_str());

        feeds.insert(feed);
    }

    return Ok(feeds);
}

/// Write feed db or create if it does not exist.
pub fn write_feed_db(data_dir: &Path, mut feeds: Feeds) -> Result<(), Box<Error>> {
    debug!("Writing feed db.");

    let mut db_file = open_or_create_feed_db(data_dir)?;

    let mut buf = Vec::new();
    {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .flexible(true) // may have any number of filters
            .from_writer(&mut buf);

        for feed in feeds.drain() {
            trace!("Serializing feed {}", feed.url.as_str());
            writer.serialize(feed.to_record())?;
        }
    }
  
    // there should be a better way of doing this
    let result = db_file.write_all(&mut buf).map_err(|e| {
        error!("Error writing to file. DATA MAY BE CORRUPTED!");
        e
    })
    .and_then(|_| {
        db_file.set_len(buf.len() as u64)
    });

    Ok(result?)
}
