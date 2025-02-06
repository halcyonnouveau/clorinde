use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{config::StaticFile, error::PersistError};

/// In memory storage of file
/// Can only fail while persisting
pub struct Vfs {
    fs: BTreeMap<PathBuf, String>,
}

impl Vfs {
    pub fn empty() -> Self {
        Self {
            fs: BTreeMap::new(),
        }
    }

    pub(crate) fn rustfmt(path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();

        // Check if rustfmt is available
        if Command::new("rustfmt").arg("--version").output().is_err() {
            // rustfmt not installed - return true since this isn't a critical error
            return true;
        }

        Command::new("rustfmt")
            .args([
                "--edition",
                "2024",
                path.join("src/lib.rs").to_str().unwrap(),
            ])
            .status()
            .unwrap()
            .success()
    }

    /// Add a new file
    pub fn add(&mut self, path: impl Into<PathBuf>, content: proc_macro2::TokenStream) {
        let warning = "// This file was generated with `clorinde`. Do not modify.\n\n";

        let syntax_tree = syn::parse2(content).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);

        let file_content = format!("{}{}", warning, formatted);
        assert!(self.fs.insert(path.into(), file_content).is_none())
    }

    /// Add a new file from a string
    pub fn add_string(&mut self, path: impl Into<PathBuf>, content: impl Into<String>) {
        assert!(self.fs.insert(path.into(), content.into()).is_none())
    }

    pub fn persist(
        self,
        destination: impl AsRef<Path>,
        static_files: Vec<StaticFile>,
    ) -> Result<(), PersistError> {
        let destination = destination.as_ref();
        // First write in a temporary directory to prevent leaving the destination in a bad state
        let tmp = tempfile::tempdir().map_err(PersistError::wrap("tempfile"))?;

        // Write files to temp directory
        for (path, content) in self.fs {
            let path = tmp.path().join(path);
            let parent = path
                .parent()
                .expect("Must at least has 'destination' as parent");
            std::fs::create_dir_all(parent).ok(); // Might already exist
            std::fs::write(&path, content).map_err(PersistError::wrap(path))?;
        }

        // Copy static files to temp directory
        if !static_files.is_empty() {
            for file in static_files {
                let (path, hard_link) = match file {
                    StaticFile::Simple(path) => (path, false),
                    StaticFile::Detailed { path, hard_link } => (path, hard_link),
                };

                let src = Path::new(&path);
                if !src.exists() {
                    return Err(PersistError::wrap(&path)(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Static file not found",
                    )));
                }

                let file_name = src.file_name().ok_or_else(|| {
                    PersistError::wrap(&path)(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid file name",
                    ))
                })?;

                let dst = tmp.path().join(file_name);

                if hard_link {
                    std::fs::hard_link(src, dst).map_err(PersistError::wrap(&path))?;
                } else {
                    std::fs::copy(src, dst).map_err(PersistError::wrap(&path))?;
                }
            }
        }

        // Remove existing destination if it exists
        if destination.exists() {
            std::fs::remove_dir_all(destination).map_err(PersistError::wrap(destination))?;
        }

        // Format with rustfmt
        Vfs::rustfmt(tmp.path());

        // Create destination directory
        std::fs::create_dir_all(destination).map_err(PersistError::wrap(destination))?;

        // Copy directory contents recursively as a fallback for rename
        fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
            let src = src.as_ref();
            let dst = dst.as_ref();

            if src.is_dir() {
                for entry in std::fs::read_dir(src)? {
                    let entry = entry?;
                    let ty = entry.file_type()?;
                    let path = entry.path();

                    let dst_path = dst.join(path.file_name().expect("file name exists"));

                    if ty.is_dir() {
                        std::fs::create_dir_all(&dst_path)?;
                        copy_dir_recursive(&path, &dst_path)?;
                    } else if ty.is_file() {
                        std::fs::copy(&path, &dst_path)?;
                    }
                }
            }
            Ok(())
        }

        // Try rename first, fall back to copy if it fails
        match std::fs::rename(tmp.path(), destination) {
            Ok(_) => Ok(()), // Rename successful
            Err(e) if e.raw_os_error() == Some(18) => {
                // EXDEV error, fall back to copy
                copy_dir_recursive(tmp.path(), destination)
                    .map_err(PersistError::wrap(destination))?;
                Ok(())
            }
            Err(e) => Err(PersistError::wrap(destination)(e)),
        }
    }
}
