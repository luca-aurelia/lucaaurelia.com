use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct File {
    /// The path to the vault. For example, `/Users/photon-garden/library-of-babel`
    pub vault_path: String,

    /// For example, /Users/photon-garden/library-of-babel/people/richard feynman.md
    pub absolute_path: PathBuf,

    /// For example, `richard feynman.md`
    pub file_name: String,
    /// For example, `richard feynman`
    pub file_name_without_extension: String,

    /// Includes file extension. For example, `people/richard feynman.md`
    pub path_from_vault_root: String,
    /// For example, `people/richard feynman`
    pub path_from_vault_root_without_extension: String,

    /// Richard Feynman was an American physicist...
    pub contents: Contents,

    pub created_at: SystemTime,
}

pub fn files_in_vault(vault_path: String) -> impl Iterator<Item = File> {
    WalkDir::new(&vault_path)
        .into_iter()
        // We copy public files from my Obsidian vault into the built assets folder
        // so they're available at https://photon.garden. But we don't want to
        // treat them as part of the vault since they're duplicates.
        // We also omit anything in target/ or node_modules/ since those are
        // build artifacts and dependencies.
        .filter_entry(|entry| {
            let path = entry.path().to_string_lossy();
            !is_hidden(entry)
                && !is_in_built_assets_folder(entry)
                && !path.contains("target/")
                && !path.contains("node_modules/")
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(move |entry| File::from_dir_entry(&vault_path, &entry))
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_in_built_assets_folder(entry: &DirEntry) -> bool {
    let built_assets_dir = paths::built_assets_dir();
    entry.path().starts_with(built_assets_dir)
}

impl File {
    pub fn create(vault_path: &str, absolute_file_path: PathBuf, contents: String) -> File {
        File::write_file_including_intermediate_folders(&absolute_file_path, &contents)
            .expect("Error saving file to disk.");

        let created_at = fs::metadata(&absolute_file_path)
            .expect("Couldn't get file metadata.")
            .created()
            .expect("Couldn't get file creation time.");

        File::new(
            vault_path,
            absolute_file_path,
            GetContents::FromMarkdown(contents),
            created_at,
        )
    }

    fn write_file_including_intermediate_folders(
        path: &PathBuf,
        contents: &String,
    ) -> io::Result<()> {
        let path_to_parent_folder = path.parent().expect("Couldn't get parent from path.");
        fs::create_dir_all(path_to_parent_folder)?;
        fs::write(path, contents)
    }

    fn from_dir_entry(vault_path: &str, entry: &DirEntry) -> File {
        let absolute_path = entry.path().to_path_buf();

        let created_at = entry
            .metadata()
            .expect("Error reading file metadata.")
            .created()
            .expect("Couldn't get created_at timestamp for file.");

        File::new(
            vault_path,
            absolute_path,
            GetContents::FromFileSystem,
            created_at,
        )
    }

    pub fn from_path(vault_path: &str, absolute_path: impl Into<PathBuf>) -> File {
        let absolute_path = absolute_path.into();

        let created_at = fs::File::open(&absolute_path)
            .expect("Couldn't open file.")
            .metadata()
            .expect("Couldn't get metadata.")
            .created()
            .expect("Couldn't get created time.");

        File::new(
            vault_path,
            absolute_path,
            GetContents::FromFileSystem,
            created_at,
        )
    }

    pub fn new(
        vault_path: &str,
        absolute_path: PathBuf,
        how_to_get_contents: GetContents,
        created_at: SystemTime,
    ) -> File {
        let file_name_without_extension = absolute_path
            .file_stem()
            .expect("Error getting file file name without extension.")
            .to_str()
            .expect("Error converting file name without extension to string.")
            .to_owned();

        let extension = absolute_path
            .extension()
            .unwrap_or(std::ffi::OsStr::new("")) // If there's no extension, just use an empty string.
            .to_str()
            .expect("Error converting file extension to string.")
            .parse::<FileExtension>()
            .expect("Error parsing file extension.");

        let path_from_vault_root = {
            let path_from_vault_root = absolute_path
                .to_str()
                .expect("Error converting path to unicode.")
                .replace(vault_path, "");

            if path_from_vault_root.starts_with('/') {
                path_from_vault_root
                    .strip_prefix('/')
                    .expect("Error stripping prefix.")
                    .to_owned()
            } else {
                path_from_vault_root
            }
        };

        let path_from_vault_root_without_extension = path_from_vault_root
            .strip_suffix(&format!(".{}", extension))
            .unwrap_or(&path_from_vault_root) // strip_suffix returns None if the string doesn't end with the suffix. In that case, just use the original string.
            .to_string();

        let contents = match how_to_get_contents {
            GetContents::PassedInDirectly(contents) => contents,
            GetContents::FromFileSystem => Contents::from_extension(&extension, &absolute_path),
            GetContents::FromMarkdown(text) => Contents::Markdown { text },
        };

        let file_name = absolute_path
            .file_name()
            .expect("Error getting file name.")
            .to_str()
            .expect("Error converting file name to string.")
            .to_owned();

        File {
            vault_path: vault_path.to_string(),
            file_name_without_extension,
            file_name,
            path_from_vault_root,
            path_from_vault_root_without_extension,
            absolute_path,
            created_at,
            contents,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self.contents, Contents::Image {})
    }

    pub fn move_file(&mut self, new_path_from_vault_root: &str) {
        let new_absolute_path = Path::new(&self.vault_path).join(new_path_from_vault_root);

        fs::rename(&self.absolute_path, &new_absolute_path).expect("Error moving file.");

        let new_file = File::new(
            &self.vault_path,
            new_absolute_path,
            GetContents::PassedInDirectly(self.contents.clone()),
            self.created_at,
        );

        *self = new_file;
    }

    pub fn browser_src(&self) -> String {
        let path_from_vault_root = PathBuf::from_str(&self.path_from_vault_root).unwrap();
        paths::asset_url_path(&path_from_vault_root)
            .to_string_lossy()
            .to_string()
    }

    pub fn obsidian_src(&self) -> String {
        // This will only work on desktop. Still need to figure out how to
        // get it to work on mobile.
        let absolute_path = &self.absolute_path.to_string_lossy();
        format!("file://{absolute_path}")
    }
}

pub enum GetContents {
    FromFileSystem,
    PassedInDirectly(Contents),
    FromMarkdown(String),
}

#[derive(Debug, Clone)]
pub enum Contents {
    Markdown { text: String },
    Image {},
    Audio {},
    Video {},
    Pdf {},
    Unknown {},
}

impl Contents {
    fn from_extension(extension: &FileExtension, absolute_path: &Path) -> Contents {
        match extension.content_type() {
            ContentType::Markdown => {
                let text = fs::read_to_string(absolute_path)
                    .expect("Error reading markdown file contents.");
                Contents::Markdown { text }
            }

            ContentType::Image => Contents::Image {},
            ContentType::Audio => Contents::Audio {},
            ContentType::Video => Contents::Video {},
            ContentType::Pdf => Contents::Pdf {},
            ContentType::Unknown => Contents::Unknown {},
        }
    }
}

enum FileExtension {
    // Images
    Png,
    Jpg,
    Jpeg,
    Gif,
    Bmp,
    Svg,

    // Audio
    Mp3,
    Wav,
    M4a,
    Ogg,
    ThreeGp,
    Flac,

    // Video
    Mp4,
    Webm,
    Ogv,
    Mov,
    Mkv,

    // Text
    Md,
    Txt,

    // Code
    Html,
    Css,
    Js,
    Json,
    Xml,
    Yaml,
    Toml,

    // Other
    Pdf,

    Unknown,
}

impl FileExtension {
    fn content_type(&self) -> ContentType {
        match self {
            FileExtension::Png
            | FileExtension::Jpg
            | FileExtension::Jpeg
            | FileExtension::Gif
            | FileExtension::Bmp
            | FileExtension::Svg => ContentType::Image,

            FileExtension::Mp3
            | FileExtension::Wav
            | FileExtension::M4a
            | FileExtension::Ogg
            | FileExtension::ThreeGp
            | FileExtension::Flac => ContentType::Audio,

            FileExtension::Mp4
            | FileExtension::Webm
            | FileExtension::Ogv
            | FileExtension::Mov
            | FileExtension::Mkv => ContentType::Video,

            FileExtension::Md | FileExtension::Txt => ContentType::Markdown,

            FileExtension::Html
            | FileExtension::Css
            | FileExtension::Js
            | FileExtension::Json
            | FileExtension::Xml
            | FileExtension::Yaml
            | FileExtension::Toml => ContentType::Unknown,

            FileExtension::Pdf => ContentType::Pdf,

            FileExtension::Unknown => ContentType::Unknown,
        }
    }
}

impl std::str::FromStr for FileExtension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(FileExtension::Png),
            "jpg" => Ok(FileExtension::Jpg),
            "jpeg" => Ok(FileExtension::Jpeg),
            "gif" => Ok(FileExtension::Gif),
            "bmp" => Ok(FileExtension::Bmp),
            "svg" => Ok(FileExtension::Svg),

            "mp3" => Ok(FileExtension::Mp3),
            "wav" => Ok(FileExtension::Wav),
            "m4a" => Ok(FileExtension::M4a),
            "ogg" => Ok(FileExtension::Ogg),
            "3gp" => Ok(FileExtension::ThreeGp),
            "flac" => Ok(FileExtension::Flac),

            "mp4" => Ok(FileExtension::Mp4),
            "webm" => Ok(FileExtension::Webm),
            "ogv" => Ok(FileExtension::Ogv),
            "mov" => Ok(FileExtension::Mov),
            "mkv" => Ok(FileExtension::Mkv),

            "md" => Ok(FileExtension::Md),
            "txt" => Ok(FileExtension::Txt),

            "html" => Ok(FileExtension::Html),
            "css" => Ok(FileExtension::Css),
            "js" => Ok(FileExtension::Js),
            "json" => Ok(FileExtension::Json),
            "xml" => Ok(FileExtension::Xml),
            "yaml" => Ok(FileExtension::Yaml),
            "toml" => Ok(FileExtension::Toml),

            "pdf" => Ok(FileExtension::Pdf),

            _ => Ok(FileExtension::Unknown),
        }
    }
}

