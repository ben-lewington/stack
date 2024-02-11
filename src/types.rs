pub(crate) mod stack;

#[derive(Debug, thiserror::Error)]
pub enum WaError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{at}: Expected \"{expected}\", got {got}. While parsing token at {parsing_from}:")]
    UnexpectedToken {
        parsing_from: FileStamp,
        at: FileStamp,
        expected: String,
        got: char,
    },
    #[error("{at}: Expected \"{expected}\", got EOF. While parsing token at {parsing_from}:")]
    UnexpectedEof {
        parsing_from: FileStamp,
        at: FileStamp,
        expected: String,
    },
    #[error("{at}: required {expected} elements on the stack, got {got}")]
    StackUnderflow {
        at: FileStamp,
        expected: isize,
        got: isize,
    },
    #[error("{0}")]
    Err(String),
}

#[derive(Debug, Clone, Copy)]
pub struct FileLoc<'a> {
    pub file_name: &'a str,
    pub at_char: isize,
    pub row: isize,
    pub col: isize,
}

pub type FileMapped<'a, T> = (FileLoc<'a>, T);

#[derive(Debug, Clone)]
pub struct FileStamp(String);

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Push(isize),
    Intrinsic1_0(Op1_0),
    Intrinsic2_1(Op2_1),
    Intrinsic2_2(Op2_2),
}

type Arity<const IN: usize, const OUT: usize> = fn([isize; IN]) -> [isize; OUT];

#[derive(Debug, Clone, Copy)]
pub enum Op1_0 {
    Display,
}

#[derive(Debug, Clone, Copy)]
pub enum Op2_1 {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy)]
pub enum Op2_2 {
    DivMod,
}

impl Op1_0 {
    pub fn into_op(self) -> Arity<1, 0> {
        match self {
            Op1_0::Display => |[t]| {
                println!("{t}");
                []
            },
        }
    }
}

impl Op2_1 {
    pub fn into_op(self) -> Arity<2, 1> {
        match self {
            Op2_1::Add => |[t, t1]| [t + t1],
            Op2_1::Sub => |[t, t1]| [t - t1],
            Op2_1::Mul => |[t, t1]| [t * t1],
            Op2_1::Div => |[t, t1]| [t / t1],
            Op2_1::Mod => |[t, t1]| [t % t1],
        }
    }
}

impl Op2_2 {
    pub fn into_op(self) -> Arity<2, 2> {
        match self {
            Op2_2::DivMod => |[t, t1]| [t / t1, t % t1],
        }
    }
}

impl<'a> From<FileLoc<'a>> for FileStamp {
    fn from(loc: FileLoc<'_>) -> Self {
        Self(format!("{}:{}:{}", loc.file_name, loc.row + 1, loc.col + 1))
    }
}

impl std::fmt::Display for FileStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Push(n) => write!(f, "PUSH {n}"),
            Op::Intrinsic2_1(op_id) => write!(
                f,
                "{}",
                match op_id {
                    Op2_1::Add => "ADD",
                    Op2_1::Sub => "SUB",
                    Op2_1::Mul => "MUL",
                    Op2_1::Div => "DIV",
                    Op2_1::Mod => "MOD",
                }
            ),
            Op::Intrinsic2_2(op_id) => write!(
                f,
                "{}",
                match op_id {
                    Op2_2::DivMod => "DIVMOD",
                }
            ),
            Op::Intrinsic1_0(op_id) => write!(
                f,
                "{}",
                match op_id {
                    Op1_0::Display => "DISPLAY",
                }
            ),
        }
    }
}

impl WaError {
    pub fn splat(self) -> Self {
        Self::Err(self.to_string())
    }
}

impl From<&str> for WaError {
    fn from(value: &str) -> Self {
        Self::Err(value.into())
    }
}
