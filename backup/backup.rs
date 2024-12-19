// This library is for Assignment #3, problem_1, "Oh My Shell"

// Interact module manages stdin, out with a oh-my-shell user.
pub mod Interact{
    use std::io::{self, Write};
    use std::path::Path; 
    use std::process::exit;
    use super::Subprocess;

    // get_instruction
    pub fn interact() -> Option<()>{
        print!(">>> ");
        io::stdout().flush().expect("Failed to flushing stdout buffer");
        let mut instruction = String::new();
        io::stdin().read_line(&mut instruction).expect("Fail to read instruction");
        let ins_vec: Vec<String> = instruction.trim().split_whitespace().map(|s| s.to_string()).collect();
        let ins_vec = preprocess_vector(ins_vec);

        let command_result = match ins_vec.get(0).map(String::as_str){
            Some("exit") => Some(terminating()), 
            Some(other) if supported(other) || ins_vec.contains(&"|".to_string()) => Subprocess::subprocessing(ins_vec),
            _ => {
                println!("Invalid command!");
                Some(())
            }
        };
        command_result
    }

    fn is_path(input: &str) -> bool{
        let path = Path::new(input);
        path.exists()
    }

    fn supported(command: &str) -> bool{
        let support_arr: [&str; 9] = ["ls","cat","pwd","echo","mkdir","grep","rmdir","touch","rm"];
        support_arr.contains(&command) || is_path(command)
    }

    fn preprocess_vector(input: Vec<String>) -> Vec<String>{
        let mut result = Vec::new();
        let mut temp = String::new();
        let mut is_quote = false;

        for element in input.iter(){
            if is_quote {
                temp.push_str(" ");
                temp.push_str(element);
                if element.ends_with('\"'){
                    result.push(temp.clone());
                    temp.clear();
                    is_quote = false;
                }
            }
            else{
                if element.starts_with('\"'){
                    is_quote = true;
                    temp.push_str(element);
                }
                else {
                    result.push(element.clone());
                }
            }
        }
        if is_quote{
            let mut remain: Vec<String> = temp.split_whitespace().map(|x| x.to_string()).collect();
            result.append(&mut remain);
        }
        result
    }

    fn terminating(){
        println!("Exit Oh My Shell. Good Bye!");
        exit(0);
    }
}


pub mod Subprocess { // Module using System Calls.
    use nix::unistd::{fork, execvp, ForkResult, dup2};
    use nix::sys::wait::{waitpid, WaitStatus};
    use nix::libc::{self,STDIN_FILENO,STDOUT_FILENO};
    use std::ffi::CString;
    use std::path::Path; 
    use std::fs::OpenOptions;
    use std::os::unix::io::AsRawFd;
    use std::process::exit;
    use std::fs;
    use std::env::temp_dir;

    fn is_path(input: &str) -> bool{
        let path = Path::new(input);
        path.exists()
    }

    fn preprocess_vector(input: Vec<String>) -> Vec<String>{ // to tokenizing the instruction +, contol the "symbol
        let mut result = Vec::new();
        let mut temp = String::new();
        let mut is_quote = false;

        for element in input.iter(){
            if is_quote {
                temp.push_str(" ");
                temp.push_str(element);
                if element.ends_with('\"'){
                    result.push(temp.clone());
                    temp.clear();
                    is_quote = false;
                }
            }
            else{
                if element.starts_with('\"'){
                    is_quote = true;
                    temp.push_str(element);
                }
                else {
                    result.push(element.clone());
                }
            }
        }
        if is_quote{
            let mut remain: Vec<String> = temp.split_whitespace().map(|x| x.to_string()).collect();
            result.append(&mut remain);
        }
        result
    }

