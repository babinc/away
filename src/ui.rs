use std::io;
use std::io::Write;

pub struct Ui {
    line_len: usize
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            line_len: 0
        }
    }

    pub fn write(&mut self, output: &str) {
        if output.len() > self.line_len {
            self.clear();
        }
        print!("\r");
        io::stdout().flush().unwrap();

        print!("{}", output);
        io::stdout().flush().unwrap();

        self.line_len = output.len();
    }

    fn clear(&self) {
        print!("\r");
        io::stdout().flush().unwrap();

        let clear_char = " ";
        let mut clear_str = "".to_string();

        for _ in 0..self.line_len {
            clear_str += clear_char;
        }

        print!("{}", clear_str);
        io::stdout().flush().unwrap();
    }
}