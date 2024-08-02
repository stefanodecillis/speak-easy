use std::time::Duration;
use std::fs;
use std::io;


fn keep_last_logs(directory_path: &str, prefix: &str, holds_num: &usize) -> io::Result<()> {
    // Read the directory
    let mut entries = fs::read_dir(directory_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // Filter the entries to only include files that start with the prefix
    entries.retain(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with(prefix))
            .unwrap_or(false)
    });

    // Sort the entries by modified date in descending order
    entries.sort_by_key(|path| fs::metadata(path).and_then(|meta| meta.modified()).unwrap());
    entries.reverse();

    // Remove all but the last five entries
    for path in entries.into_iter().skip(*holds_num) {
        fs::remove_file(path)?;
    }

    Ok(())
}


#[cfg(feature="tokio_async")]
pub fn spawn_processor(directory_path: String, prefix: String, cleanup_interval: u64, keep_last: usize) {
    tokio::spawn(async move {
        loop {
            let _ = keep_last_logs(&directory_path, &prefix, &keep_last);
            tokio::time::sleep(Duration::from_secs(cleanup_interval)).await;
        }
    });
}

#[cfg(not(feature="tokio_async"))]
pub fn spawn_processor(directory_path: String, prefix: String, cleanup_interval: u64, keep_last: usize) {
    std::thread::spawn(move || {
        loop {
            let _ = keep_last_logs(&directory_path, &prefix, &keep_last);
            std::thread::sleep(Duration::from_secs(cleanup_interval));
        }
    });
}