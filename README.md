Rust rework of my old [rss\_torrent](https://github.com/boustrophedon/rss_torrent) with some extra usability features and more robust error handling. Part of the reason I wanted to do the rewrite is to try caching the feeds in a hashmap and sharing it across multiple threads, and just to use some of the libraries (csv, logging, structopt). The [structopt](https://github.com/TeXitoi/structopt/) library by @TeXitoi in particular is really nice.

The primary usecase for this is automatically downloading torrents from rss feeds, though the command that actually runs on the files that are downloaded is user-configured and the files don't necessarily have to be torrent files, i.e. you could use this to download anything from an RSS feed and do something with it.

# Testing

I'm not sure if I'm going to put much effort into testing this because it's mostly just gluing a bunch of libraries together. There's like 2 whole pieces of actual logic in the entire program.

# Why CSV

Why use CSV files for the "databases" instead of eg sqlite? The data really isn't relational (the aliases are kind of relational I guess) and you might want to inspect or modify them manually.
