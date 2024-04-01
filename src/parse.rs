use anyhow::Context;

use crate::{
    ops::{Op, Op1_0, Op1_2, Op2_1, Op2_2, OpIdx},
    tokenise::Span,
    utils::Chunk,
};

pub struct Program {
    pub ops: Vec<Span<Op>>,
    pub branches: Vec<Branch>,
}

pub enum Branch {
    If {
        at: Span<OpIdx>,
        elses: Vec<Span<OpIdx>>,
        end: Span<OpIdx>,
    },
}

pub fn parse_ops(tokens: Vec<Span<&str>>, file_name: impl AsRef<str>) -> anyhow::Result<Program> {
    let file_name = file_name.as_ref();
    let mut it = crate::utils::Descend(tokens.into_iter().enumerate());
    let mut ops = vec![];
    let branches = vec![];
    let parse_depth = 0;

    loop {
        match it.chop_opt::<1>() {
            Chunk::AllOf([Some((_, Span { idx: tok_id, token }))]) => {
                println!("{file_name}:{tok_id}: {token}");
                let at = tok_id.as_stamp(file_name);
                let op = match token {
                    s if s.len() > 2 && (&s[0..2] == "0x" || &s[0..2] == "0b") => {
                        let base = match s.chars().skip(1).next() {
                            Some('x') => 16,
                            Some('b') => 2,
                            _ => unreachable!(),
                        };
                        let (_, num) = s.split_at(2);
                        Op::Push(isize::from_str_radix(num, base).with_context(|| {
                            format!("{at}: unable to parse \"{s}\" as base-{base} numeric literal",)
                        })?)
                    }
                    s if s.len() > 1
                        && s.chars().next().filter(|&ch| ch == '-').is_some()
                        && s.chars().skip(1).all(|ch| ch.is_digit(10)) =>
                    {
                        Op::Push(-s[1..].parse::<isize>().with_context(|| {
                            format!("{at}: unable to parse \"{s}\" as negative numeric literal",)
                        })?)
                    }
                    s if s.len() > 0 && s.chars().all(|ch| ch.is_digit(10)) => {
                        Op::Push(s.parse::<isize>().with_context(|| {
                            format!("{at}: unable to parse \"{s}\" as numeric literal",)
                        })?)
                    }
                    "." => Op::Intr1_0(Op1_0::Display),
                    "+" => Op::Intr2_1(Op2_1::Add),
                    "-" => Op::Intr2_1(Op2_1::Sub),
                    "*" => Op::Intr2_1(Op2_1::Mul),
                    "/" => Op::Intr2_1(Op2_1::Div),
                    "%" => Op::Intr2_1(Op2_1::Mod),
                    "=" => Op::Intr2_1(Op2_1::Equ),
                    "<" => Op::Intr2_1(Op2_1::Less),
                    ">" => Op::Intr2_1(Op2_1::Greater),
                    "<=" => Op::Intr2_1(Op2_1::LessEqu),
                    ">=" => Op::Intr2_1(Op2_1::GreaterEqu),
                    "/%" => Op::Intr2_2(Op2_2::DivMod),
                    "drop" => Op::Intr1_0(Op1_0::Drop),
                    "dup" => Op::Intr1_2(Op1_2::Duplicate),
                    "swap" => Op::Intr2_2(Op2_2::Swap),
                    "if" => {
                        let mut t = it.clone();
                        let end_addr = loop {
                            match t.chop_opt::<1>() {
                                Chunk::AllOf([Some((e_id, Span { idx: _, token }))]) => match token
                                {
                                    "end" | "else" => break e_id,
                                    _ => {}
                                },
                                Chunk::NoneOf => anyhow::bail!(
                                    "{at}: Unbalanced IF expression",
                                    at = tok_id.as_stamp(&file_name)
                                ),
                                _ => unreachable!("Chunk::<1, I>::SomeOf??"),
                            }
                        };
                        Op::If(OpIdx::new(end_addr + 1))
                    }
                    "end" => Op::End,
                    t => anyhow::bail!("{at}: unknown token \"{t}\""),
                };
                ops.push(Span {
                    idx: tok_id,
                    token: op,
                });
            }
            Chunk::NoneOf => break,
            _ => unreachable!(),
        }
    }
    Ok(Program { ops, branches })
}
