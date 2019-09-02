use crate::Vec2d;
use std::convert::TryFrom;

/// Adds methods to access elements through wrapping indices, such as `[0][-1]` for the last
/// element in the first row.
pub struct WrappingVec2d<T> {
    data: Vec2d<T>,
}

#[allow(non_camel_case_types)]
pub type wsize = isize;

fn get_index(mut val: wsize, num: wsize) -> usize {
    while val < 0 { val += num; }
    while val >= num { val -= num; }
    let conversion = usize::try_from(val);
    if conversion.is_err() { panic!("Unable to get array index"); }
    conversion.unwrap()
}

impl <T> WrappingVec2d<T> {
    pub fn mut_array(&mut self) -> &mut Vec2d<T> {
        &mut self.data
    }
    pub fn array(&self) -> &Vec2d<T> {
        &self.data
    }
    pub fn count_cols(&self) -> wsize {
        let cols = self.array().count_cols();
        wsize::try_from(cols).unwrap()
    }
    pub fn count_rows(&self) -> wsize {
        let rows = self.array().count_rows();
        wsize::try_from(rows).unwrap()
    }
    pub fn get_col_index(&self, col: wsize) -> usize {
        // converts negative indices to positive
        let conversion =  wsize::try_from(self.data.count_cols());
        if conversion.is_err() { panic!("Too many columns in array to convert to wsize"); }
        let n_cols = conversion.unwrap();
        get_index(col, n_cols)
    }
    pub fn get_row_index(&self, row: wsize) -> usize {
        // converts negative indices to positive
        let conversion =  wsize::try_from(self.data.count_rows());
        if conversion.is_err() { panic!("Too many rows in array to convert to wsize"); }
        let n_rows = conversion.unwrap();
        get_index(row, n_rows)
    }
    pub fn index<'a>(&'a self, row: wsize, col: wsize) -> &'a T {
        let row: usize = self.get_row_index(row);
        let col: usize = self.get_col_index(col);
        &self.array()[row][col]
    }
    pub fn index_mut<'a>(&'a mut self, row: wsize, col: wsize) -> &'a mut T {
        let row: usize = self.get_row_index(row);
        let col: usize = self.get_col_index(col);
        &mut self.mut_array()[row][col]        
    }
}

impl <T: Copy> WrappingVec2d<T> {
    pub fn from(width: usize, data: &[T]) -> WrappingVec2d<T> {
        WrappingVec2d {
            data: Vec2d::from(width, data),
        }
    }
    pub fn from_vec2d(data: Vec2d<T>) -> WrappingVec2d<T> {
        WrappingVec2d {
            data: data,
        }
    }
}

mod tests {
    use super::*;
    type DataType = i32;

    #[test]
    fn check_array_access() {
        let data: [DataType;12] = [1,2,3,4,5,6,7,8,9,10,11,12];
        let v: Vec2d<DataType> = Vec2d::from(3, &data);
        let mut w: WrappingVec2d<DataType> = WrappingVec2d::from_vec2d(v);
        w.mut_array()[3][2] = 99;
        assert_eq!(w.mut_array()[3][2], 99);
    }

    #[test]
    fn check_index() {
        let data: [DataType;12] = [1,2,3,4,5,6,7,8,9,10,11,12];
        let v: Vec2d<DataType> = Vec2d::from(3, &data);
        let mut w: WrappingVec2d<DataType> = WrappingVec2d::from_vec2d(v);
        *w.index_mut(3,2) = 99;
        assert_eq!(*w.index(3,2), 99);
        assert_eq!(w.array()[3][2], *w.index(3,2));
    }
}