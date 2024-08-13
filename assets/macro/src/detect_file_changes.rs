use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileInfo {
    modified: std::time::SystemTime,
    hash: String,
}

#[derive(Debug)]
pub enum FileChange {
    Added,
    Modified,
    Deleted,
}

pub fn has_any_file_changed<P: AsRef<Path>>(dir: P, cache_file_name: &str) -> bool {
    let changed_files = list_changed_files(dir, cache_file_name);
    changed_files.len() > 0
}

// TODO: Eventually garbage collect the cache since the current implementation lets it
// grow without bound.
pub fn has_file_changed<P: AsRef<Path>>(path_to_file: P, cache_file_name: &str) -> bool {
    let rt = Runtime::new().expect("Couldn't create Tokio runtime.");
    rt.block_on(async {
        let path = path_to_file.as_ref();
        let cache_file_path = paths::path_to_detect_file_changes_cache(cache_file_name);

        let mut previous_metadata = load_previous_metadata(&cache_file_path);

        let current_hash = calculate_hash(path)
            .await
            .expect("Error calculating file hash.");
        let metadata = fs::metadata(path).expect("Error reading file metadata.");
        let current_modified = metadata
            .modified()
            .expect("Error reading file modified time.");

        let file_path_str = path.to_string_lossy().to_string();

        let has_changed = match previous_metadata.get(&file_path_str) {
            Some(previous_info) => {
                previous_info.hash != current_hash || previous_info.modified != current_modified
            }
            None => true, // File is new or untracked
        };

        if has_changed {
            // Update the cache with the new FileInfo
            previous_metadata.insert(
                file_path_str.clone(),
                FileInfo {
                    modified: current_modified,
                    hash: current_hash,
                },
            );

            // Save the updated metadata back to the cache file
            save_metadata(cache_file_path, &previous_metadata);
        }

        has_changed
    })
}

pub fn list_changed_files<P: AsRef<Path>>(
    dir: P,
    cache_file_name: &str,
) -> Vec<(String, FileChange)> {
    let rt = Runtime::new().expect("Couldn't create Tokio runtime."); // Create a new runtime for blocking within this synchronous function
    rt.block_on(async {
        let cache_file_path = paths::path_to_detect_file_changes_cache(cache_file_name);
        let previous_metadata = load_previous_metadata(&cache_file_path);
        let current_metadata = scan_directory(dir).await;

        let mut changes = Vec::new();

        // Check for modified or new files
        for (path, current_info) in &current_metadata {
            match previous_metadata.get(path) {
                Some(previous_info) => {
                    if previous_info.hash != current_info.hash {
                        changes.push((path.clone(), FileChange::Modified));
                    }
                }
                None => changes.push((path.clone(), FileChange::Added)), // New file
            }
        }

        // Check for deleted files
        for path in previous_metadata.keys() {
            if !current_metadata.contains_key(path) {
                changes.push((path.clone(), FileChange::Deleted));
            }
        }

        save_metadata(cache_file_path, &current_metadata);

        changes
    })
}

async fn calculate_hash<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let file = File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 1024];

    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

async fn scan_directory<P: AsRef<Path>>(dir: P) -> HashMap<String, FileInfo> {
    let mut file_info_map = HashMap::new();
    let walker = WalkDir::new(dir)
        .into_iter()
        .filter_entry(|entry| entry.file_name() != "target" && entry.file_name() != ".DS_Store");

    for entry in walker {
        let entry = entry.expect("Error reading directory entry.");
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(path).expect("Error reading file metadata.");
            let modified = metadata
                .modified()
                .expect("Error reading file modified time.");
            let hash = calculate_hash(path)
                .await
                .expect("Error calculating file hash.");

            file_info_map.insert(
                path.to_string_lossy().to_string(),
                FileInfo { modified, hash },
            );
        }
    }

    file_info_map
}

fn load_previous_metadata<P: AsRef<Path>>(path: P) -> HashMap<String, FileInfo> {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_metadata<P: AsRef<Path>>(path: P, metadata: &HashMap<String, FileInfo>) {
    let data = serde_json::to_string_pretty(metadata).expect("Error serializing metadata.");
    // Ensure the parent directory exists
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("Error creating metadata directory.");
    }
    let mut file = fs::File::create(path).expect("Error creating metadata file.");
    file.write_all(data.as_bytes())
        .expect("Error writing metadata file.");
}
