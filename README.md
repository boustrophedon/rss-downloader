Rust rework of my old [rss\_torrent](https://github.com/boustrophedon/rss_torrent) with some extra usability features and more robust error handling. Part of the reason I wanted to do the rewrite is to try caching the feeds in a hashmap and sharing it across multiple threads.

The primary usecase for this is for automatically downloading torrents from rss feeds, though the command that actually runs on the files that are downloaded is user-configured and the files don't necessarily have to be torrent files.
