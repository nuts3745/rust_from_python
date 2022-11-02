use std::fs;

fn main() {
    let text1 = fs::read_to_string("hoge.txt").unwrap_or("失敗した text1".to_string());
    let text2 = match fs::read_to_string("hoge.txt") {
        Ok(text) => text,
        Err(_) => "失敗した text2".to_string(),
    };
    if let Ok(s) = fs::read_to_string("hoge.txt") {
        println!("{}", s);
    } else {
        println!("失敗した text3")
    }
    println!("{} {}", text1, text2);
}
