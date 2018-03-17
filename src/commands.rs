#[derive(StructOpt, Debug)]
#[structopt(name = "rss-torrent")]
pub struct RTArgs {
    #[structopt(short = "u", long = "update")]
    /// Force an update check after running this command.
    pub update: bool,

    #[structopt(short = "v", parse(from_occurrences))]
    /// Verbosity level. Without this flag, only errors will be printed to stdout. Increasing
    /// repetitions of this flag will produce more output.
    pub verbosity: u64,

    #[structopt(short = "c", long = "config")]
    /// Override the default configuration directory.
    pub config: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<RTCommand>,
}

#[derive(StructOpt, Debug)]
pub enum RTCommand {
    #[structopt(name = "add")]
    /// Add a feed to the database with optional filters. The feed url may be a url or a
    /// preexisting alias.
    Add(RTAdd),

    #[structopt(name = "alias")]
    /// Add an alias for a rss feed url.
    Alias(RTAlias),

    #[structopt(name = "update")]
    /// Run an update, fetching and parsing feeds and adding torrents not seen since the last update.
    Update,

    #[structopt(name = "delete")]
    /// Delete a feed from the database. `url_or_alias` is optional, but if the `--filters` option
    /// is used without `--all` or a `url_or_alias`, this command does nothing.
    Delete(RTDelete)
}

#[derive(StructOpt, Debug)]
pub struct RTAdd {
    /// A RSS feed url or an existing alias.
    pub url_or_alias: String,

    /// A list of filters that each item in the feed will be matched against. The search checks
    /// that each word (indepedently) of the filter is somewhere in the title of the item, and
    /// if all of them are, the item matches.
    pub filters: Vec<String>,
}

#[derive(StructOpt, Debug)]
pub struct RTAlias {
    /// An alias for this url which can be used in `add` and `delete` commands.
    pub alias: String,

    /// The url of an RSS feed.
    pub url: String,
}

#[derive(StructOpt, Debug)]
pub struct RTDelete {
    /// Delete all feeds from the database with the given url, or a url that matches a given
    /// alias's url.
    pub url_or_alias: Option<String>,

    #[structopt(long = "dry-run")]
    /// Do not modify the database, only print out the feeds that will be deleted plus their
    /// fiters.
    pub dry_run: bool,

    #[structopt(long = "all")]
    /// Search all feeds that match the given filters rather than just ones with the same
    /// url_or_alias.
    pub search_all: bool,

    /// Delete only feeds with filters matching the ones passed.
    #[structopt(long = "filters")]
    pub filters: Vec<String>
}
