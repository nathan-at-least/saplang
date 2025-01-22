use super::Item::{self, *};
use crate::Stream;
use std::fmt;

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_stream(self, f)
    }
}

fn fmt_stream(stream: &Stream, f: &mut fmt::Formatter) -> fmt::Result {
    for item in &stream.items[..] {
        fmt_item(item, f, stream.depth)?;
    }
    Ok(())
}

fn fmt_item(item: &Item, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    match item {
        Leaf(s) => f.write_str(s),
        Break => {
            f.write_str("\n")?;
            for _ in 0..depth {
                f.write_str("  ")?;
            }
            Ok(())
        }
        Substream(sub) => fmt_stream(sub, f),
    }
}
