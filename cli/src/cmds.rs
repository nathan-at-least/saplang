use crate::{Result, SourceOption};

pub fn eval(source: &SourceOption) -> Result<()> {
    let x = sappho_interpreter::interpret(source)?;
    println!("{:#?}", x);
    Ok(())
}

pub fn parse(source: &SourceOption) -> Result<()> {
    let x = sappho_parser::parse(source)?;
    println!("Parsed: {:#?}", x);
    Ok(())
}

pub fn canonicalize(source: &SourceOption) -> Result<()> {
    let x = sappho_parser::parse(source)?;
    println!("{}", x);
    Ok(())
}

pub fn elemental(source: &SourceOption) -> Result<()> {
    let x = sappho_parser::parse(source)?;
    let y = sappho_east::PureExpr::from(x);
    println!("{}", y);
    Ok(())
}
