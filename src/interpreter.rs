use boa::{Expr, Op1, Op2};
use im::HashMap;

pub fn eval(expr: &Expr) -> i32 {
    let env = HashMap::new();
    eval_expr(expr, &env)
}

fn eval_expr(expr: &Expr, env: &HashMap<String, i32>) -> i32 {
    match expr {
        Expr::Number(n) => *n,

        Expr::Id(name) => {
            *env.get(name).unwrap_or_else(|| panic!("Unbound variable identifier {}", name))
        }

        Expr::UnOp(op, sub) => {
            let val = eval_expr(sub, env);
            match op {
                Op1::Add1 => val.wrapping_add(1),
                Op1::Sub1 => val.wrapping_sub(1),
            }
        }

        Expr::BinOp(op, left, right) => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Op2::Plus => l.wrapping_add(r),
                Op2::Minus => l.wrapping_sub(r),
                Op2::Times => l.wrapping_mul(r),
            }
        }

        Expr::Let(bindings, body) => {
            let mut new_env: HashMap<String, i32> = env.clone();
            for (name, val_expr) in bindings {
                let val = eval_expr(val_expr, &new_env);
                if new_env.contains_key(name) {
                    panic!("Duplicate binding");
                }
                new_env = new_env.update(name.clone(), val);
            }
            eval_expr(body, &new_env)
        }
    }
}
