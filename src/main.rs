use std::time::SystemTime;

use filetime::FileTime;
use regex::Regex;
use rexif::ExifTag;
use walkdir::{DirEntry, WalkDir};

fn handle_file(de: DirEntry) -> () {
    let re = Regex::new(r"(\d{4}):(\d{2}):(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap();

    match rexif::parse_file(de.path()) {
        Ok(exif) => {
            match exif.entries.into_iter()
                .find(|e| e.tag == ExifTag::DateTimeOriginal) {
                Some(entry) => {
                    let entry = entry.value.to_string();
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

                    println!("{:?} ({:?}) -> {:?}", de.path(), FileTime::from_last_modification_time(&std::fs::metadata(de.path()).unwrap()).unix_seconds(), mtime.unix_seconds());

                    filetime::set_file_mtime(de.path(), mtime).unwrap();
                },
                None => eprintln!("Could not find DateTimeOriginal tag in EXIF of {:?}", de.path())
            }
        },
        Err(e) => eprintln!("Could not read EXIF for {:?}: {}", de.path(), e)
    }
}

fn main() {
    let root_dir = std::env::args().skip(1).next().unwrap_or("Root dir path was not provided".to_string());

    println!("Working dir: {}", root_dir);

    WalkDir::new(root_dir)
        .same_file_system(true)
        .follow_links(false)
        .into_iter()
        .map(|f| f.unwrap())
        .filter(|f| f.path().metadata().unwrap().file_type().is_file())
        .filter(|f| f.path().file_name().unwrap().to_str().unwrap().to_lowercase().ends_with("jpg"))
        .for_each(handle_file);
}
