use crate::types::{FileMapped, Op, Op1_0, Op2_1, Op2_2, WaError};

impl<'a, T: Iterator<Item = FileMapped<'a, char>> + Clone> WaLexer<T> {
    pub fn parse_ops(mut self) -> crate::Result<Vec<FileMapped<'a, Op>>> {
        let mut ops = vec![];
        'tok: loop {
            match self.advance1() {
                Some((loc, ch)) if ch.is_digit(10) => loop {
                    let start_loc = loc;
                    let (mut n, base) = match self.peek1() {
                        // 0[x,b]<num in diff base>
                        Some((loc, radix)) if ch == '0' && (radix == 'x' || radix == 'b') => {
                            self.advance1();
                            (
                                String::from(
                                    self.advance1()
                                        .ok_or(WaError::UnexpectedEof {
                                            parsing_from: start_loc.into(),
                                            at: loc.into(),
                                            expected: match radix {
                                                'x' => "hexadecimal",
                                                'b' => "binary",
                                                _ => unreachable!(),
                                            }
                                            .to_string(),
                                        })?
                                        .1,
                                ),
                                match radix {
                                    'x' => 16,
                                    'b' => 2,
                                    _ => unreachable!(),
                                },
                            )
                        }

                        Some((_, ch_1)) if ch_1.is_digit(10) => (String::from(ch), 10),
                        Some((loc, ch)) => {
                            return Err(WaError::UnexpectedToken {
                                parsing_from: start_loc.into(),
                                at: loc.into(),
                                expected: "digit".into(),
                                got: ch.clone(),
                            }
                            .splat())
                        }
                        None => break 'tok,
                    };

                    let mut prev_loc = loc;
                    'dig: loop {
                        match self.peek1() {
                            Some((loc, ch)) if ch.is_digit(base) => {
                                self.advance1();
                                n.push(ch);
                                prev_loc = loc;
                            }
                            Some((_, ch)) if ch.is_whitespace() => break 'dig,
                            Some((t, ch)) => {
                                return Err(WaError::UnexpectedToken {
                                    parsing_from: start_loc.into(),
                                    at: t.into(),
                                    expected: "digit".into(),
                                    got: ch.clone(),
                                }
                                .splat());
                            }
                            None => break 'tok,
                        }
                    }
                    ops.push((prev_loc, Op::Push(isize::from_str_radix(&n, base)?)));
                    self.advance1();
                    continue 'tok;
                },
                Some((loc, ch)) if ch == '+' => ops.push((loc, Op::Intrinsic2_1(Op2_1::Add))),
                Some((loc, ch)) if ch == '-' => ops.push((loc, Op::Intrinsic2_1(Op2_1::Sub))),
                Some((loc, ch)) if ch == '*' => ops.push((loc, Op::Intrinsic2_1(Op2_1::Mul))),
                Some((loc, ch)) if ch == '/' => ops.push((loc, Op::Intrinsic2_1(Op2_1::Div))),
                Some((loc, ch)) if ch == '%' => ops.push((loc, Op::Intrinsic2_1(Op2_1::Mod))),
                Some((loc, ch)) if ch == '@' => ops.push((loc, Op::Intrinsic2_2(Op2_2::DivMod))),
                Some((loc, ch)) if ch == '.' => ops.push((loc, Op::Intrinsic1_0(Op1_0::Display))),
                Some((loc, ch)) if ch == '.' => ops.push((loc, Op::Intrinsic1_0(Op1_0::Display))),
                Some((_, ch)) if ch.is_whitespace() => {}
                Some(t) => {
                    todo!("{t:?}");
                }
                None => break,
            }
        }
        Ok(ops)
    }
}
