// use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
// use std::fs;
// use std::path::Path;
// use walkdir::WalkDir;
// use sha2::{Sha256, Digest};
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct FileInfo {
//     modified: std::time::SystemTime,
//     hash: String,
// }

// #[derive(Debug)]
// enum FileChange {
//     Added,
//     Modified,
//     Deleted,
// }

// pub fn has_any_file_changed<P: AsRef<Path>>(dir: P, metadata_file: P) -> bool {
//     list_changed_files(dir, metadata_file).len() > 0
// }

// pub fn list_changed_files<P: AsRef<Path>>(dir: P, metadata_file: P) -> Vec<(String, FileChange)> {
//     let mut rt = Runtime::new().unwrap();  // Create a new runtime for blocking within this synchronous function
//     rt.block_on(async {
//         let previous_metadata = load_previous_metadata(&metadata_file);
//         let current_metadata = scan_directory(dir).await;

//         let mut changes = Vec::new();

//         // Check for modified or new files
//         for (path, current_info) in &current_metadata {
//             match previous_metadata.get(path) {
//                 Some(previous_info) => {
//                     if previous_info.hash != current_info.hash {
//                         changes.push((path.clone(), FileChange::Modified));
//                     }
//                 },
//                 None => changes.push((path.clone(), FileChange::Added)), // New file
//             }
//         }

//         // Check for deleted files
//         for path in previous_metadata.keys() {
//             if !current_metadata.contains_key(path) {
//                 changes.push((path.clone(), FileChange::Deleted));
//             }
//         }

//         save_metadata(metadata_file, &current_metadata);

//         changes
//     })
// }

// async fn calculate_hash<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
//     let file = File::open(path).await?;
//     let mut reader = tokio::io::BufReader::new(file);
//     let mut hasher = Sha256::new();
//     let mut buffer = vec![0; 1024];

//     loop {
//         let n = reader.read(&mut buffer).await?;
//         if n == 0 {
//             break;
//         }
//         hasher.update(&buffer[..n]);
//     }

//     Ok(format!("{:x}", hasher.finalize()))
// }

// async fn scan_directory<P: AsRef<Path>>(dir: P) -> HashMap<String, FileInfo> {
//     let mut file_info_map = HashMap::new();

//     for entry in WalkDir::new(dir) {
//         let entry = entry.unwrap();
//         let path = entry.path();

//         if path.is_file() {
//             let metadata = fs::metadata(path).unwrap();
//             let modified = metadata.modified().unwrap();
//             let hash = calculate_hash(path).await.unwrap();

//             file_info_map.insert(path.to_string_lossy().to_string(), FileInfo { modified, hash });
//         }
//     }

//     file_info_map
// }

// fn load_previous_metadata<P: AsRef<Path>>(path: P) -> HashMap<String, FileInfo> {
//     if let Ok(data) = fs::read_to_string(path) {
//         serde_json::from_str(&data).unwrap_or_default()
//     } else {
//         HashMap::new()
//     }
// }

// fn save_metadata<P: AsRef<Path>>(path: P, metadata: &HashMap<String, FileInfo>) {
//     let data = serde_json::to_string_pretty(metadata).unwrap();
//     let mut file = fs::File::create(path).unwrap();
//     file.write_all(data.as_bytes()).unwrap();
// }
