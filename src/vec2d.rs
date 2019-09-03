
/// A two-dimensional array. Unlike a standard vector, `Vec2d` must maintain a constant
/// number of elements equal to its number of rows times its number of columns.
#[derive (Clone)]
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
    /// If the array is empty, creates a row of one element.
    pub fn add_row_of_default(&mut self) {
        // pushes a number of elements to data equal to the width
        // width does not change unless array was empty
        if self.width == 0 { self.width += 1; }
        let size = self.count() + self.width;
        while self.data.len() < size {
            self.data.push(T::default());
        }
    }

    /// Adds a new column to the array and sets all elements of the column to `T::default()`.
    pub fn add_col_of_default(&mut self) {
        // inserts a default item at the position representing the end of each row
        // note that this position shifts as elements are added
        let new_width = self.width + 1;
        let mut idx = self.width;
        for _row in 0..self.count_rows() {
            self.data.insert(idx, T::default());
            idx += new_width;
        }
        self.width = new_width;
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

    /// Creates a new `Vec2d<T>` from an array slice.
    /// The slice must have a length that is divisible by `cols` in order to fill the new array.
    /// The array is filled left to right, top to bottom.
    pub fn from(width: usize, arr: &[T]) -> Vec2d<T> {
        let size = arr.len();
        assert_eq!(size % width, 0);
        let mut data: Vec<T> = Vec::with_capacity(size);
        for idx in 0..size {
            data.push(arr[idx]);
        }
        Vec2d {
            data,
            width,
        }
    }

    /// Inserts a column of data before the columns indicated by `index`.
    pub fn insert_col(&mut self, index: usize, data: &[T]) {
        let size: usize = data.len();
        // Data must have same size as array height
        if self.width == 0 {
            self.width = 1;
            for item in data {
                self.data.push(*item);
            }
        }
        else {
            assert_eq!(size, self.count_rows());
            let mut idx = index;
            self.width += 1;
            for row in 0..size {
                self.data.insert(idx, data[row]);
                idx += self.width;
            }
        }
    }

    /// Inserts a row of data before the row indicated by `index`. If `index` is `0`, calls
    /// `push_row(data)`.
    pub fn insert_row(&mut self, index: usize, data: &[T]) {
        // If array isn't empty, row length must match width
        // if it is empty, add the entire data vector as a new row
        // and set width to the length of the data
        if self.width > 0 {
            assert_eq!(self.width, data.len());
            // currently just pushes items one by one
            // TODO: implement this more efficiently
            let mut idx = self.width * index;
            for item in data {
                self.data.insert(idx, *item);
                idx += 1;
            }
        }
        else { self.push_row(data); }
    }

    /// Adds a new column of data to the right edge of the array. The length of the slice `data`
    /// must be exactly equal to the height of the array.
    pub fn push_col(&mut self, data: &[T]) {
        self.insert_col(self.count_cols(), data);
    }

    /// Adds a row of data at the bottom of the array. The length of the slice `data` must be
    /// exactly equal to the array width or the method will panic.
    pub fn push_row(&mut self, data: &[T]) {
        // If array isn't empty, row length must match width
        // if it is empty, add the entire data vector as a new row
        // and set width to the length of the data
        if self.width > 0 { assert_eq!(self.width, data.len()); }
        else { self.width = data.len(); }
        for item in data {
            self.data.push(*item);
        }
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

impl<T: PartialEq> Vec2d<T> {

    /// Checks if `val` is equivalent to any of the elements in the array.
    /// Returns `true` if there is a match, `false` otherwise.
    pub fn contains(&self, val: &T) -> bool {
        for idx in 0..self.count() {
            if *val == self.data[idx] { return true; }
        }
        false
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

    /// Creates a new `Vec2d<T>` and initializes its values from `initializer`,
    /// a passed-in function that takes the current cell index (row and column)
    /// and returns a `T`.
    pub fn from_fn(rows: usize, cols: usize, initializer: &dyn Fn(usize, usize) -> T) -> Vec2d<T> {
        Vec2d::new_empty(rows, cols).initialize_from_fn(initializer)
    }

    /// Returns a raw pointer to the underlying vector's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Returns a reference to the underlying vector.
    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
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

    // private
    fn new_empty(rows: usize, cols: usize) -> Vec2d<T> {
        Vec2d {
            data: Vec::with_capacity(rows*cols),
            width: cols,
        }
    }

    fn initialize_from_fn(mut self, initializer: &dyn Fn(usize, usize) -> T) -> Vec2d<T> {
        let size = self.data.capacity();
        for idx in 0..size {
            let row = idx / self.width;
            let col = idx - (row * self.width);
            self.data.push(initializer(row, col));
        }
        self
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
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        let end = start + self.width;
        &mut self.data[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_initializer() {
        let v = Vec2d::from_fn(3, 3, &|row, col| row + col);
        for idx in 0..v.count() {
            let row = idx / v.count_cols();
            let col = idx - (row * v.count_cols());
            assert_eq!(v[row][col], row+col);
        }
    }

    #[test]
    #[should_panic]
    fn panics_on_mismatched_arr_size() {
        let arr: [i32;4] = [1,2,3,4];
        let slc: &[i32] = &arr[0..3];
        let _v: Vec2d<i32> = Vec2d::from(2, slc);
        let arr: [i32;5] = [1,2,3,4,5];
        let _v = Vec2d::from(2, &arr);
    }

    #[test]
    fn build_from_data() {
        let data: [i32;12] = [1,2,3,4,5,6,7,8,9,10,11,12];
        for divs in 1..13 {
            if 12 % divs == 0 {
                let _v = Vec2d::from(divs, &data);
                for row in 0.._v.count_rows() {
                    for col in 0.._v.count_cols() {
                        let idx = row * _v.count_cols() + col;
                        assert_eq!(data[idx], _v[row][col]);
                    }
                }
            }
        }
    }

    #[test]
    fn build_from_defaults() {
        type DataType = i32;
        let mut v:Vec2d<DataType> = Vec2d::new();
        assert_eq!(v.count(), 0);
        v.add_row_of_default();
        assert_eq!(v.count_rows(), 1);
        v.add_row_of_default();
        v.add_col_of_default();
        assert_eq!(v.count(), 4);
        assert_eq!(v.count_rows(), 2);
        assert_eq!(v.count_cols(), 2);
    }

    #[test]
    fn test_data_pushing() {
        type DataType = i32;
        let mut v: Vec2d<DataType> = Vec2d::new();
        let data = [1,2,3,4,5,6,7,8,9,10,11,12];
        v.push_row(&data[0..1]);
        v.push_col(&data[1..2]);
        v.push_row(&data[2..4]);
        v.push_row(&data[4..6]);
        v.push_col(&data[6..9]);
        v.push_col(&data[9..12]);
        assert_eq!(v.count(), 12);
        assert_eq!(v.count_rows(), 3);
        assert_eq!(v.count_cols(), 4);
        let answers = [1,2,7,10,3,4,8,11,5,6,9,12];
        let mut idx: usize = 0;
        for row in 0..v.count_rows() {
            for col in 0..v.count_cols() {
                assert_eq!(v[row][col], answers[idx]);
                idx += 1;
            }
        }
    }

    #[test]
    fn test_data_insertion() {
        type DataType = i32;
        let mut v: Vec2d<DataType> = Vec2d::new();
        let data = [1,2,3,4,5,6,7,8,9,10,11,12];
        v.insert_row(0,&data[0..1]);
        v.insert_col(0,&data[1..2]);
        v.insert_row(1,&data[2..4]);
        v.insert_row(1, &data[4..6]);
        v.insert_col(1,&data[6..9]);
        v.insert_col(3,&data[9..12]);
        assert_eq!(v.count(), 12);
        assert_eq!(v.count_rows(), 3);
        assert_eq!(v.count_cols(), 4);
        let answers = [2,7,1,10,5,8,6,11,3,9,4,12];
        let mut idx: usize = 0;
        for row in 0..v.count_rows() {
            for col in 0..v.count_cols() {
                assert_eq!(v[row][col], answers[idx]);
                idx += 1;
            }
        }
    }
}
