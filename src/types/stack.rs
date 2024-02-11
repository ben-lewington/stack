use crate::types::Arity;

use super::FileLoc;

pub struct RunStack(Vec<isize>);

impl RunStack {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push<const N: usize>(&mut self, values: [isize; N]) {
        for i in 0..N {
            self.0.push(unsafe { *values.get_unchecked(N - i - 1) });
        }
    }

    pub fn pop<'a, const N: usize>(&mut self, at: super::FileLoc<'a>) -> crate::Result<[isize; N]> {
        let mut ret = [0; N];
        for i in 0..N {
            let ret = unsafe { ret.get_unchecked_mut(i) };

            *ret = self.0.pop().ok_or(
                super::WaError::StackUnderflow {
                    at: at.into(),
                    expected: N as isize,
                    got: i as isize,
                }
                .splat(),
            )?;
        }
        Ok(ret)
    }

    // pub fn iter(&self) -> impl Iterator<Item = isize> {
    //     self.0.clone().into_iter()
    // }

    // pub fn into_iter(self) -> impl Iterator<Item = isize> {
    //     self.0.into_iter()
    // }
}

pub trait StackOp<const IN: usize, const OUT: usize> {
    fn op(&self, input: [isize; IN]) -> [isize; OUT];

    fn run_op(&self, stack: &mut RunStack, at: FileLoc<'_>) -> crate::Result<()> {
        let s = stack.pop::<IN>(at)?;
        stack.push(self.op(s));
        Ok(())
    }
}
impl<const IN: usize, const OUT: usize> StackOp<IN, OUT> for Arity<IN, OUT> {
    fn op(&self, input: [isize; IN]) -> [isize; OUT] {
        (self)(input)
    }
}
