use std::path::PathBuf;

/// A location in a file.
///
/// The `Location` struct contains a starting and ending character index, along with the filename it belongs to.
#[derive(Clone, Debug)]
pub struct Location {
    start: usize,
    end: usize,
    filename: PathBuf,
}

impl Location {
    /// Constructs a new `Location` object.
    ///
    /// - `start`: the starting character index
    /// - `end`: the ending character index
    /// - `filename`: the filename
    pub fn new(start: usize, end: usize, filename: &PathBuf) -> Self {
        Self {
            start,
            end,
            filename: filename.to_owned(),
        }
    }

    /// Returns the starting character index of the `Location`.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the ending character index of the `Location`.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the filename of the `Location`.
    pub fn filename(&self) -> &PathBuf {
        &self.filename
    }
}
