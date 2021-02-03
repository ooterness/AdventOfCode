/// Day 18: https://adventofcode.com/2020/day/18
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

/// Tokenize an expression string
#[derive(Copy, Clone, Debug)]
enum Token {
    Number(i64),
    ParenL,
    ParenR,
    OpAdd,
    OpMul,
}
type TokenList = Vec<Token>;

fn push_if(num: &Option<i64>, vec:&mut TokenList) {
    if let Some(n) = num {
        vec.push(Token::Number(*n));
    }
}

fn parse_tokens(s:&String) -> Option<TokenList> {
    let mut tok:TokenList = Vec::new();
    let mut num:Option<i64> = None;
    let mut plevel:usize = 0;
    for c in s.chars() {
        // Contiguous digits form a number.
        if let Some(d) = c.to_digit(10) {
            if let Some(n) = num {
                num = Some(10*n + d as i64);
            } else {
                num = Some(d as i64);
            }
        } else {
            push_if(&num, &mut tok);        // Handle end-of-digits
            num = None;
            match c {
                '(' => {tok.push(Token::ParenL); plevel += 1;},
                ')' => {tok.push(Token::ParenR); if plevel > 0 {plevel -= 1;} else {return None;}},
                '+' => {tok.push(Token::OpAdd)},
                '*' => {tok.push(Token::OpMul)},
                ' ' => {},                  // Ignore whitespace
                _   => {return None},       // Unexpected character
            }
        }
    }
    push_if(&num, &mut tok);                // Finish last number, if applicable
    return Some(tok);                       // Success!
}

/// Nested mathematical expression
#[derive(Debug)]
enum Expression {
    Const(i64),
    Paren(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn new(s:&String, adv:bool) -> Option<Expression> {
        if let Some(tokens) = parse_tokens(&s) {
            Self::compile(&tokens, 0, tokens.len(), adv)
        } else {None}
    }

    fn compile(tokens:&TokenList, from:usize, to:usize, adv:bool) -> Option<Expression> {
        // Sanity check on array bounds:
        if from >= to {return None;}
        // Is this a two-part expression?
        let split = if adv {
            Self::find_split_adv(tokens, from, to)
        } else {
            Self::find_split_beg(tokens, from, to)
        };
        if split < to {
            // Two-part expression: LHS op RHS
            let lhs = Self::compile(tokens, from, split, adv);
            let rhs = Self::compile(tokens, split+1, to, adv);
            match (lhs, tokens[split], rhs) {
                (Some(l), Token::OpAdd, Some(r))
                    => return Some(Self::Add(Box::new(l), Box::new(r))),
                (Some(l), Token::OpMul, Some(r))
                    => return Some(Self::Mul(Box::new(l), Box::new(r))),
                _   => return None,
            }
        } else if to > from+2 {
            // Parenthetical expression: (X)
            let mid = Self::compile(tokens, from+1, to-1, adv);
            match (tokens[from], mid, tokens[to-1]) {
                (Token::ParenL, Some(x), Token::ParenR)
                    => return Some(Self::Paren(Box::new(x))),
                _   => return None,
            }
        } else if to == from+1 {
            // Only legal option is a constant.
            match tokens[from] {
                Token::Number(n)
                    => return Some(Self::Const(n)),
                _   => None,
            }
        } else {
            return None
        }
    }

    /// Find the rightmost +/* token at current nesting level.
    fn find_split_beg(tokens:&TokenList, from:usize, to:usize) -> usize {
        let mut p:usize = 0;
        for n in (from..to).rev() {
            match tokens[n] {
                Token::ParenR       => {p += 1;},
                Token::ParenL       => {p -= 1;},
                Token::OpAdd        => if p == 0 {return n;},
                Token::OpMul        => if p == 0 {return n;},
                _                   => {},
            }
        }
        to
    }

    /// Find first * token at current nesting level, then first + token.
    fn find_split_adv(tokens:&TokenList, from:usize, to:usize) -> usize {
        let mut p:usize = 0;
        for n in from..to {
            match tokens[n] {
                Token::ParenL       => {p += 1;},
                Token::ParenR       => {p -= 1;},
                Token::OpMul        => if p == 0 {return n;},
                _                   => {},
            }
        }
        for n in from..to {
            match tokens[n] {
                Token::ParenL       => {p += 1;},
                Token::ParenR       => {p -= 1;},
                Token::OpAdd        => if p == 0 {return n;},
                _                   => {},
            }
        }
        to
    }

    /// Recursively evaluate this expression.
    pub fn eval(&self) -> i64 {
        match self {
            Self::Const(x) => *x,
            Self::Paren(x) => x.eval(),
            Self::Add(x,y) => x.eval() + y.eval(),
            Self::Mul(x,y) => x.eval() * y.eval(),
        }
    }
}

fn test_eval(refa:i64, refb:i64, x:&str) {
    let s = String::from(x);
    let expr_beg = Expression::new(&s, false);
    let expr_adv = Expression::new(&s, true);
    if let (Some(beg),Some(adv)) = (expr_beg, expr_adv) {
        let x = beg.eval();
        let y = adv.eval();
        if x != refa {eprintln!("Test value mismatch {} /= {}", x, refa)};
        if y != refb {eprintln!("Test value mismatch {} /= {}", y, refb)};
    } else {
        eprintln!("Test compile error: {}", x);
    }
}

pub fn solve() {
    // Unit tests:
    test_eval(71,    231,    "1 + 2 * 3 + 4 * 5 + 6");
    test_eval(51,    51,     "1 + (2 * 3) + (4 * (5 + 6))");
    test_eval(26,    46,     "2 * 3 + (4 * 5)");
    test_eval(437,   1445,   "5 + (8 * 3 + 9 + 3 * 4 * 3)");
    test_eval(12240, 669060, "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    test_eval(13632, 23340,  "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

    // Part 1: Basic arithmetic rules.
    let input = common::read_strings("input/input18.txt");
    let expr_beg:Vec<Expression> = input.iter()
        .filter_map(|x| Expression::new(x,false)).collect();
    if expr_beg.len() == input.len() {
        let sum:i64 = expr_beg.iter().map(|x| x.eval()).sum();
        println!("Part1: Sum = {}", sum);
    } else {
        eprintln!("Part1: Compile error");
    }

    // Part 2: Advanced arithmetic rules.
    let expr_adv:Vec<Expression> = input.iter()
        .filter_map(|x| Expression::new(x,true)).collect();
    if expr_adv.len() == input.len() {
        let sum:i64 = expr_adv.iter().map(|x| x.eval()).sum();
        println!("Part2: Sum = {}", sum);
    } else {
        eprintln!("Part2: Compile error");
    }
}
