#[derive(Debug, Clone, Copy)]
pub struct RawToken<'a> {
    pub row: usize,
    pub col: usize,
    pub token: &'a str,
}

#[derive(Debug, Clone)]
pub struct WaTokenizer<'a> {
    file_contents: &'a [u8],
    row: usize,
    col: usize,
}

impl<'a> WaTokenizer<'a> {
    pub fn new(file_name: &'a str, file_contents: &'a [u8]) -> Self {
        Self {
            file_contents,
            row: 0,
            col: 0,
        }
    }

    pub fn trim_whitespace(&mut self) -> Option<()> {
        let mut i = 0;
        let l = self.file_contents.len();
        while i < l {
            match unsafe { self.file_contents.get_unchecked(i) } {
                b'\n' => {
                    self.col = 0;
                    self.row += 1;
                }
                c if c.is_ascii_whitespace() => self.col += 1,
                _ => break,
            }
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
                b'\n' => {
                    self.col = 0;
                    self.row += 1;
                }
                _ => self.col += 1,
            }
            i += 1;
        }
        let (tok, rest) = self.file_contents.split_at(i);
        self.file_contents = rest;
        Some(tok)
    }
}

impl<'a> Iterator for WaTokenizer<'a> {
    type Item = RawToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(_) = self.trim_whitespace() else {
            return None;
        };
        let row = self.row;
        let col = self.col;
        let t = self.split_by_predicate(|ch| !ch.is_ascii_whitespace());
        let Some(token) = t else {
            return None;
        };
        Some(RawToken {
            row,
            col,
            token: std::str::from_utf8(token).expect("Non ASCII chars"),
        })
    }
}
