use std::path::Path;

use oxc_allocator::Allocator;
use oxc_ast::{ast::Argument, ast::Expression, AstKind, Visit};
use oxc_parser::Parser;
use oxc_span::{GetSpan, SourceType};

use crate::logger::Logger;

pub fn extract_imports(path: &Path, panic_on_dynamic_errors: bool) -> Vec<String> {
    let file_name = path.to_str().unwrap();
    let source_text =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("{file_name} not found"));
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).unwrap();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    for error in ret.errors {
        let error = error.with_source_code(source_text.clone());
        Logger::error(&format!("{error:?}"));
    }

    let program = ret.program;

    let mut ast_pass = ASTPass {
        file_path: path.to_str().unwrap().to_string(),
        panic_on_dynamic_errors,
        ..Default::default()
    };
    ast_pass.visit_program(&program);

    ast_pass.package_names
}

#[derive(Debug, Default)]
struct ASTPass {
    file_path: String,
    panic_on_dynamic_errors: bool,
    package_names: Vec<String>,
}

impl<'a> Visit<'a> for ASTPass {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::ImportDeclaration(ast) => {
                // Handles all static imports
                let package_name = ast.source.value.to_string();
                self.package_names.push(package_name);
            }
            AstKind::ImportExpression(ast) => {
                // Handles dynamic imports, but only with a single string input
                match &ast.source {
                    Expression::StringLiteral(ast) => {
                        let package_name = ast.value.to_string();
                        self.package_names.push(package_name)
                    }
                    ast => {
                        let output = format!(
                            "Unsupported ImportExpression value in {} from chars {} to {}\n\
                            Are you using dynamic package names in your import() calls?",
                            self.file_path,
                            ast.span().start,
                            ast.span().end,
                        );
                        if self.panic_on_dynamic_errors {
                            Logger::error(&output);
                            panic!("Unexpected error, see logs.")
                        } else {
                            Logger::warn(&output);
                        }
                    }
                }
            }
            AstKind::CallExpression(ast) => {
                // Handles require calls, but only with a single string input
                if ast.is_require_call() {
                    match &ast.arguments[0] {
                        Argument::Expression(Expression::StringLiteral(ast)) => {
                            let package_name = ast.value.to_string();
                            self.package_names.push(package_name);
                        }
                        ast => {
                            // this actually won't happen because of the `is_require_call` check
                            // above which asserts that there is only one StringLiteral argument
                            // however, it's good to have a catch-all in case the parser changes
                            let output = format!(
                                "Unsupported require Expression value in {} from chars {} to {}\n\
                            Are you using dynamic package names in your require() calls?",
                                self.file_path,
                                ast.span().start,
                                ast.span().end,
                            );
                            if self.panic_on_dynamic_errors {
                                Logger::error(&output);
                                panic!("Unexpected error, see logs.")
                            } else {
                                Logger::warn(&output);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
