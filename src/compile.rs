use lang_c::{ast::ExternalDeclaration, driver::Parse, span::Node};
use log;
use thiserror::Error as ThisError;

pub fn compile(parsed: Parse) -> Result<(), Error> {
    let mut ctx = Context::default();
    compile_translation_unit(&mut ctx, parsed.unit.0)
}

/// A context for whole compilation process.
#[derive(Default)]
struct Context {}

#[derive(Debug, ThisError)]
pub enum Error {}

fn compile_translation_unit(
    ctx: &mut Context,
    decls: Vec<Node<ExternalDeclaration>>,
) -> Result<(), Error> {
    log::info!("Skip compilation process since it's unimplemented");
    Ok(())
}
