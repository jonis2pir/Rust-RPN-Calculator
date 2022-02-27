mod helper;
mod parse_parts;
use helper::variables::VarList;
use helper::{sys, math, printer, bitwise};
use parse_parts::tokenlist::TokenList;
use parse_parts::token::Token;
use std::io::{self, Write};

fn main() {
    sys::reset(false);
    printer::print_intro();

    let mut vertag  = false;
    let mut answer  = f64::NAN;
    let mut varlist = VarList::new();

    //input logic 
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut ori_line = String::new();
        std::io::stdin().read_line(&mut ori_line).expect("Couldn't read line");
        let line = ori_line.trim();
        
        if line.is_empty() || line.starts_with("#") {
            continue; //skip comment or blank line
        } else {
            match line {
                "exit"     => return,
                "reset"    => sys::reset(true),
                "clear"    => sys::reset(false),
                "help"     => printer::print_help(),
                "advhelp"  => printer::print_advanced_help(),
                "commands" => printer::print_commands(),
                "consts"   => printer::print_consts(),
                "vars"     => varlist.list_vars(),
                "verbose"  => {
                    if vertag == false {
                        vertag = true;
                        println!("verbose mode on\n")
                    } else {
                        vertag = false;
                        println!("verbose mode off\n")
                    }
                }
                "pop"      => {
                    if let Err(pop_err) = varlist.pop() {
                        println!("{}\n", pop_err);
                    } else { println!("pop successful\n") }
                }
                _ => {
                    if line.contains("=") {
                        add_var(&mut varlist, &line, &answer, vertag);
                    } else if line.starts_with("remove ") {
                        if line.len() > 7 {
                            if let Err(err) = varlist.remove(line[7..].trim()) {
                                println!("{}\n", err);
                            } else { println!("variable removed successfully\n") }
                        }
                    } else {
                        if let Ok(result) = do_rpn(&line, &answer, vertag, &varlist) {
                            println!("= {}\n", result);
                            answer = result;
                        } else { println!() }
                    }
                }
            }
        }
    }
}

/******************************* RPN CALCULATOR LOGIC BELOW *******************************/
fn do_rpn(expr: &str, prev_ans: &f64, verbose: bool, varlist: &VarList) -> Result<f64, ()> {
    let tokenizer_init = TokenList::tokenize(expr, varlist, &prev_ans);
    let mut tokens: TokenList;

    match tokenizer_init {
        Ok(token_list) => tokens = token_list,
        Err(error) => {
            println!("{}", error);
            return Err(());
        }
    }

    for step in 0..tokens.len() {
        let mut unary = false;

        if tokens.token_at(step).is_operator() {
            if verbose == true { tokens.print_list() };

            let val1 = tokens.fetch_back(step, 2).value();
            let val2 = tokens.fetch_back(step, 1).value();
            let result: f64;

            match tokens.token_at(step) {
                Token::ADD => result = val1 + val2,
                Token::SUB => result = val1 - val2,
                Token::MUL => result = val1 * val2,
                Token::DIV => result = val1 / val2,
                Token::MOD => result = val1 % val2,
                Token::POW => result = f64::powf(val1, val2),
                Token::ROT => result = math::nth_root(val1, val2),
                Token::LSH => result = bitwise::left_shift(val1, val2),
                Token::RSH => result = bitwise::right_shift(val1, val2),
                Token::AND => result = bitwise::and(val1, val2),
                Token::OR  => result = bitwise:: or(val1, val2),
                Token::XOR => result = bitwise::xor(val1, val2),
                Token::FAC => {
                    result = math::factorial(val2 as u32) as f64;
                    unary  = true;
                }
                Token::NOT => {
                    result = bitwise::not(val2);
                    unary  = true;
                }
                ___ => {
                    return Err(()); //something went wrong - return error
                }
            }

            tokens.set(step, Token::ANS(result));

            if !unary {
                tokens.set_used(step - 1);
                tokens.set_used(step - 2);
            } else {
                tokens.set_used(step - 1);
            }
        }
    }

    if verbose == true { tokens.print_list() };
    return Ok(tokens.last_ans());
}

fn add_var(varlist: &mut VarList, line: &str, prev_ans: &f64, verbose: bool) {
    let eqloc = line.find("=").unwrap(); //index of '='
    let vname = (line[..eqloc]).trim(); //variable name

    if let Ok(value) = do_rpn(&line[eqloc + 1..], prev_ans, verbose, varlist) {
        //expensive for now -> O(n)
        for i in 0..varlist.list.len() {
            if varlist.list[i].name_matches(vname) {
                varlist.list[i].value = value;
                println!("= {}\n", value);
                return;
            }
        }
        
        println!("= {}\n",value);
        varlist.add(vname, value);
    }
}