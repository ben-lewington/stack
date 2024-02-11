mod lexer;
mod parse;
mod types;

use lexer::RawToken;
use types::{Op, Op1_0, Op2_1, Op2_2};

pub type Result<T, E = types::WaError> = core::result::Result<T, E>;

fn usage() {
    println!("usage: wa <command> <file>");
    println!("    subcommands:");
    println!("      - interpret, intrp, i: construct and run wa IR");
    println!("      - compile, com, c: compile generated bytecode");
    println!("      - dump, d: dump generated bytecode to file");
}

fn main() -> Result<()> {
    let mut args = std::env::args();

    if args.len() < 3 {
        usage();
        return Err("Not enough arguments".into());
    }

    let _program = args.next().unwrap();
    let subcmd = args.next().unwrap();
    let file_name = args.next().unwrap();
    let file = std::fs::read_to_string(&file_name[..])?;

    let ts: Vec<_> = lexer::WaTokenizer::new(file_name.as_ref(), file.as_bytes().as_ref())
        .into_iter()
        .map(|RawToken { row, col, token }| match token {
            s if s.len() > 2 && (&s[0..2] == "0x" || &s[0..2] == "0b") => {
                let base = match s.chars().skip(1).next() {
                    Some('x')=> 16,
                    Some('b') => 2,
                    _ => todo!("error handling properly"),
                };
                todo!()
            }
            s if s.len() > 1
                && s.chars().next().filter(|&ch| ch == '-').is_some()
                && s.chars().skip(1).all(|ch| ch.is_digit(10)) =>
            {
                Op::Push(-s[1..].parse::<isize>().unwrap())
            }
            s if s.len() > 0 && s.chars().all(|ch| ch.is_digit(10)) => {
                Op::Push(s.parse::<isize>().unwrap())
            }
            "+" => Op::Intrinsic2_1(Op2_1::Add),
            "-" => Op::Intrinsic2_1(Op2_1::Sub),
            "*" => Op::Intrinsic2_1(Op2_1::Mul),
            "/" => Op::Intrinsic2_1(Op2_1::Div),
            "%" => Op::Intrinsic2_1(Op2_1::Mod),
            "/%" => Op::Intrinsic2_2(Op2_2::DivMod),
            "." => Op::Intrinsic1_0(Op1_0::Display),
            "." => Op::Intrinsic1_0(Op1_0::Display),
        })
        .collect();

    // let ops = WaLexer(iter_charpoints_file(file_name.as_ref(), file.as_ref())).parse_ops()?;
    //
    // match subcmd.as_ref() {
    //     "interpret" | "intrp" | "i" => {
    //         let mut run_stack = RunStack::new();
    //         for (loc, op) in ops {
    //             match op {
    //                 Op::Push(n) => run_stack.push([n]),
    //                 Op::Intrinsic1_0(op_id) => op_id.into_op().run_op(&mut run_stack, loc)?,
    //                 Op::Intrinsic2_1(op_id) => op_id.into_op().run_op(&mut run_stack, loc)?,
    //                 Op::Intrinsic2_2(op_id) => op_id.into_op().run_op(&mut run_stack, loc)?,
    //             }
    //         }
    //     }
    //     _ => {
    //         todo!()
    //     }
    // }
    //
    Ok(())
}
