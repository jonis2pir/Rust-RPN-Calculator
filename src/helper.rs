pub mod variables {
    pub struct Var {
        pub name: String,
        pub value: f64
    }
    
    impl Var {
        pub fn new(name: &str, value: f64) -> Self {
            Self {
                name: name.to_string(),
                value: value
            }
        }

        pub fn name_matches(&self, name: &str) -> bool {
            if self.name == name { true } else { false }
        }

        pub fn print(&self) {
            println!("[{} = {}]", self.name, self.value)
        }
    }

    
    /****** START OF VARLIST STRUCT ******/
    pub struct VarList {
        pub list: Vec<Var>,
    }

    impl VarList {
        pub fn new() -> Self {
            Self {
                list: Vec::new()
            }
        }

        pub fn add(&mut self, name: &str, value: f64) {
            self.list.push(Var::new(name, value));
        }

        pub fn pop(&mut self) -> Result<(), &'static str> {
            if self.list.is_empty() {
                return Err("no variables to pop")
            }
            
            self.list.pop();
            return Ok(())
        }

        pub fn remove(&mut self, name: &str) -> Result<(), String> {
            if self.list.is_empty() {
                return Err("no variables to remove".to_string())
            }

            for i in 0..self.list.len() {
                if self.list[i].name_matches(name) {
                    self.list.swap_remove(i);
                    return Ok(());
                }
            }

            return Err(format!("'{}' is not defined", name));
        }

        pub fn fetch(&self, name: &str) -> Result<f64, ()> {
            for i in 0..self.list.len() {
                if self.list[i].name_matches(name) {
                    return Ok(self.list[i].value);
                }
            }
        
            Err(()) //return error if var is undefined
        }

        pub fn list_vars(&self) {
            for i in 0..self.list.len() {
                self.list[i].print();
            }
            println!();
        }
    }
}

pub mod math {
    pub fn wants_constant(s: &str) -> bool {
        let c = s.to_uppercase();
        c == "PI" || c == "TAU" || c == "E"
    }

    pub fn fetch_constant(s: &str) -> f64 {
        let c = s.to_ascii_uppercase();

        if c == "PI"  { std::f64::consts::PI  } else
        if c == "TAU" { std::f64::consts::TAU } else
        if c == "E"   { std::f64::consts::E   } else

        {0.0} //default case
    }

    pub fn nth_root(radicand: f64, index: f64) -> f64 {
        f64::powf(index, 1.0 / radicand)
    }

    pub fn factorial(n: u32) -> u128 {
        let mut u_fact: u128 = 1;
        let mut x = n as u128;

        while x > 1 {
            u_fact *= x;
            x -= 1;
        }
        
        u_fact
    }
}

pub mod bitwise {
    pub fn and(a: f64, b: f64) -> f64 {
        ((a as i64) & (b as i64)) as f64
    }

    pub fn or(a: f64, b: f64) -> f64 {
        ((a as i64) | (b as i64)) as f64
    }

    pub fn xor(a: f64, b: f64) -> f64 {
        ((a as i64) ^ (b as i64)) as f64
    }

    pub fn not(a: f64) -> f64 {
        (!(a as i64)) as f64
    }

    pub fn left_shift(base: f64, amt: f64) -> f64 {
        ((base as i64) << (amt as i64)) as f64
    }

    pub fn right_shift(base: f64, amt: f64) -> f64 {
        ((base as i64) >> (amt as i64)) as f64
    }
}

pub mod printer {
    pub fn print_intro() {
        println!("┌──────────────────────────────────────────┐");
        println!("│ RPN (Reverse Polish Notation) Calculator │");
        println!("├──────────────────────────────────────────┤");
        println!("│ type 'help' or 'advhelp' to show help    │");
        println!("│ type 'commands' to list all commands     │");
        println!("│ type 'reset' to clear screen             │");
        println!("│ type 'exit' to stop program              │");
        println!("└──────────────────────────────────────────┘\n");
    }

    pub fn print_help() {
        println!("┌───────────────────────────────────────────────────────┐");
        println!("│ permitted operators/keywords: + - * / ^ root fact ans │");
        println!("│ permitted bitwise operators:  << >> and or xor not    │");
        println!("├───────────────────────────────────────────────────────┤");
        println!("│ infix: 2 * 4  ->  postfix: 2 4 *   (multiplication)   │");
        println!("│ infix: 2 ^ 8  ->  postfix: 2 8 ^   (exponentiation)   │");
        println!("│ infix: 8 % 2  ->  postfix: 8 2 %   (remainder)        │");
        println!("│ infix: 8 / 4  ->  postfix: 8 4 /   (division)         │");
        println!("│ infix: 28!    ->  postfix: 28 fact (factorial)        │");
        println!("└───────────────────────────────────────────────────────┘\n");
    }

    pub fn print_advanced_help() {
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│ permitted operators/keywords:  + - * / % ^ root fact ans        │");
        println!("│ permitted bitwise operators:   << >> and or xor not             │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ ('ans' will use previous output/result)                         │");
        println!("│ ('fact' will only use previous number/result as argument)       │");
        println!("│ (malformed expressions result in undefined behavior)            │");
        println!("│ (define variable: x = 1 8 <<     where x is the variable name)  │"); 
        println!("│ (access variable: $x 2 /         where x is the variable name)  │");
        println!("│                                                                 │");
        println!("│ infix: (a - b) * (c + ans)   ->  postfix: a b - c ans + *       │");
        println!("│ infix: a ^ b / (c * PI) + 3  ->  postfix: a b ^ c PI * / e +    │");
        println!("│ infix: 8√256 + (100 * 10)    ->  postfix: 8 256 root 100 10 * + │");
        println!("│ infix: 24√(2 ^ (2 * 3 * 4))  ->  postfix: 24 2 2 3 * 4 * ^ root │");
        println!("└─────────────────────────────────────────────────────────────────┘\n");
    }

    pub fn print_commands() {
        println!("┌──────────────────────────────────────────────────┐");
        println!("│ help              shows help                     │");
        println!("│ advhelp           shows advanced help            │");
        println!("│ consts            shows available constants      │");
        println!("│ vars              shows user defined variables   │");
        println!("│ pop               pops (removes) last variable   │");
        println!("│ remove <name>     removes specified variable     │");
        println!("│ clear             clears screen (hides intro)    │");
        println!("│ reset             clears screen (shows intro)    │");
        println!("│ exit              exits program                  │");
        println!("│ verbose           shows extra information such   │");
        println!("│                   as stack for current operation │");
        println!("└──────────────────────────────────────────────────┘\n");
    }

    pub fn print_consts() {
        println!("┌────────────────┐");
        println!("│ PI  ≈ 3.141592 │");
        println!("│ TAU ≈ 6.283185 │");
        println!("│ E   ≈ 2.718281 │");
        println!("└────────────────┘\n");
    }
}

pub mod sys {
    pub fn reset(print_intro: bool) {
        print!("{esc}c", esc = 27 as char);

        if print_intro {
            crate::helper::printer::print_intro();
        }
    }
}