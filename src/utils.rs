#[derive(Debug, Clone)]
pub struct Descend<I, T: Iterator<Item = I> + Clone>(pub T);

pub enum Chunk<const N: usize, I> {
    NoneOf,
    SomeOf(usize, [I; N]),
    AllOf([I; N]),
}

impl<I: Copy, T: Iterator<Item = I> + Clone> Descend<I, T> {
    pub fn peek_opt<const N: usize>(&self) -> Chunk<N, Option<I>> {
        let mut its = self.0.clone();
        let mut res = [None; N];
        for i in 0..N {
            let Some(it) = its.next() else {
                if i == 0 {
                    return Chunk::NoneOf;
                }
                return Chunk::SomeOf(i, res);
            };
            let r = unsafe { res.get_unchecked_mut(N - 1 - i) };
            *r = Some(it);
        }
        Chunk::AllOf(res)
    }

    pub fn chop_opt<const N: usize>(&mut self) -> Chunk<N, Option<I>> {
        let its = &mut self.0;
        let mut res = [None; N];
        for i in 0..N {
            let Some(it) = its.next() else {
                if i == 0 {
                    return Chunk::NoneOf;
                }
                return Chunk::SomeOf(i, res);
            };
            let r = unsafe { res.get_unchecked_mut(N - 1 - i) };
            *r = Some(it);
        }
        Chunk::AllOf(res)
    }
}

impl<I: Copy + Default, T: Iterator<Item = I> + Clone> Descend<I, T> {
    pub fn peek<const N: usize>(&self) -> Chunk<N, I> {
        let mut its = self.0.clone();
        let mut res = [I::default(); N];
        for i in 0..N {
            let Some(it) = its.next() else {
                return if i == 0 {
                    Chunk::NoneOf
                } else {
                    Chunk::SomeOf(i, res)
                };
            };
            let r = unsafe { res.get_unchecked_mut(N - 1 - i) };
            *r = it;
        }
        Chunk::AllOf(res)
    }

    pub fn chop<const N: usize>(&self) -> Chunk<N, I> {
        let mut its = self.0.clone();
        let mut res = [I::default(); N];
        for i in 0..N {
            let Some(it) = its.next() else {
                return if i == 0 {
                    Chunk::NoneOf
                } else {
                    Chunk::SomeOf(i, res)
                };
            };
            let r = unsafe { res.get_unchecked_mut(N - 1 - i) };
            *r = it;
        }
        Chunk::AllOf(res)
    }
}

impl<I, T: Iterator<Item = I> + Clone> Descend<I, T> {
    pub fn peek1(&self) -> Option<I> {
        let mut i = self.0.clone();
        i.next()
    }

    pub fn advance1(&mut self) -> Option<I> {
        self.0.next()
    }
}
