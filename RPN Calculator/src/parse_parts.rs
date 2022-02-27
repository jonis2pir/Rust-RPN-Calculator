pub mod tokenlist {
    use crate::math;
    use crate::parse_parts::token::Token;
    use crate::helper::variables::VarList;

    pub struct TokenList {
        pub list: Vec<Token>
    }

    impl TokenList {
        pub fn tokenize(expr: &str, vars: &VarList, prev_ans: &f64) -> Result<Self, String> {
            let items: Vec<&str> = expr.split_ascii_whitespace().collect();
            let mut tlist = Vec::with_capacity(items.len());

            for i in 0..items.len() {
                match items[i] {
                    "+"     => tlist.push(Token::ADD),
                    "-"     => tlist.push(Token::SUB),
                    "*"     => tlist.push(Token::MUL),
                    "/"     => tlist.push(Token::DIV),
                    "%"     => tlist.push(Token::MOD),
                    "^"     => tlist.push(Token::POW),
                    "root"  => tlist.push(Token::ROT),
                    "fact"  => tlist.push(Token::FAC),
                    "and"   => tlist.push(Token::AND),
                    "not"   => tlist.push(Token::NOT),
                    "or"    => tlist.push(Token::OR ),
                    "xor"   => tlist.push(Token::XOR),
                    "<<"    => tlist.push(Token::LSH),
                    ">>"    => tlist.push(Token::RSH),
                    "ans"   => tlist.push(Token::NUM(*prev_ans)),
                    _____   => {
                        if math::wants_constant(items[i]) {
                            tlist.push(Token::NUM(
                                math::fetch_constant(items[i])
                            ))
                        //get variable
                        } else if items[i].starts_with("$") {
                            let vname = &items[i][1..];
                            if let Ok(var) = vars.fetch(vname) {
                                tlist.push(Token::NUM(var))
                            } else {
                                return Err(format!("'{}' is not defined", vname))
                            }
                        //value is numerical or invalid
                        } else {
                            if let Ok(val) = items[i].parse::<f64>() {
                                tlist.push(Token::NUM(val));
                            } else {
                                return Err(format!("'{}' is an invalid token", items[i]))
                            }
                        }
                    }
                }
            }

            Ok( Self {list: tlist} )
        }

        pub fn print_list(&self) {
            println!("{:?}", self.list);
        }

        pub fn len(&self) -> usize {
            self.list.len()
        }

        pub fn set(&mut self, index: usize, token: Token) {
                self.list[index] = token;
        }

        pub fn set_used(&mut self, mut index: usize) {
            while matches!(self.token_at(index), Token::USED) {
                index -= 1;
            }

            self.set(index, Token::USED);
        }

        pub fn token_at(&self, index: usize) -> &Token {
            return &self.list[index];
        }

        pub fn fetch_back(&self, mut from: usize, amount: usize) -> &Token {
            from -= if amount <= from {amount} else {0};

            while matches!(self.token_at(from), Token::USED) && from != 0 {
                from -= 1;
            }

            return self.token_at(from);
        }

        //gets the result from the list
        pub fn last_ans(&self) -> f64 {
            for token in (0..self.len()).rev() {
                match &self.list[token] {
                    Token::ANS(ans) => return *ans,
                    _______________ => (),
                }
            }

            for token in (0..self.len()).rev() {
                match &self.list[token] {
                    Token::NUM(num) => return *num,
                    _______________ => (),
                }
            }

            return f64::NAN;
        }
    }
}

pub mod token {
    #[derive(Debug)]
    pub enum Token {
        /**** RPN TOKENS ****/
        ADD,  SUB,  MUL,  DIV,
        POW,  ROT,  FAC,  MOD,
        AND,  OR ,  NOT,  XOR,
        LSH,  RSH,
        USED,
        NUM(f64),
        ANS(f64)
    }

    impl Token {
        pub fn is_operator(&self) -> bool {
            match self {
                Token::NUM(_f64) => return false,
                ___ => return true
            }
        }

        pub fn value(&self) -> f64 {
            if let Token::NUM(num) = self { return *num } else
            if let Token::ANS(ans) = self { return *ans } else
            { return f64::NAN }
        }
    }
}