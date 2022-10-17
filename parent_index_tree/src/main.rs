fn main() {
    let arr : [i32; 7] = [1,5,5,2,2,-1,3];
    println!("Depth of the tree is {}", get_depth(&arr))
}

fn get_depth(arr: &[i32]) -> i32 {
    // cannot use a normal array as the length is to be taken from another variable
    let mut depth_arr : Vec<i32> = vec![-1; arr.len()];

    for i in 0..arr.len() {
        fill_depth(arr, i, &mut depth_arr)
    }

    return *depth_arr.iter().max().ok_or("This is unexpected").unwrap();

}

fn fill_depth(arr: &[i32], idx: usize, depth_arr: &mut Vec<i32>) {
    println!("Called this function for {}", arr[idx]);

    if depth_arr[idx] != -1 {
        return; // already computed
    }

    let parent_idx: i32 = arr[idx];

    if parent_idx == -1 {
        depth_arr[idx] = 0; // this is the root
    } else {
        if depth_arr[parent_idx as usize] == -1 {    // depth of parent is not known
            fill_depth(arr, parent_idx as usize, depth_arr);
        }
        depth_arr[idx] = depth_arr[parent_idx as usize] + 1;
    }
}
