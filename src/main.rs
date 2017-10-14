extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

mod commands;
use commands::RTArgs;

fn main() {
    // As an experiment I'm just going to leave all of my planning/structure comments here in main
    // instead of deleting them or leaving them in a separate uncommitted file like usual.


    // parse args
    //
    //      -u / --update option forces an update after other operations are completed
    //      -c / --config option overrides the directory for database files
    //
    // determine if files exist
    //      first check if commandline arg specifies config directory
    //      else use xdg crate to check $XDG_CONFIG_DIR/rss-torrent/
    //      else make them using xdg crate
    //
    // match on command:
    //      add
    //      alias
    //      update
    //      delete
    //
    // add: add new feed to database
    //      rss-torrent [args] add URL_OR_ALIAS [filter terms]
    //      
    //  database path is passed in as part of context/opts argument
    //  alias path is passed in as part of context/ops argument
    //
    //  open database path in append mode with
    //  `let mut db = OpenOptions::new().append(true).open(database_path)`
    //  and pass to csv writer with
    //  `let mut writer = csv::Writer::from_writer(db);`
    //  open aliases via csv and shove into hashmap
    //  
    //
    //  check if URL_OR_ALIAS is alias, and if so replace with the feed url
    //  create entry struct from args
    //      #[derive(Debug, Serialize, Deserialize)]
    //      struct Entry {
    //          url: String,
    //          lastupdate: i64,
    //          filter: Option<String>
    //      }
    //      lastupdate should be a unix timestamp. it will be set to 0 on creation which will in
    //      most cases cause the feed to update the next time the update command is run unless your
    //      feed is providing torrents from the past.
    //
    //  serialize with writer.serialize(entry)
    //  flush
    //
    // alias: create an alias for a feed
    //      rss-torrent [args] alias URL ALIAS
    //
    //      this is useful for private tracker feeds
    //
    //  context arg like in add
    //  open aliases in append mode as in add command and pass to csv writer
    //
    //  create Alias from args
    //      #[derive(Debug, Serialize, Deserialize)]
    //      struct Alias {
    //          name: String,
    //          url: String,
    //      }
    //      
    //  serialize with writer.serialize(entry)
    //  flush
    //
    // update: pull new torrents from feeds in database
    //      rss-torrent [args] update
    //     
    //      this is the command you will want in your cron job or service manager unit file
    //
    //  open db for reading and parse into vec of structs, logging errors
    //  drop the csv::Reader so we can
    //  open db for writing
    //
    //  for each entry: 
    //  pull feed, check pubdates and filters and add torrents until we get past the last update time for the current entry
    //  keep track of newest pubdate seen, set lastupdate, serialize
    //
    // delete: remove feed from database
    //      rss-torrent [args] delete [--dry-run] URL_OR_ALIAS [filter terms]
    //      rss-torrent [args] delete [--dry-run] --all [filter terms]
    //
    //      --dry-run just prints out feeds that will be deleted
    //      filter terms may be a subset of the actual filters. if --all is passed we check every
    //      entry in the db
    //
    //  open alias db and replace alias with url if applicable
    //
    //  open db for reading, parse into vec of structs as in update
    //  drop Reader
    //  open for writing
    //
    //  for each entry, check if url matches or if --all is in effect, then match against filters
    //  if it doesn't match, move to output vector
    //  serialize all in output vector
    //  TODO: for this and for update, see if there's a read/write csv reader
    //
    //  Other TODO:
    //  add lock file to prevent e.g. update being run via cron while you're manually running an
    //  add or delete
}
