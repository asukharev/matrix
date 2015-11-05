/// A size.
pub trait Size {
    /// Return the number of rows.
    fn rows(&self) -> usize;

    /// Return the number of columns.
    fn columns(&self) -> usize;

    /// Return the number of rows and columns.
    fn dimensions(&self) -> (usize, usize) {
        (self.rows(), self.columns())
    }
}

impl Size for (usize, usize) {
    fn rows(&self) -> usize {
        self.0
    }

    fn columns(&self) -> usize {
        self.1
    }
}
