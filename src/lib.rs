pub mod ops;
pub mod parse;
pub mod stack;
pub mod tokenise;
pub mod utils;

use parse::Program;

use crate::tokenise::{Span, TokenIdx};

pub fn interp_program(
    file_name: impl AsRef<str>,
    Program { ops, branches: _ }: Program,
) -> anyhow::Result<()> {
    let mut stack = stack::Stack::new();
    let mut jmp_check = vec![];

    let mut prev_tok_id: Option<TokenIdx> = None;
    let mut ip = 0;
    loop {
        match ops.get(ip) {
            Some(Span {
                idx: tok_id,
                token: op,
            }) => {
                let fmt_span = Span {
                    idx: *tok_id,
                    token: file_name.as_ref(),
                };
                match *op {
                    ops::Op::Push(n) => stack.push([n]),
                    ops::Op::Intr1_0(op_id) => stack.run(op_id.into_op(), fmt_span)?,
                    ops::Op::Intr1_2(op_id) => stack.run(op_id.into_op(), fmt_span)?,
                    ops::Op::Intr2_1(op_id) => stack.run(op_id.into_op(), fmt_span)?,
                    ops::Op::Intr2_2(op_id) => stack.run(op_id.into_op(), fmt_span)?,
                    ops::Op::If(end_idx) => {
                        let [i] = stack.pop::<1>(fmt_span)?;
                        jmp_check.push(stack.len());
                        match i {
                            1 => {}
                            0 => {
                                ip = end_idx.0;
                                continue;
                            }
                            i => anyhow::bail!(
                                "{at}: expected bool, got {i}",
                                at = tok_id.as_stamp(&file_name)
                            ),
                        }
                    }
                    ops::Op::End => {
                        let len = jmp_check.pop().ok_or(anyhow::anyhow!(
                            "{at}: Unbalanced END expr",
                            at = tok_id.as_stamp(&file_name)
                        ))?;
                        // TODO: This can be checked at compile time
                        if len != stack.len() {
                            anyhow::bail!(
                                "{}: conditional execution must not alter stack length. expected: {len}, got: {}", tok_id.as_stamp(&file_name), stack.len())
                        }
                    }
                };
                println!("{at}: {op}", at = tok_id.as_stamp(&file_name));
                println!("{stack} ");
                prev_tok_id.replace(*tok_id);
                ip += 1;
            }
            None => break,
        }
    }

    if stack.len() != 0 {
        anyhow::bail!(
            "{}: Unhandled data on the stack. {} element(s) remaining after last operation",
            prev_tok_id
                .unwrap_or(Default::default())
                .as_stamp(&file_name),
            stack.len()
        )
    }

    Ok(())
}

pub fn compile_program() -> anyhow::Result<()> {
    Ok(())
}
