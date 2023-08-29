use std::io::Read;

extern crate rand;
use rand::Rng;

pub struct Environment {
    stack: [usize; 1000],
    sp: usize,
    field: [[usize; 80]; 25],
    position: [usize; 2],
    velocity: [isize; 2],
    rng: rand::rngs::ThreadRng,
    string_mode: bool,
    running: bool,
}

pub trait Befunge93Interpreter {
    fn new(code: String) -> Self;
    fn get_current_instruction(&self) -> usize;
    fn update_position(&mut self);
    fn set_cell(&mut self, x: usize, y: usize, value: usize);
    fn get_cell(&self, x: usize, y: usize) -> usize;
    fn push(&mut self, value: usize);
    fn pop(&mut self) -> usize;
    fn execute_instruction(&mut self, instruction: usize);
    fn execute(&mut self);
}

impl Befunge93Interpreter for Environment {
    fn new(code: String) -> Self {
        let mut field: [[usize; 80]; 25] = [[0; 80]; 25];
        let mut row = 0;
        let mut col = 0;
        for c in code.chars() {
            if c == '\n' {
                row += 1;
                col = 0;
            } else {
                match field.get_mut(row) {
                    Some(r) => {
                        match r.get_mut(col) {
                            Some(cell) => {
                                *cell = c as usize;
                            },
                            None => {
                                panic!("Code must have less than 80 columns");
                            }
                        }
                    },
                    None => {
                        panic!("Code must have less than 25 rows");
                    }
                }
                col += 1;
            }
        }
        let rng = rand::thread_rng();
        return Self {
            stack: [0; 1000],
            sp: 0,
            field: field,
            position: [0, 0],
            velocity: [1, 0],
            rng: rng,
            string_mode: false,
            running: false,
        };
    }

    fn update_position(&mut self) {
        for (index, item) in self.velocity.iter().enumerate() {
            if *item == 0 {
                continue;
            }
            let mut new_position = self.position[index] as isize + *item;
            if new_position < 0 {
                new_position = match index {
                    0 => 79,
                    1 => 24,
                    _ => panic!("Invalid index"),
                };
            } else if new_position > match index {
                0 => 79,
                1 => 24,
                _ => panic!("Invalid index"),
            } as isize {
                new_position = 0;
            }
            self.position[index] = new_position as usize;
        }
    }

    fn get_current_instruction(&self) -> usize {
        let col = self.position[0];
        let row = self.position[1];
        self.field[row][col]
    }

    fn set_cell(&mut self, x: usize, y: usize, value: usize) {
        match self.field.get_mut(y) {
            Some(r) => {
                match r.get_mut(x) {
                    Some(cell) => {
                        *cell = value;
                    },
                    None => {
                        panic!("Cannot set cell outside of field");
                    }
                }
            },
            None => {
                panic!("Cannot set cell outside of field");
            }
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> usize {
        self.field[y][x]
    }

    fn push(&mut self, value: usize) {
        self.sp += 1;
        self.stack[self.sp] = value;
    }

    fn pop(&mut self) -> usize {
        let value = self.stack[self.sp];
        self.sp -= 1;
        value
    }

    fn execute_instruction(&mut self, instruction: usize) {
        let i = char::from(instruction as u8);

        if self.string_mode {
            if i == '"' {
                self.string_mode = false;
            } else {
                self.push(instruction);
            }
            self.update_position();
            return;
        }

        match i {
            '0'..='9' => {
                self.push(instruction - usize::from('0' as u8));
            },
            '+' => {
                let a = self.pop();
                let b = self.pop();
                self.push(a + b);
            },
            '-' => {
                let a = self.pop();
                let b = self.pop();
                self.push(b - a);
            },
            '*' => {
                let a = self.pop();
                let b = self.pop();
                self.push(a * b);
            },
            '/' => {
                let a = self.pop();
                let b = self.pop();
                self.push(b / a);
            },
            '%' => {
                let a = self.pop();
                let b = self.pop();
                self.push(b % a);
            },
            '!' => {
                let a = self.pop();
                if a == 0 {
                    self.push(1);
                } else {
                    self.push(0);
                }
            },
            '`' => {
                let a = self.pop();
                let b = self.pop();
                if b > a {
                    self.push(1);
                } else {
                    self.push(0);
                }
            },
            '>' => {
                self.velocity = [1, 0];
            },
            '<' => {
                self.velocity = [-1, 0];
            },
            '^' => {
                self.velocity = [0, -1];
            },
            'v' => {
                self.velocity = [0, 1];
            },
            '?' => {
                let direction = self.rng.gen_range(0..4);
                match direction {
                    0 => {
                        self.velocity = [1, 0];
                    },
                    1 => {
                        self.velocity = [-1, 0];
                    },
                    2 => {
                        self.velocity = [0, -1];
                    },
                    3 => {
                        self.velocity = [0, 1];
                    },
                    _ => {
                        panic!("Invalid direction");
                    }
                }
            },
            '_' => {
                let a = self.pop();
                if a == 0 {
                    self.velocity = [1, 0];
                } else {
                    self.velocity = [-1, 0];
                }
            },
            '|' => {
                let a = self.pop();
                if a == 0 {
                    self.velocity = [0, 1];
                } else {
                    self.velocity = [0, -1];
                }
            },
            '"' => {
                self.string_mode = !self.string_mode;
            },
            ':' => {
                if self.sp == 0 {
                    self.push(0);
                } 
                let a = self.pop();
                self.push(a);
                self.push(a);
            },
            '\\' => {
                let a = self.pop();
                let b = self.pop();
                self.push(a);
                self.push(b);
            },
            '$' => {
                self.pop();
            },
            '.' => {
                let a = self.pop();
                print!("{}", a);
            },
            ',' => {
                let a = self.pop();
                print!("{}", char::from(a as u8));
            },
            '#' => {
                self.update_position();
            },
            'p' => {
                let y = self.pop();
                let x = self.pop();
                let v = self.pop();
                self.set_cell(x, y, v);
            },
            'g' => {
                let y = self.pop();
                let x = self.pop();
                let mut v = 0;
                if (y <= 24) && (x <= 79){
                    v = self.get_cell(x, y);
                }
                self.push(v);
            },
            '&' => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let val: usize = input.trim().parse().expect("Input not an integer");
                self.push(val)
            },
            '~' => {
                let mut buf: [u8; 1] = [0];
                std::io::stdin().read_exact(&mut buf).expect("Failed to read byte");
                self.push(buf[0] as usize)
            },
            '@' => {
                self.running = false;
            },
            _ => {
                // Do nothing if the instruction is not recognized
            }
        }
        self.update_position();
    }

    fn execute(&mut self) {
        self.running = true;
        while self.running {
            let instruction = self.get_current_instruction();
            self.execute_instruction(instruction);
        }
    }
    
}
