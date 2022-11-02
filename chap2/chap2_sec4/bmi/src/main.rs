fn main() {
    let mut height_cm;
    loop {
        println!("身長(cm)は？: ");
        height_cm = input_f(0.0);
        if height_cm > 0.0 {
            break;
        };
        println!("正しい数値を入力してください。");
    }
    let mut weight;
    loop {
        println!("体重(kg)は？");
        weight = input_f(0.0);
        if weight > 0.0 {
            break;
        };
        println!("正しい数値を入力してください");
    }

    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    let standard_body_weight = height.powf(2.0) * 22.0;
    let difference_weight = standard_body_weight - weight;

    println!("BMI={:.1}", bmi);
    print!("あなたは ");
    if bmi < 18.5 {
        print!("低体重");
    } else if bmi < 25.0 {
        print!("普通体重");
    } else if bmi < 30.0 {
        print!("肥満1度");
    } else if bmi < 35.0 {
        print!("肥満2度");
    } else if bmi < 40.0 {
        print!("肥満3度");
    } else {
        print!("肥満4度");
    }
    println!(" です");
    println!("適正体重まであと {:.0} kg", difference_weight);
}

fn input_f(arg: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => arg,
    }
}

fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    s.trim_end().to_string()
}
