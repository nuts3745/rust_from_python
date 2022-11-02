fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", nums);

    let mut nums2 = Vec::new();
    nums2.push(1);
    nums2.push(2);
    nums2.push(3);
    println!("{:?}", nums2);

    let a_vec: Vec<u32> = vec![100, 200, 300];
    for i in a_vec {
        println!("{}", i);
    }

    let s_vec: Vec<&str> = vec!["犬", "猫", "鶏"];
    for i in s_vec {
        println!("{}", i);
    }
}
