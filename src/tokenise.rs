#[derive(Debug, Clone, Copy, Default)]
pub struct TokenIdx {
    pub row: usize,
    pub col: usize,
}

impl TokenIdx {
    fn tick_col(&mut self) {
        self.col += 1;
    }

    fn tick_row(&mut self) {
        self.row += 1;
        self.col = 0;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span<T> {
    pub idx: TokenIdx,
    pub token: T,
}

#[derive(Debug, Clone)]
pub struct Tokeniser<'a> {
    cur_tok_id: TokenIdx,
    file_contents: &'a [u8],
}

impl<'a> Iterator for Tokeniser<'a> {
    type Item = Span<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_whitespace()?;
        let t = self.cur_tok_id;
        self.split_by_predicate(|ch| !ch.is_ascii_whitespace())
            .map(|token| Span {
                idx: t,
                token: std::str::from_utf8(token).expect("Non Utf-8 chars"),
            })
    }
}

impl<'a> Tokeniser<'a> {
    pub fn new(file_contents: &'a [u8]) -> Self {
        Self {
            file_contents,
            cur_tok_id: Default::default(),
        }
    }

    pub fn trim_whitespace(&mut self) -> Option<()> {
        let mut i = 0;
        let l = self.file_contents.len();
        while i < l {
            match unsafe { self.file_contents.get_unchecked(i) } {
                b'\n' => self.cur_tok_id.tick_row(),
                c if c.is_ascii_whitespace() => self.cur_tok_id.tick_col(),
                _ => break,
            }
            println!("{}", self.cur_tok_id);
            i += 1;
        }
        self.file_contents = self.file_contents.split_at(i).1;
        if i == l {
            return None;
        }
        Some(())
    }

    pub fn split_by_predicate(&mut self, predicate: impl Fn(u8) -> bool) -> Option<&'a [u8]> {
        let l = self.file_contents.len();
        if l == 0 {
            return None;
        };
        let mut i = 0;
        while i < self.file_contents.len() {
            match unsafe { *self.file_contents.get_unchecked(i) } {
                ch if !predicate(ch) => break,
                b'\n' => self.cur_tok_id.tick_row(),
                _ => self.cur_tok_id.tick_col(),
            }
            i += 1;
        }
        let (tok, rest) = self.file_contents.split_at(i);
        self.file_contents = rest;
        Some(tok)
    }
}

impl TokenIdx {
    pub fn as_stamp(self, file_name: impl AsRef<str>) -> String {
        format!("{}:{}", file_name.as_ref(), self)
    }
}

impl std::fmt::Display for TokenIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.col + 1)
    }
}
