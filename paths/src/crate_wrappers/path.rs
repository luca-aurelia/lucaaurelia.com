use camino::Utf8PathBuf;
use std::path::PathBuf;

pub trait PathBufExtension {
    fn into_utf8_path_buf(self) -> Utf8PathBuf;
}

impl PathBufExtension for PathBuf {
    fn into_utf8_path_buf(self) -> Utf8PathBuf {
        self.try_into().expect("PathBuf is not valid UTF-8")
    }
}
