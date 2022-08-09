use crate::sort::util;
pub fn quick_sort(arr: &mut Vec<usize>, start: usize, end: usize) {
    if start >= end {
        return;
    }
    let mid = partition(arr, start, end);
    if mid >= 1 {
        quick_sort(arr, start, mid - 1);
    }
    quick_sort(arr, mid + 1, end);
}
fn partition(arr: &mut Vec<usize>, low: usize, high: usize) -> usize {
    let mut i: usize = low.clone();
    let mut j: usize = high;
    let v: usize = arr[low];
    loop {
        while i < high && arr[i] <= v {
            i = i + 1;
        }
        while j > low && arr[j] >= v {
            j = j - 1;
        }
        if i >= j {
            break;
        }
        util::swap(arr, i, j);
    }
    util::swap(arr, j, low);
    return j;
}
