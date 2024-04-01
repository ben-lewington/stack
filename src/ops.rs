use std::isize;

#[derive(Debug, Clone, Copy)]
pub struct OpIdx(pub usize);

impl OpIdx {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for OpIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Push(isize),
    Intr1_0(Op1_0),
    Intr1_2(Op1_2),
    Intr2_1(Op2_1),
    Intr2_2(Op2_2),
    If(OpIdx),
    End,
}

#[derive(Debug, Clone, Copy)]
pub enum Op1_0 {
    Display,
    Drop,
}

#[derive(Debug, Clone, Copy)]
pub enum Op1_2 {
    Duplicate,
}

#[derive(Debug, Clone, Copy)]
pub enum Op2_1 {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equ,
    Less,
    Greater,
    LessEqu,
    GreaterEqu,
}

#[derive(Debug, Clone, Copy)]
pub enum Op2_2 {
    DivMod,
    Swap,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Push(n) => write!(f, "PUSH {n}"),
            Op::Intr1_0(op_id) => write!(
                f,
                "{}",
                match op_id {
                    Op1_0::Display => "DISPLAY",
                    Op1_0::Drop => "DROP",
                }
            ),
            Op::Intr1_2(op_id) => write!(f, "{}",
                match op_id {
                    Op1_2::Duplicate => "DUP"
                }
            ),
            Op::Intr2_1(op_id) => write!(
                f,
                "{}",
                match op_id {
                    Op2_1::Add => "ADD",
                    Op2_1::Sub => "SUB",
                    Op2_1::Mul => "MUL",
                    Op2_1::Div => "DIV",
                    Op2_1::Mod => "MOD",
                    Op2_1::Equ => "EQ",
                    Op2_1::Less => "LT",
                    Op2_1::Greater => "GT",
                    Op2_1::LessEqu => "LTEQ",
                    Op2_1::GreaterEqu => "GTEQ",
                }
            ),
            Op::Intr2_2(op_id) => write!(
                f,
                "{}",
                match op_id {
                    Op2_2::DivMod => "DIVMOD",
                    Op2_2::Swap => "SWAP",
                }
            ),
            Op::If(jmp_idx) => write!(f, "IF => {jmp_idx}"),
            Op::End => write!(f, "END"),
        }
    }
}

impl Op1_0 {
    pub fn into_op(self) -> crate::stack::VirtStackOp<1, 0> {
        match self {
            Op1_0::Display => |[t]| {
                println!("{t}");
                []
            },
            Op1_0::Drop => |[_]| [],
        }
    }
}

impl Op1_2 {
    pub fn into_op(self) -> crate::stack::VirtStackOp<1, 2> {
        match self {
            Op1_2::Duplicate => |[t]| [t, t],
        }
    }
}

impl Op2_1 {
    pub fn into_op(self) -> crate::stack::VirtStackOp<2, 1> {
        match self {
            Op2_1::Add => |[t, t1]| [t + t1],
            Op2_1::Sub => |[t, t1]| [t - t1],
            Op2_1::Mul => |[t, t1]| [t * t1],
            Op2_1::Div => |[t, t1]| [t / t1],
            Op2_1::Mod => |[t, t1]| [t % t1],
            Op2_1::Equ => |[t, t1]| [(t == t1) as isize],
            Op2_1::Less => |[t, t1]| [(t < t1) as isize],
            Op2_1::Greater => |[t, t1]| [(t > t1) as isize],
            Op2_1::LessEqu => |[t, t1]| [(t <= t1) as isize],
            Op2_1::GreaterEqu => |[t, t1]| [(t >= t1) as isize],
        }
    }
}

impl Op2_2 {
    pub fn into_op(self) -> crate::stack::VirtStackOp<2, 2> {
        match self {
            Op2_2::DivMod => |[t, t1]| [t / t1, t % t1],
            Op2_2::Swap => |[t, t1]| [t1, t],
        }
    }
}
