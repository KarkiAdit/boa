use crate::lib::{Expr, Instr, Op1, Op2, Reg, Val};
use im::HashMap;

pub fn compile_to_instrs(expr: &Expr) -> Vec<Instr> {
    let mut env = HashMap::new();
    let mut stack_index = 0;
    compile_expr(expr, &mut env, &mut stack_index)
}

fn compile_expr(expr: &Expr, env: &mut HashMap<String, i32>, stack_index: &mut i32) -> Vec<Instr> {
    match expr {
        Expr::Number(n) => {
            vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n))]
        }

        Expr::Id(name) => {
            match env.get(name) {
                Some(offset) => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, *offset))],
                None => panic!("Unbound variable identifier {}", name),
            }
        }

        Expr::UnOp(op, subexpr) => {
            let mut code = compile_expr(subexpr, env, stack_index);
            match op {
                Op1::Add1 => code.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1))),
                Op1::Sub1 => code.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(1))),
            }
            code
        }

        Expr::BinOp(op, left, right) => {
            let mut code = compile_expr(left, env, stack_index);
            code.push(Instr::IMov(Val::RegOffset(Reg::RSP, *stack_index), Val::Reg(Reg::RAX)));

            env.insert("__tmp".to_string(), *stack_index);
            *stack_index -= 8;

            code.extend(compile_expr(right, env, stack_index));
            *stack_index += 8;

            let tmp_offset = *env.get("__tmp").unwrap();
            let lhs = Val::RegOffset(Reg::RSP, tmp_offset);

            match op {
                Op2::Plus => code.push(Instr::IAdd(Val::Reg(Reg::RAX), lhs)),
                Op2::Minus => {
                    code.push(Instr::IMov(Val::Reg(Reg::RAX), lhs));
                    code.push(Instr::ISub(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, tmp_offset + 8)));
                }
                Op2::Times => {
                    code.push(Instr::IMul(Val::Reg(Reg::RAX), lhs));
                }
            }

            env.remove("__tmp");
            code
        }

        Expr::Let(bindings, body) => {
            let mut code = vec![];
            let original_env = env.clone();

            for (name, value_expr) in bindings {
                code.extend(compile_expr(value_expr, env, stack_index));
                code.push(Instr::IMov(Val::RegOffset(Reg::RSP, *stack_index), Val::Reg(Reg::RAX)));
                env.insert(name.clone(), *stack_index);
                *stack_index -= 8;
            }

            code.extend(compile_expr(body, env, stack_index));
            *env = original_env;
            code
        }
    }
}

pub fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::IMov(dest, src) => format!("mov {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::IAdd(dest, src) => format!("add {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::ISub(dest, src) => format!("sub {}, {}", val_to_str(dest), val_to_str(src)),
    }
}

pub fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(Reg::RAX) => "rax".to_string(),
        Val::Reg(Reg::RSP) => "rsp".to_string(),
        Val::Imm(n) => format!("{}", n),
        Val::RegOffset(Reg::RSP, offset) => format!("[rsp - {}]", -offset),
        Val::RegOffset(_, _) => panic!("Only RSP offset supported for now"),
    }
}
