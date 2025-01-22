use crate::{Break, Stream};

pub trait Unparse {
    fn unparse(&self) -> Stream {
        let mut s = Stream::new();
        self.unparse_into(&mut s);
        s
    }

    fn unparse_into(&self, s: &mut Stream);
}

impl Unparse for str {
    fn unparse_into(&self, s: &mut Stream) {
        s.write_string(self.to_string())
    }
}

impl Unparse for String {
    fn unparse_into(&self, s: &mut Stream) {
        s.write_string(self.clone())
    }
}

impl Unparse for Break {
    fn unparse_into(&self, s: &mut Stream) {
        s.add_break()
    }
}

impl<T> Unparse for Box<T>
where
    T: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(self.as_ref())
    }
}
