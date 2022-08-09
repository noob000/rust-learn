pub fn swap(arr: &mut Vec<usize>, i: usize, j: usize) {
    if i >= arr.len() || j >= arr.len() {
        panic!("positionIndex exceed length");
    }
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}