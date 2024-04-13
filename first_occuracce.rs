fn first_occurrence(arr:&[i32],target:i32)->Option<unsize>{
    for(index,&num) in arr,iter().enumerate() {
        if num == target {
            return Some(index);

        } else if num >target  {
            break;
        }
    }
    None
}

fn main() {
    let arr = [1, 2, 3, 4, 4, 5, 6, 6, 7];
    let target = 4;
    
    if let Some(index) = first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}