    pub fn subprocessing(mut ins_vec: Vec<String>) -> Option<()>{
        let mut commands = Vec::new();
        let mut current_cmd = Vec::new();

        for token in ins_vec { // much deeper tokenizing by pipeline |
            if token == "|" {
                current_cmd = preprocess_vector(current_cmd);
                commands.push(current_cmd);
                current_cmd = Vec::new();
            }
            else{
                current_cmd.push(token);
            }
        }
        if !current_cmd.is_empty(){ // pushing last element
            current_cmd = preprocess_vector(current_cmd);
            commands.push(current_cmd);
        }

        let subprocess_trial = commands.len();
        //if subprocess_trial == 0{ // if there are no command
        //    return Some(());
        //}

        // Only one command in vector!
        if subprocess_trial == 1 {
            match unsafe { fork() } {
                Ok(ForkResult::Parent {child }) => {
                    waitpid(child, None).ok();
                }
                Ok(ForkResult::Child) => {
                    handle_redirection(commands[0].clone());
                    exit(1);
                }
                Err(err) => {
                    eprintln!("fork failed: {}", err);
                    return None;
                }
            }
            return Some(());
        }

        // Leveraging Named pipe
        let pid = std::process::id();
        let mut fifo_paths = Vec::new();
        for i in 0..(subprocess_trial-1) {
            let fifo_path = temp_dir().join(format!("ohmyshell_fifo_{}_{}", pid, i));
            let fifo_cstr = CString::new(fifo_path.as_os_str().as_encoded_bytes()).expect("CString error");
            unsafe {
                if libc::mkfifo(fifo_cstr.as_ptr(), 0o600) < 0 {
                    eprintln!("mkfifo failed");
                    // cleanup
                    for p in &fifo_paths {
                        let _ = fs::remove_file(p);
                    }
                    return None;
                }
            }
            fifo_paths.push(fifo_path);
        }

        let mut children = Vec::new();

        for i in 0..subprocess_trial {
            match unsafe{fork()} {
                Ok(ForkResult::Parent { child }) => {
                    children.push(child);
                }
                Ok(ForkResult::Child) => {
                    // 이전 FIFO에서 입력 설정
                    if i > 0 {
                        let prev_fifo = &fifo_paths[i-1];
                        let prev_fd = OpenOptions::new().read(true).open(prev_fifo)
                            .expect("Failed to open previous FIFO for read");
                        dup2(prev_fd.as_raw_fd(), STDIN_FILENO).expect("Failed to dup2 for STDIN");
                    }

                    // 다음 FIFO로 출력 설정
                    if i < subprocess_trial-1 {
                        let next_fifo = &fifo_paths[i];
                        let next_fd = OpenOptions::new().write(true).open(next_fifo)
                            .expect("Failed to open next FIFO for write");
                        dup2(next_fd.as_raw_fd(), STDOUT_FILENO).expect("Failed to dup2 for STDOUT");
                    }

                    handle_redirection(commands[i].clone());
                    exit(1);
                }
                Err(err) => {
                    eprintln!("fork failed: {}", err);
                    for p in &fifo_paths {
                        let _ = fs::remove_file(p);
                    }
                    return None;
                }
            }
        }


        for child in children { // Waiting for entire child
            if let Ok(status) = waitpid(child, None) {
                match status {
                    WaitStatus::Exited(_, code) => {
                        println!("[Oh My Shell] Child process terminated: {child} with code {code}");
                    },
                    WaitStatus::Signaled(_, _sig, _) => {
                        println!("[Oh My Shell] Child process terminated: {} by signal {:?}",child,_sig);
                    },
                    _ => {
                        println!("[Oh My Shell] Child process terminated: {child}");
                    }
                }
            }
        }

        
        for p in &fifo_paths { // deleting fifo
            let _ = fs::remove_file(p);
        }
        Some(())
    }

    fn handle_redirection(mut ins_vec: Vec<String>) -> Option<()>{
        if let Some(redirection_position) = ins_vec.iter().position(|x| x == ">" || x == "<") {
            let mut redirection_part = ins_vec.split_off(redirection_position);
            let command = ins_vec;
            let direction = redirection_part.remove(0);
            let target: &str = if let Some(input) = redirection_part.get(0){
                input.as_str()
            }else{
                println!("No target!");
                return None;
            };

            if direction == ">" {
                let file = OpenOptions::new().write(true).create(true).truncate(true).open(target).expect("Failed to open file for write");
                let fd = file.as_raw_fd();
                dup2(fd, libc::STDOUT_FILENO).expect("Failed to redirect STDOUT");
            } else { // "<"
                if !is_path(target){
                    println!("{target}: No such file or directory");
                    return None;
                }
                let file = OpenOptions::new().read(true).open(target).expect("Failed to open file for read");
                let fd = file.as_raw_fd();
                dup2(fd, libc::STDIN_FILENO).expect("Failed to redirect STDIN");
            }

            exec_command(command)
        } else {
            exec_command(ins_vec)
        }
    }

    fn exec_command(cmd_vec: Vec<String>) -> Option<()> {
        if cmd_vec.is_empty() {
            return None;
        }
        let command = CString::new(cmd_vec[0].as_str()).expect("Failed to create C-style string");
        let args: Vec<CString> = cmd_vec.iter().map(|s| CString::new(s.as_str()).expect("Failed to create C-string")).collect();
        match execvp(&command, &args){
            Ok(_) => Some(()),
            Err(_) => None,
        }
    }

}