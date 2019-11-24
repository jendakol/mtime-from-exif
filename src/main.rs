#[macro_use]
extern crate clap;

use std::time::SystemTime;

use filetime::FileTime;
use rayon::prelude::*;
use regex::Regex;
use rexif::ExifTag;
use walkdir::{DirEntry, WalkDir};

fn handle_file(de: DirEntry, verbosity: u8) -> () {
    let re = Regex::new(r"(\d{4}):(\d{2}):(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap();

    if verbosity >= 3 {
        println!("Going to handle file {:?}", de.path())
    }

    match rexif::parse_file(de.path()) {
        Ok(exif) => {
            match exif.entries.into_iter()
                .find(|e| e.tag == ExifTag::DateTimeOriginal) {
                Some(entry) => {
                    let entry = entry.value.to_string();

                    if verbosity >= 2 {
                        println!("Going to handle file {:?}", de.path())
                    }

                    let caps = re.captures(entry.as_ref()).unwrap();

                    let mtime = format!(
                        "{}-{}-{} {}:{}:{}",
                        caps.get(1).unwrap().as_str(),
                        caps.get(2).unwrap().as_str(),
                        caps.get(3).unwrap().as_str(),
                        caps.get(4).unwrap().as_str(),
                        caps.get(5).unwrap().as_str(),
                        caps.get(6).unwrap().as_str()
                    );

                    let mtime: SystemTime = mtime.parse::<humantime::Timestamp>().unwrap().into();
                    let mtime: FileTime = FileTime::from(mtime);

                    if verbosity >= 1 {
                        println!("Changing file {:?} ({:?}) -> {:?}", de.path(), FileTime::from_last_modification_time(&std::fs::metadata(de.path()).unwrap()).unix_seconds(), mtime.unix_seconds());
                    }

                    filetime::set_file_mtime(de.path(), mtime).unwrap();
                },
                None => eprintln!("Could not find DateTimeOriginal tag in EXIF of {:?}", de.path())
            }
        },
        Err(e) => eprintln!("Could not read EXIF for {:?}: {}", de.path(), e)
    }
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.2")
        (author: "Jenda K. <jendakolena@gmail.com>")
        (about: "Changes JPG's mtime to DateTimeOriginal from their EXIF")
        (@arg PARALLELISM: -p --parallelism +takes_value "Sets parallelism value")
        (@arg DIR: +required "Working dir")
        (@arg VERBOSITY: -v ... "Sets the level of verbosity")
    )
        .get_matches();

    let root_dir = matches.value_of("DIR").unwrap_or_else(|| panic!("Working dir must have been provided"));
    let parallelism = value_t!(matches, "PARALLELISM", usize).unwrap_or(4);
    let verbosity = matches.occurrences_of("VERBOSITY") as u8;

    println!("Working in {} with parallelism {}", root_dir, parallelism);

    rayon::ThreadPoolBuilder::new().num_threads(parallelism).build_global().unwrap();

    WalkDir::new(root_dir)
        .same_file_system(true)
        .follow_links(false)
        .into_iter()
        .par_bridge()
        .map(|f| f.unwrap())
        .filter(|f| f.path().metadata().unwrap().file_type().is_file())
        .filter(|f| f.path().file_name().unwrap().to_str().unwrap().to_lowercase().ends_with("jpg"))
        .for_each(|f| handle_file(f, verbosity));
}