impl std::fmt::Display for FileExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extension = match self {
            FileExtension::Png => "png",
            FileExtension::Jpg => "jpg",
            FileExtension::Jpeg => "jpeg",
            FileExtension::Gif => "gif",
            FileExtension::Bmp => "bmp",
            FileExtension::Svg => "svg",

            FileExtension::Mp3 => "mp3",
            FileExtension::Wav => "wav",
            FileExtension::M4a => "m4a",
            FileExtension::Ogg => "ogg",
            FileExtension::ThreeGp => "3gp",
            FileExtension::Flac => "flac",

            FileExtension::Mp4 => "mp4",
            FileExtension::Webm => "webm",
            FileExtension::Ogv => "ogv",
            FileExtension::Mov => "mov",
            FileExtension::Mkv => "mkv",

            FileExtension::Md => "md",
            FileExtension::Txt => "txt",

            FileExtension::Html => "html",
            FileExtension::Css => "css",
            FileExtension::Js => "js",
            FileExtension::Json => "json",
            FileExtension::Xml => "xml",
            FileExtension::Yaml => "yaml",
            FileExtension::Toml => "toml",

            FileExtension::Pdf => "pdf",

            FileExtension::Unknown => "unknown",
        };

        write!(f, "{}", extension)
    }
}

enum ContentType {
    Markdown,
    Image,
    Audio,
    Video,
    Pdf,
    Unknown,
}
