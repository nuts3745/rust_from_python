use std::collections::HashMap;

const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main() {
    let mut c_map = HashMap::new();
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w] + 1);
    }
    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, c_map[k]);
    }

    let tuki = [
        "睦月",
        "如月",
        "弥生",
        "卯月",
        "皐月",
        "水無月",
        "文月",
        "葉月",
        "長月",
        "神無月",
        "霜月",
        "師走",
    ];
    let mut tuki_map: HashMap<&str, usize> = HashMap::new();
    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, i + 1);
    }
    println!("水無月 = {}月", tuki_map["水無月"]);
    println!("神無月 = {}月", tuki_map["神無月"]);
    match tuki_map.get("神在月") {
        None => println!("神在月は存在しない"),
        Some(v) => println!("神在月 = {}月", v),
    }
    println!("師走 = {}月", tuki_map["師走"]);
}
