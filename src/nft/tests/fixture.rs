use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

pub struct Fixture {
    pub path: PathBuf,
    source: PathBuf,
    _tempdir: TempDir,
}

impl Fixture {
    pub fn blank(fixture_filename: &str) -> Self {
        // First, figure out the right file in `tests/fixtures/`:
        let root_dir = &std::env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests/fixtures");
        source.push(&fixture_filename);

        // The "real" path of the file is going to be under a temporary directory:
        let tempdir = tempfile::tempdir().unwrap();
        let path = PathBuf::from(&tempdir.path());

        Fixture {
            _tempdir: tempdir,
            source,
            path,
        }
    }

    pub fn create_layers_dirs(fixture_filename: &str, layer_dirs: &[&str]) -> Self {
        let fixture = Fixture::blank(fixture_filename);

        for layer in layer_dirs.iter() {
            let layer_path = fixture.path.join(layer);
            std::fs::create_dir(&layer_path).expect("Create layer dir should work in test");
            for i in 0..10 {
                let image_path = layer_path.join(format!("image{}#{}.png", i, i));
                std::fs::copy(&fixture.source, image_path)
                    .expect("create png file should work in test");
            }
        }
        fixture
    }
}

impl Deref for Fixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.deref()
    }
}
