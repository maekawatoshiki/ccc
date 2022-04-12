use lang_c::{ast::ExternalDeclaration, driver::Parse, span::Node};
use log;

pub fn compile(parsed: Parse) -> Result<(), ()> {
    let mut ctx = Context::default();
    compile_translation_unit(&mut ctx, parsed.unit.0)
}

/// A context for whole compilation process.
#[derive(Default)]
struct Context {}

fn compile_translation_unit(
    ctx: &mut Context,
    decls: Vec<Node<ExternalDeclaration>>,
) -> Result<(), ()> {
    log::info!("Skip compilation process since it's unimplemented");
    Ok(())
}
