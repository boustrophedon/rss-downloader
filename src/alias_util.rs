use csv;

use std::error::Error;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::Write;

use alias::{AliasRecord, Aliases};


const ALIAS_DB_FILENAME: &str = "aliases.csv";

/// Opens for read write and create because it's simpler
fn open_or_create_alias_db(data_dir: &Path) -> Result<File, Box<Error>> {
    trace!("Opening alias file.");

    let mut db_path = data_dir.to_path_buf();
    db_path.push(ALIAS_DB_FILENAME);

    if !db_path.exists() {
        warn!("Alias db not found at {}, creating.", db_path.to_string_lossy());
    }
    else {
        trace!("Alias db found at {}.", db_path.to_string_lossy());
    }

    Ok(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(db_path)?)
}

/// Read alias db or create if it does not exist.
pub fn read_alias_db(data_dir: &Path) -> Result<Aliases, Box<Error>> {
    debug!("Reading alias db.");

    let mut aliases = Aliases::new();

    let db_file = open_or_create_alias_db(data_dir)?;

    let mut reader = csv::Reader::from_reader(db_file);

    for line in reader.deserialize() {
        let record: AliasRecord = line?;
        let alias = record.to_alias()?;

        aliases.insert(alias.name.clone(), alias);
    }

    return Ok(aliases);
}

/// Write alias db or create if it does not exist.
pub fn write_alias_db(data_dir: &Path, mut aliases: Aliases) -> Result<(), Box<Error>> {
    debug!("Writing alias db.");

    let mut db_file = open_or_create_alias_db(data_dir)?;

    let mut buf = Vec::new();
    {
        let mut writer = csv::Writer::from_writer(&mut buf);
        for (_, alias) in aliases.drain() {
            trace!("Serializing alias {}", alias.name);
            writer.serialize(alias.to_record())?;
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
