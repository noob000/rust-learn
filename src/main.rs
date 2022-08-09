pub mod sort;
use crate::sort::quick_sort;
fn main() {
    let mut arr: Vec<usize> = vec![5, 4, 3, 2, 1, 0, 0, 1, 2];
    let length = arr.len();
    quick_sort::quick_sort(&mut arr, 0, length - 1);
    println!("arr is {:?}", arr);
}
// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn quick_sort(arr: Vec<usize>, start: usize, end: usize) {}
