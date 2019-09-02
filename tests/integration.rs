use std::convert::TryInto;
use vec2dim::Vec2d;
use vec2dim::WrappingVec2d;

type DataType = i32;
fn initializer(row: usize, col: usize) -> DataType {
    (row as i32) * (col as i32)
}

#[test]
fn test_init_fn() {
    let v = Vec2d::from_fn(3, 4, &initializer);
    for row in 0..v.count_rows() {
        for col in 0..v.count_cols() {
            assert_eq!(v[row][col], initializer(row, col));
        }
    }
}

#[test]
fn test_init_closure() {
    let closure = &|row, col| -> DataType { (row as DataType) * (col as DataType) };
    let v = Vec2d::from_fn(3, 4, closure);
    for row in 0..v.count_rows() {
        for col in 0..v.count_cols() {
            assert_eq!(v[row][col], closure(row, col));
        }
    }
}

#[test]
fn test_wrapping() {
    let w = WrappingVec2d::from_vec2d(Vec2d::from_fn(5,2,&initializer));
    for wrow in 0..w.count_rows() {
        for wcol in 0..w.count_cols() {
            assert_eq!(*w.index(wrow,wcol), initializer(wrow.try_into().unwrap(), wcol.try_into().unwrap()));
        }
    }
    for wrow in -5..20 {
        for wcol in -20..100 {
            let col: usize = w.get_col_index(wcol);
            let row: usize = w.get_row_index(wrow);
            assert_eq!(*w.index(wrow, wcol), w.array()[row][col]);
        }
    }     
}