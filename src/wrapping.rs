use crate::Vec2d;
use std::convert::TryFrom;
use std::ops::Index;

/// Adds methods to access elements through wrapping indices, such as `[0][-1]` for the last
/// element in the first row.
pub struct WrappingVec2d<T> {
    data: Vec2d<T>,
}

struct WrappingRow<'a, T> {
    data: &'a [T],
}

fn get_index(mut val: isize, num: isize) -> usize {
    while val < 0 { val += num; }
    while val >= num { val -= num; }
    let conversion = usize::try_from(val);
    if conversion.is_err() { panic!("Unable to get array index"); }
    conversion.unwrap()
}

impl<'a, T> WrappingRow<'a, T> {
    fn get_col_index(&self, mut col: isize) -> usize {
        let conversion =  isize::try_from(self.data.len());
        if conversion.is_err() { panic!("Too many columns in array to convert to isize"); }
        let n_cols = conversion.unwrap();
        get_index(col, n_cols)
    }
}

impl<'a, T> Index<isize> for WrappingRow<'a, T> {
    type Output = &'a T;

    fn index(&self, index: isize) -> &Self::Output {
        let col = self.get_col_index(index);
        self.data[col]
    }
}

impl <T> WrappingVec2d<T> {
    pub fn mut_array(&mut self) -> &mut Vec2d<T> {
        &mut self.data
    }
    pub fn array(&self) -> &Vec2d<T> {
        &self.data
    }
    fn get_row_index(&self, mut row: isize) -> usize {
        // converts negative indices to positive
        let conversion =  isize::try_from(self.data.count_rows());
        if conversion.is_err() { panic!("Too many rows in array to convert to isize"); }
        let n_rows = conversion.unwrap();
        get_index(row, n_rows)
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
            data,
        }
    }
}

impl<'a, T> std::ops::Index<isize> for WrappingVec2d<T> {
    type Output = WrappingRow<'a, T>;
    fn index(&'a self, row: isize) -> Self::Output {
        let row = self.get_row_index(row);
        WrappingRow {
            data: &self.data[row],
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
}