use std::io;
fn main() {
    let mut meta = String::new();
    io::stdin().read_line(&mut meta).expect("input error");
    let mut splitted = meta.split_whitespace(); 
    // iterator 
    let trial : u64 = splitted.next().expect("parsing error").parse::<u64>().expect("Parsing error");
    let criteria : f64 = splitted.next().expect("Parsing error").parse::<f64>().expect("Parsing error");
    let mut credit_sum : f64 = 0.0;
    let mut creditXgrade : f64 = 0.0;
    for i in 0..trial-1 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("input error");
        let mut splitted = line.split_whitespace();
        let mut credit : f64 = splitted.next().expect("Parsing error").parse::<f64>().expect("Parsing error");
        let mut grade = splitted.next().expect("Parsing error");
        credit_sum += credit;
        match grade {
            "A+" => creditXgrade += 4.5 * credit,
            "A0" => creditXgrade += 4.0 * credit,
            "B+" => creditXgrade += 3.5 * credit,
            "B0" => creditXgrade += 3.0 * credit,
            "C+" => creditXgrade += 2.5 * credit,
            "C0" => creditXgrade += 2.0 * credit,
            "D+" => creditXgrade += 1.5 * credit,
            "D0" => creditXgrade += 1.0 * credit,
            "F" => creditXgrade += 0.0,
            &_ => println!("Error"),
        }
    }
    let mut last = String::new();
    io::stdin().read_line(&mut last).expect("Input error");
    let target_credit = last.trim().parse::<f64>().expect("Parsing error");
    credit_sum += target_credit;
    let need = ((criteria * credit_sum) - creditXgrade) / target_credit;
    if need > 4.5 {
        println!("impossible");
    }
    else if need >4.0 {
        let more = (target_credit * 4.5 + creditXgrade) / credit_sum;
        let truncated = (more * 100.0).floor() / 100.0;
        if truncated == criteria {
            println!("impossible");
        }
        else {
            println!("A+");
        }
    }
    else if need >3.5 {
        println!("A0");
    }
    else if need >3.0 {
        println!("B+");
    }
    else if need >2.5 {
        println!("B0");
    }
    else if need >2.0 {
        println!("C+");
    }
    else if need >1.5 {
        println!("C0");
    }
    else if need >1.0 {
        println!("D+");
    }
    else if need >0.0 {
        println!("D0");
    }
    else {
        println!("F");
    }
}
