use sexp::{Sexp, Atom::*};
use boa::{Expr, Op1, Op2};

pub fn parse(source: &str) -> Expr {
    let parsed = sexp::parse(source).expect("Invalid S-expression");

    match parsed {
        Sexp::List(mut exprs) => {
            if exprs.len() != 1 {
                panic!("Expected exactly one expression");
            }
            parse_expr(&exprs.remove(0))
        }
        _ => panic!("Expected top-level list of expressions"),
    }
}

pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Number(*n as i32),
        Sexp::Atom(S(s)) => Expr::Id(s.clone()),
        Sexp::List(list) => {
            if list.is_empty() {
                panic!("Invalid: empty list");
            }

            match &list[0] {
                Sexp::Atom(S(op)) => match op.as_str() {
                    "add1" => {
                        if list.len() != 2 {
                            panic!("Invalid: add1 takes 1 argument");
                        }
                        Expr::UnOp(Op1::Add1, Box::new(parse_expr(&list[1])))
                    }
                    "sub1" => {
                        if list.len() != 2 {
                            panic!("Invalid: sub1 takes 1 argument");
                        }
                        Expr::UnOp(Op1::Sub1, Box::new(parse_expr(&list[1])))
                    }
                    "+" | "-" | "*" => {
                        if list.len() != 3 {
                            panic!("Invalid: binary operator takes 2 arguments");
                        }
                        let lhs = Box::new(parse_expr(&list[1]));
                        let rhs = Box::new(parse_expr(&list[2]));
                        let op = match op.as_str() {
                            "+" => Op2::Plus,
                            "-" => Op2::Minus,
                            "*" => Op2::Times,
                            _ => unreachable!(),
                        };
                        Expr::BinOp(op, lhs, rhs)
                    }
                    "let" => {
                        if list.len() != 3 {
                            panic!("Invalid: let must have bindings and a body");
                        }

                        let bindings_list = match &list[1] {
                            Sexp::List(v) => v,
                            _ => panic!("Invalid: expected binding list"),
                        };

                        let mut bindings = vec![];
                        let mut names = std::collections::HashSet::new();

                        for b in bindings_list {
                            let (name, val) = parse_bind(b);
                            if names.contains(&name) {
                                panic!("Duplicate binding");
                            }
                            names.insert(name.clone());
                            bindings.push((name, val));
                        }

                        let body = Box::new(parse_expr(&list[2]));
                        Expr::Let(bindings, body)
                    }
                    _ => panic!("Invalid: unknown operator '{}'", op),
                },
                _ => panic!("Invalid: first element of list must be an operator"),
            }
        }
        _ => panic!("Invalid: unexpected expression"),
    }
}

pub fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(v) => {
            if v.len() != 2 {
                panic!("Invalid: binding must be of form (name expr)");
            }

            let name = match &v[0] {
                Sexp::Atom(S(s)) => s.clone(),
                _ => panic!("Invalid: binding name must be identifier"),
            };

            let reserved = ["let", "add1", "sub1", "+", "-", "*"];
            if reserved.contains(&name.as_str()) {
                panic!("Invalid: '{}' is a reserved keyword", name);
            }

            let expr = parse_expr(&v[1]);
            (name, expr)
        }
        _ => panic!("Invalid: expected a binding list"),
    }
}
