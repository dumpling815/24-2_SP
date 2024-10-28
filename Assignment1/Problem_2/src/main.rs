use std::io;
use std::collections::VecDeque;

fn NUM(x: i64, stack: &mut Vec<i64>) {
    stack.push(x);
}

fn POP(stack: &mut Vec<i64>) {
    stack.pop();
}

fn INV(stack: &mut Vec<i64>) {
    if let Some(top) = stack.pop() {
        stack.push(-top);
    }
}

fn DUP(stack: &mut Vec<i64>) {
    if let Some(&top) = stack.last() {
        stack.push(top);
    }
}

fn SWP(stack: &mut Vec<i64>) {
    if stack.len() >= 2 {
        let first = stack.pop().unwrap();
        let second = stack.pop().unwrap();
        stack.push(first);
        stack.push(second);
    }
}

fn ADD(stack: &mut Vec<i64>, output_queue: &mut VecDeque<String>) {
    if stack.len() >= 2 {
        let first = stack.pop().unwrap();
        let second = stack.pop().unwrap();
        if let Some(result) = first.checked_add(second) {
            stack.push(result);
        } else {
            output_queue.push_back("error".to_string());
        }
    } else {
        output_queue.push_back("error".to_string());
    }
}

fn SUB(stack: &mut Vec<i64>, output_queue: &mut VecDeque<String>) {
    if stack.len() >= 2 {
        let first = stack.pop().unwrap();
        let second = stack.pop().unwrap();
        if let Some(result) = second.checked_sub(first) {
            stack.push(result);
        } else {
            output_queue.push_back("error".to_string());
        }
    } else {
        output_queue.push_back("error".to_string());
    }
}

fn MUL(stack: &mut Vec<i64>, output_queue: &mut VecDeque<String>) {
    if stack.len() >= 2 {
        let first = stack.pop().unwrap();
        let second = stack.pop().unwrap();
        if let Some(result) = first.checked_mul(second) {
            stack.push(result);
        } else {
            output_queue.push_back("error".to_string());
        }
    } else {
        output_queue.push_back("error".to_string());
    }
}

fn DIV(stack: &mut Vec<i64>, output_queue: &mut VecDeque<String>) {
    if stack.len() >= 2 {
        let first = stack.pop().unwrap();
        if first == 0 {
            output_queue.push_back("error".to_string());
            return; // devide by zero
        }
        let second = stack.pop().unwrap();
        if let Some(result) = second.checked_div(first) {
            stack.push(result);
        } else {
            output_queue.push_back("error".to_string());
        }
    } else {
        output_queue.push_back("error".to_string());
    }
}

fn MOD(stack: &mut Vec<i64>, output_queue: &mut VecDeque<String>) {
    if stack.len() >= 2 {
        let first = stack.pop().unwrap();
        if first == 0 {
            output_queue.push_back("error".to_string());
            return; // divide by zeor
        }
        let second = stack.pop().unwrap();
        if let Some(result) = second.checked_rem(first) {
            stack.push(result);
        } else {
            output_queue.push_back("error".to_string());
        }
    } else {
        output_queue.push_back("error".to_string());
    }
}

fn main() {
    let mut stack: Vec<i64> = Vec::new();
    let mut cmd_queue: VecDeque<String> = VecDeque::new();
    let mut output_queue: VecDeque<String> = VecDeque::new();
    
    'outer: loop {
        loop {
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Input Error");
            let command = command.trim();
            if command.is_empty() {
                continue;
            }
            let mut splitted = command.split_whitespace();
            let cmd = splitted.next().expect("Split Error").to_string();

            if cmd == "NUM" {
                let num = splitted.next().expect("Expected number").to_string();
                cmd_queue.push_back(cmd);
                cmd_queue.push_back(num);
            } else if cmd == "END" {
                break;
            } else if cmd == "QUIT" {
                break 'outer;
            } else {
                cmd_queue.push_back(cmd);
            }
        }
        // get trial.(입력횟수)
        let mut input_num = String::new();
        io::stdin().read_line(&mut input_num).expect("Input Error");
        if input_num.trim().is_empty() {
            continue;
        }
        let input_num: usize = input_num.trim().parse().expect("Parsing Error");
        for _ in 0..input_num {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Input Error");
            if input.trim().is_empty() {
                continue;  // empty string!
            }
            match input.trim().parse::<i64>() {
                Ok(num) => stack.push(num),
                Err(_) => {
                    output_queue.push_back("error".to_string());
                    continue;
                }
            }
            let mut local_cmd_queue = cmd_queue.clone();
            while let Some(command) = local_cmd_queue.pop_front() {
                match command.as_str() {
                    "NUM" => {
                        match local_cmd_queue.pop_front().unwrap().parse::<i64>() {
                            Ok(num) => NUM(num, &mut stack),
                            Err(_) => {
                                output_queue.push_back("error".to_string());
                                break;
                            }
                        }
                    }
                    "POP" => POP(&mut stack),
                    "INV" => INV(&mut stack),
                    "DUP" => DUP(&mut stack),
                    "SWP" => SWP(&mut stack),
                    "ADD" => ADD(&mut stack, &mut output_queue),
                    "SUB" => SUB(&mut stack, &mut output_queue),
                    "MUL" => MUL(&mut stack, &mut output_queue),
                    "DIV" => DIV(&mut stack, &mut output_queue),
                    "MOD" => MOD(&mut stack, &mut output_queue),
                    _ => println!("Unknown command: {}", command),
                }
            }
            if stack.len()== 1 {
                output_queue.push_back(stack.pop().unwrap().to_string());
                stack.clear();
            } else {
                output_queue.push_back("error".to_string());
                stack.clear();
            }
        }
    }
    while let Some(result) = output_queue.pop_front() {
        println!("{}", result);
    }
}
