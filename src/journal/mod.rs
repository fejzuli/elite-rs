use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use directories::UserDirs;

pub use events::*;

mod events;

pub fn journals_path() -> PathBuf {
    let user_dirs = UserDirs::new().expect("I'm sorry but your OS sucks :(");
    let home_dir = user_dirs
        .home_dir()
        .to_str()
        .expect("Your home directory name contains some ancient runes you imbecile >:(");

    [
        home_dir,
        "Saved Games",
        "Frontier Developments",
        "Elite Dangerous",
    ]
    .iter()
    .collect()
}

pub fn journal_files() -> Result<Vec<PathBuf>, io::Error> {
    let mut paths = fs::read_dir(journals_path())?
        .map(|res| res.map(|e| e.path()))
        .filter(|res| match res {
            Ok(p) => p
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("Journal"),
            Err(_) => true,
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    paths.sort();

    Ok(paths)
}

pub fn latest_journal_path() -> Result<PathBuf, io::Error> {
    let paths = journal_files()?;

    Ok(paths
        .last()
        .expect("No journal file found. Please start the game at least once")
        .to_owned())
}

pub fn backpack_path() -> PathBuf {
    let mut path = journals_path();
    path.push("Backpack.json");
    path
}

pub fn cargo_path() -> PathBuf {
    let mut path = journals_path();
    path.push("Cargo.json");
    path
}

pub fn market_path() -> PathBuf {
    let mut path = journals_path();
    path.push("Market.json");
    path
}

pub fn modules_info_path() -> PathBuf {
    let mut path = journals_path();
    path.push("ModulesInfo.json");
    path
}

pub fn nav_route_path() -> PathBuf {
    let mut path = journals_path();
    path.push("NavRoute.json");
    path
}

pub fn outfitting_path() -> PathBuf {
    let mut path = journals_path();
    path.push("Outfitting.json");
    path
}

pub fn ship_locker_path() -> PathBuf {
    let mut path = journals_path();
    path.push("ShipLocker.json");
    path
}

pub fn shipyard_path() -> PathBuf {
    let mut path = journals_path();
    path.push("Shipyard.json");
    path
}

pub fn status_path() -> PathBuf {
    let mut path = journals_path();
    path.push("Status.json");
    path
}

pub fn all_events() -> Result<Vec<Event>, io::Error> {
    let mut events: Vec<Event> = Vec::new();

    for path in journal_files()? {
        for line in BufReader::new(File::open(path)?).lines() {
            events.push(serde_json::from_str(line?.as_str())?);
        }
    }

    Ok(events)
}
