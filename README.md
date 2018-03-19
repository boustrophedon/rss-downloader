Rust rework of my old [rss\_torrent](https://github.com/boustrophedon/rss_torrent) with some extra usability features and more robust error handling. Part of the reason I wanted to do the rewrite is to download the feeds concurrently (and only once per feed) and do the processing in parallel, and just to use some of the libraries (csv, logging, structopt). The [structopt](https://github.com/TeXitoi/structopt/) library by @TeXitoi in particular is really nice.

The primary usecase for this is automatically downloading torrents from rss feeds, though the command that actually runs on the downloaded files is user-configured and they don't necessarily have to be torrent files. You could use this to download anything from an RSS feed and do something with it.

# Testing

I'm not sure if I'm going to put much effort into testing this because it's mostly just gluing a bunch of libraries together. There's like 2 whole pieces of actual logic in the entire program.

Things that could be tested:

- Alias command
	- Add alias, check it is there
	- Add two aliases, check they are both there
	- Add same alias twice, check it's only there once
	- Add several aliases and several aliases twice, make sure correct number is there
	- Change alias url, check it changed
- Add command
	- Same as Alias but with the addition of making sure that feeds with different filters don't get "deduplicated"
- Delete
	- Add things and delete them, check they were deleted
		- Make sure only the things that were supposed to be deleted were actually deleted!
- Update
	This would be the most useful to test because there's actually some small amount of logic here, but the large number of IO operations (download files, write to disk, parse existing feeds db, compare downloaded feed entries to existing feeds, run subprocess command) makes it annoying to do so.

In the end it's just not worth the effort to write tests because I'm going to write this code once and then probably never change it. 

# Error handling

One of the things that I wanted to do when writing this was to make error handling more robust. In comparison to the old single-file script it certainly is, and has nice logging. However, I'm just using boxed errors everywhere and while it works for this because I'm just logging everything, it wouldn't really work if I were writing this as a library. I might try to rewrite the boxed errors using the [failure](https://github.com/rust-lang-nursery/failure) crate instead.

Some of the nested matches are kind of ugly.

# Why CSV

Why use CSV files for the "databases" instead of eg sqlite? The data really isn't relational (the aliases are kind of relational I guess) and you might want to inspect or modify them manually. Also I wanted to try the csv crate.
