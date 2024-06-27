struct Connection {

}

impl Connection {
    fn interpreter() {
        loop {
            let mut s = String::new();
            print!("0xDBE1... (You)> ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\\') = s.chars().next_back() {
                println!("You can't use backslashes!");
                continue;
            }
            println!("0xF2E1... (Mr. X)> {}", s);

        }
    }
}