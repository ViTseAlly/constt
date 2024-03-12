use tokio;
use user::UserInput;
//use fetch::{Request, Langs};

use std::error::Error;

mod user;
mod fetch;




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let mut usr_inp = UserInput::new();
    let usr_num = usr_inp.input("Enter the number of words:")
                        .is_int()
                        .to_i32()
                        .expect("Invalid input!");

    let mut lang_t = UserInput::new();
    let lang_num = lang_t.input("Enter the language [1 - it, 2 - zh, 3 - de, 4 - fr, 5 - es]:")
                        .md_usr_i(&[1, 2, 3, 4, 5])
                        .to_i32()
                        .expect("Invalid input!");

    println!("INPUT [COUNT WORDS] - {}", usr_num);
    println!("INPUT [LANG] - {}", lang_num);
    Ok(())
}
