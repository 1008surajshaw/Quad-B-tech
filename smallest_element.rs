fn kth_smallest_element(arr: &[i32],k:usize)->Option<i32> {
    if k> arr.len() {
        return None;
    }
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k-1])
}
fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let k = 3;
    match kth_smallest_element(&arr, k) {
        Some(element) => println!("The {}th smallest element is: {}", k, element),
        None => println!("The array does not have {} elements", k),
    }
}