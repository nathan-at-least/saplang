mod display;

use crate::{Break, Unparse};

#[derive(Debug)]
pub struct Stream {
    items: Vec<Item>,
    depth: usize,
}

#[derive(Debug)]
enum Item {
    Leaf(String),
    Break,
    Substream(Stream),
}
use Item::*;

#[derive(Debug)]
pub enum Brackets {
    Parens,
    Square,
    Squiggle,
}

impl Stream {
    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn write<U>(&mut self, thing: &U)
    where
        U: Unparse + ?Sized,
    {
        thing.unparse_into(self)
    }

    pub fn bracketed<F>(&mut self, brackets: Brackets, f: F)
    where
        F: FnOnce(&mut Stream),
    {
        use Brackets::*;
        use Break::{Mandatory, Opt};

        let (brk, open, close) = match brackets {
            Parens => (Opt, "(", ")"),
            Square => (Opt, "[", "]"),
            Squiggle => (Mandatory, "{", "}"),
        };

        self.write(open);
        // self.write(&brk);
        self.substream(f);
        self.write(&brk);
        self.write(close);
    }

    fn substream<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Stream),
    {
        let mut sub = Stream {
            items: vec![],
            depth: self.depth + 1,
        };
        f(&mut sub);
        self.items.push(Substream(sub));
    }

    pub(crate) fn new() -> Self {
        Stream {
            items: vec![],
            depth: 0,
        }
    }

    pub(crate) fn write_string(&mut self, s: String) {
        self.items.push(Leaf(s))
    }

    pub(crate) fn add_break(&mut self) {
        self.items.push(Break);
    }
}
