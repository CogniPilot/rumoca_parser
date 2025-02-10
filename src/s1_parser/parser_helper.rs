//! This module contains parser helper functions.
//!

use crate::s0_lexer::lexer::Lexer;
use crate::s0_lexer::tokens::LexicalError;

use crate::s1_parser::ast::node;
use crate::s1_parser::modelica::StoredDefinitionParser;

use super::ast::part::ParserContext;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use lalrpop_util::ParseError;
use md5;
use std::process;

pub fn parse_file(filename: &str) -> node::StoredDefinition {
    let file_txt = std::fs::read_to_string(filename).expect("failed to read file");
    parse(filename, &file_txt)
}

pub fn parse(filename: &str, file_txt: &str) -> node::StoredDefinition {
    let mut files = SimpleFiles::new();
    let file_id = files.add(filename, file_txt);
    let file = files.get(file_id).expect("failed to get file id");
    let file_txt = file.source();
    let lexer = Lexer::new(file_txt);
    let parser = StoredDefinitionParser::new();
    let mut context = ParserContext::default();
    let result = parser.parse(&mut context, lexer);
    if result.is_err() {
        let err = result.as_ref().expect_err("error");
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        match err {
            ParseError::User { error } => match error {
                LexicalError::InvalidInteger(e) => {
                    println!("lexer invalid integer:{:?}", e);
                }
                LexicalError::InvalidToken => {
                    println!("lexer invalid token {:?}", error);
                }
            },
            ParseError::InvalidToken { location } => {
                println!("invalid token loc:{:?}", location);
            }
            ParseError::ExtraToken { token } => {
                println!("extra token: {:?}", token);
            }
            ParseError::UnrecognizedEof { location, expected } => {
                println!("unrecognized Eof loc: {:?}, expected:", location);
                for tok in expected {
                    println!("{:?}", tok)
                }
            }
            ParseError::UnrecognizedToken { token, expected } => {
                let diagonistic = Diagnostic::error()
                    .with_message("Unrecognized Token")
                    .with_code("E001")
                    .with_labels(vec![
                        Label::primary(file_id, (token.0)..(token.2)),
                        Label::secondary(file_id, 0..(token.2)),
                    ])
                    .with_notes(vec!["expected one of: ".to_string(), expected.join(", ")]);
                codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagonistic)
                    .expect("fail");
            }
        }

        // kill process to avoid panicing when parse fails, codespan already reports error
        process::exit(1);
    }
    let mut def = result.unwrap();
    let digest = md5::compute(file_txt);
    def.model_md5 = format!("{:x}", digest);
    def.rumoca_parser_version = env!("CARGO_PKG_VERSION").to_string();
    def.rumoca_parser_git = option_env!("GIT_VER").unwrap_or("").to_string();
    def
}
