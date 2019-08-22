
use vec2dim::Vec2d;

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
