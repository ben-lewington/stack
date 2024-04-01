use crate::tokenise::Span;

#[derive(Debug)]
pub struct Stack<T>(Vec<T>);

pub type VirtStackOp<const IN: usize, const OUT: usize, T = isize> = fn([T; IN]) -> [T; OUT];

impl<T: std::fmt::Display> std::fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-----\\/ STACK\n")?;
        for (i, n) in self.0.iter().rev().enumerate() {
            write!(f, "@{:5}|= {}\n", i, n)?;
        }
        write!(f, "-----/\\\n")?;
        write!(f, "{}", 1)
    }
}

pub trait StackOp<const IN: usize, const OUT: usize, T> {
    fn op(self, input: [T; IN]) -> [T; OUT];
}

impl<T: Copy + Default> Stack<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push<const N: usize>(&mut self, values: [T; N]) {
        for i in 0..N {
            self.0.push(unsafe { *values.get_unchecked(N - i - 1) });
        }
    }

    pub fn pop<const N: usize>(
        &mut self,
        Span { idx, token }: Span<&str>,
    ) -> anyhow::Result<[T; N]> {
        let mut ret = [T::default(); N];
        for i in 0..N {
            let ret = unsafe { ret.get_unchecked_mut(i) };

            *ret = self.0.pop().ok_or(anyhow::anyhow!(
                "{}: Stack Underflow, expected at least {} element(s), got {i}",
                idx.as_stamp(token),
                N,
            ))?;
        }
        Ok(ret)
    }

    pub fn run<const IN: usize, const OUT: usize>(
        &mut self,
        stack_op: impl StackOp<IN, OUT, T>,
        at: Span<&str>,
    ) -> anyhow::Result<()> {
        let s = self.pop::<IN>(at)?;
        self.push(stack_op.op(s));
        Ok(())
    }
}

impl<const IN: usize, const OUT: usize, T, F> StackOp<IN, OUT, T> for F
where
    F: FnOnce([T; IN]) -> [T; OUT],
{
    fn op(self, input: [T; IN]) -> [T; OUT] {
        (self)(input)
    }
}
