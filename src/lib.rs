
/// A two-dimensional array. Unlike a standard vector, `Vec2d` must maintain a constant
/// number of elements equal to its number of rows times its number of columns.
pub struct Vec2d<T> {
    data: Vec<T>,
    width: usize,
}

impl<T: Default> Vec2d<T> {
    /// Creates a new `Vec2d<T>` and initializes all the values to `T::default()`.
    pub fn new_with_default(rows: usize, cols: usize) -> Vec2d<T> {
        Vec2d::new_empty(rows, cols).initialize_to_default()
    }

    /// Adds a new row to the vector and sets all elements of that row to `T::default()`.
    pub fn add_row_of_default(&mut self) {
        // pushes a number of elements to data equal to the width
        // width does not change
        let size = self.count() + self.width;
        while self.data.len() < size {
            self.data.push(T::default());
        }
    }

    // private methods
    fn initialize_to_default(mut self) -> Vec2d<T> {
        let size = self.data.capacity();
        while self.data.len() < size {
            self.data.push(T::default());
        }
        self
    }
}

impl<T: Copy> Vec2d<T> {
    /// Creates a new `Vec2d<T>` and initializes all the values to a copy of `val`.
    pub fn new_with_value(rows: usize, cols: usize, val:T) -> Vec2d<T> {
        Vec2d::new_empty(rows, cols).initialize_to_value(val)
    }


    // private
    fn initialize_to_value(mut self, val: T) -> Vec2d<T> {
        let size = self.data.capacity();
        while self.data.len() < size {
            self.data.push(val);
        }
        self
    }
}

impl<T> Vec2d<T> {
    /// Creates a new `Vec2d<T>` with no rows, columns, or elements.
    pub fn new() -> Vec2d<T> {
        // a vector without any data has width 0
        Vec2d {
            data: Vec::new(),
            width: 0,
        }
    }

    /// Returns the number of elements in the array.
    /// Equal to the number of rows times the number of columns.
    pub fn count(&self) -> usize {
        self.width * self.count_rows()
    }

    /// Returns the number of columns of the array.
    pub fn count_cols(&self) -> usize {
        self.width
    }

    /// Returns the number of rows of the array.
    pub fn count_rows(&self) -> usize {
        if self.width == 0 { 0 } else { self.data.len() / self.width }
    }

    /// Returns the dimensions of the array as a tuple of (row, col).
    pub fn size(&self) -> (usize, usize) {
        (self.count_rows(), self.count_cols())
    }

    fn new_empty(rows: usize, cols: usize) -> Vec2d<T> {
        Vec2d {
            data: Vec::with_capacity(rows*cols),
            width: cols,
        }
    }
}

impl<T> std::ops::Index<usize> for Vec2d<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let start = row * self.width;
        let end = start + self.width;
        &self.data[start..end]
    }
}

impl<T> std::ops::IndexMut<usize> for Vec2d<T> {
    fn index_mut<'a>(&'a mut self, row: usize) -> &'a mut Self::Output {
        let start = row * self.width;
        let end = start + self.width;
        &mut self.data[start..end]
    }
}
