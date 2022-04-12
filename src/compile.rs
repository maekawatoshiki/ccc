use lang_c::{
    ast::{Declarator, DeclaratorKind, ExternalDeclaration, FunctionDefinition, Identifier},
    driver::Parse,
    span::Node,
};
use log;
use thiserror::Error as ThisError;

pub fn compile(parse: &Parse) -> Result<(), Error> {
    let mut ctx = Context::default();
    compile_translation_unit(&mut ctx, &parse.unit.0)
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
    log::info!("Node: {:#?}", decls);

    for decl in decls {
        match &decl.node {
            ExternalDeclaration::FunctionDefinition(func) => compile_func_decl(ctx, func)?,
            ExternalDeclaration::Declaration(_) => log::info!("Todo: declaration"),
            ExternalDeclaration::StaticAssert(_) => log::info!("Todo: static assert"),
        }
    }

    Ok(())
}

fn compile_func_decl(
    _ctx: &mut Context,
    Node { node: func, .. }: &Node<FunctionDefinition>,
) -> Result<(), Error> {
    let name = expect_ident(&func.declarator.node).unwrap();

    log::info!("Todo: skipping function '{}'", name);

    Ok(())
}

fn expect_ident(declarator: &Declarator) -> Option<&String> {
    match &declarator.kind.node {
        DeclaratorKind::Identifier(Node {
            node: Identifier { name },
            ..
        }) => Some(name),
        _ => None,
    }
}
