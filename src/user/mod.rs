use std::io::{self, Write};

pub struct UserInput {
    val: Option<String>,
}

impl UserInput {
    pub fn new() -> Self {
        UserInput { val: None }
    }

    pub fn input(&mut self, _txt: &str) -> &mut Self {
        print!("{}", _txt);
        io::stdout().flush().expect("Error flushing stdout!");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Error: cant read input!");
        self.val = Some(inp.trim().to_string());
        self
    }

    pub fn to_i32(&mut self) -> Option<i32> {
        if let Some(value) = self.val.as_ref() {
            let trimmed_value = value.trim();
            match trimmed_value.parse() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn md_usr_i(&mut self, _arr: &[i32]) -> &mut Self {
        loop {
            if let Some(v) = self.to_i32() {
                if _arr.contains(&v) {
                    return self;
                } else {
                    println!("Invalid input! Please enter a number from the specified range.");
                    self.input("Enter the language [1 - it, 2 - zh, 3 - de, 4 - fr, 5 - es]");
                }
            } else {
                println!("Invalid input! Please enter a number from the specified range.");
                self.input("Enter the language [1 - it, 2 - zh, 3 - de, 4 - fr, 5 - es]");
            }
        }
    }

    pub fn is_int(&mut self) -> &mut Self {
        loop {
            if let Some(v) = self.to_i32() {
                return self;
            } else {
                println!("Invalid input! Please enter a number.");
                self.input("Enter the number of words:");
            } 
        }
    }
}