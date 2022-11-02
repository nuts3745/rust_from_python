use rand::seq::SliceRandom;

fn main() {
    const BINGO_NUMBER: usize = 75;
    let mut nums = [0; BINGO_NUMBER];
    for i in 1..=BINGO_NUMBER {
        nums[i - 1] = i;
    }

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!("  *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}
