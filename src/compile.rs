use lang_c::{
    ast::{ExternalDeclaration, FunctionDefinition},
    driver::Parse,
    span::Node,
};
use log;
use thiserror::Error as ThisError;

pub fn compile(parsed: &Parse) -> Result<(), Error> {
    let mut ctx = Context::default();
    compile_translation_unit(&mut ctx, &parsed.unit.0)
}

/// A context for whole compilation process.
#[derive(Default)]
struct Context {}

#[derive(Debug, ThisError)]
pub enum Error {}

fn compile_translation_unit(
    ctx: &mut Context,
    decls: &Vec<Node<ExternalDeclaration>>,
) -> Result<(), Error> {
    for decl in decls {
        match &decl.node {
            ExternalDeclaration::FunctionDefinition(func) => compile_func_decl(ctx, func)?,
            ExternalDeclaration::Declaration(_) => log::info!("Todo: declaration"),
            ExternalDeclaration::StaticAssert(_) => log::info!("Todo: static assert"),
        }
    }

    Ok(())
}

fn compile_func_decl(_ctx: &mut Context, func: &Node<FunctionDefinition>) -> Result<(), Error> {
    log::info!("Todo: skipping function");
    Ok(())
}
