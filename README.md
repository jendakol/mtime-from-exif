# MTIME from EXIF

This is very simple and almost without-error-handling utility for updating JPEG file's [mtime](https://en.wikipedia.org/wiki/Stat_(system_call)
to their `DateOriginalTime` from [EXIF](https://en.wikipedia.org/wiki/Exif).  
Tested only on Debian 10 and Ubuntu 19.04.

Usage:

```bash
git clone https://github.com/jendakol/mtime-from-exif.git
cd mtime-from-exif
cargo run --release -- /data/Photos/2018 
```

This will update the mtime for all photos (*.jpg, case insensitive) with parseable EXIF data.

Take it or leave it.
