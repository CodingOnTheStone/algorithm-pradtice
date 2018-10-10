fn main() {
    println!("Hello, Bubble sort!!!");
    // TODO : change read from file
    let mut target_array = [5, 2, 1, 3, 4];

    println!("start - {:?}", target_array);

    bubble_sort(&mut target_array);

    println!("end - {:?}", target_array);

}

// TODO : change document format
// TODO : unit test
// do bubble sort
fn bubble_sort(target_array : &mut[i32]) {
    let mut is_set = false;
    let target_array_len = target_array.len();
    
    if target_array_len > 1 { // if less then 2, we do't need to sort array.
        for i in 0..target_array_len-1 { // from 0 to target_array_len-2 ( 0..3 -> 0, 1, 2)
            if target_array[i] > target_array[i+1] {
                // swap value
                let buffer = target_array[i];
                target_array[i] = target_array[i+1];
                target_array[i+1] = buffer;
                is_set = true;
                // print result
                println!("sorting - {:?}", target_array);
            }
        }
    }
    
    // do until nothing is set.
    if is_set {
        bubble_sort(target_array);
    } 
}
