// NOTE: Due to a bug in structopt all doc comments have to be on one line in order to properly get
// into the help text :(

#[derive(StructOpt, Debug)]
#[structopt(name = "rss-torrent")]
pub struct RTArgs {
    #[structopt(short = "u", long = "update")]
    /// Force an update check after running this command.
    update: bool,

    #[structopt(short = "c", long = "config")]
    /// Override the default configuration directory.
    config: Option<String>,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "add")]
    /// Add a feed to the database with optional filters. The feed url may be a url or a preexisting alias.
    Add {
        /// A RSS feed url or an existing alias.
        url_or_alias: String,

        /// A list of filters that each item in the feed will be matched against. The search checks that each word (indepedently) of the filter is somewhere in the title of the item, and if all of them are, the item matches.
        filters: Vec<String>,
    },

    #[structopt(name = "alias")]
    /// Add an alias for a rss feed url.
    Alias {
        /// The url of an RSS feed.
        url: String,

        /// An alias for this url which can be used in `add` and `delete` commands.
        alias: String,
    },

    #[structopt(name = "update")]
    /// Run an update, fetching and parsing feeds and adding torrents not seen since the last update.
    Update,

    #[structopt(name = "delete")]
    /// Delete a feed from the database. `url_or_alias` is optional, but if the `--filters` option is used without `--all` or a `url_or_alias`, this command does nothing.
    Delete {
        /// Delete all feeds from the database with the given url, or a url that matches a given alias's url.
        url_or_alias: Option<String>,

        #[structopt(long = "dry-run")]
        /// Do not modify the database, only print out the feeds that will be deleted plus their fiters.
        dry_run: bool,

        #[structopt(long = "all")]
        /// Search all feeds that match the given filters rather than just ones with the same url_or_alias.
        search_all: bool,

        /// Delete only feeds with filters matching the ones passed.
        #[structopt(long = "filters")]
        filters: Vec<String>
    },
}
