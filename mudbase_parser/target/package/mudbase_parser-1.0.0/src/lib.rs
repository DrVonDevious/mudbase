pub mod prelude {
    use std::{path::Path, fs::File};

    /// Loads a file from the given path
    ///
    /// ### Arguments
    /// * `filepath` - The path to the desired file to be loaded
    pub fn load_file(filepath: &str) -> File {
        let path = Path::new(filepath);
        return File::open(path).expect("Error: failed to load file");
    }
}