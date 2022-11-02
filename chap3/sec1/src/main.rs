fn main() {
    {
        let s1 = String::from("聞かないで返事する人は愚か");
        let s3 = String::from("葡萄畑を美人は手がかかる");
        {
            let s2 = s1;
            println!("{}", s2);
        }
        println!("{}", s3);
    }
    let g1 = String::from("穏やかな心は体に良い");
    let g2 = g1;
    println!("{}", g2);
}
