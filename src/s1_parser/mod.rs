pub mod ast;
pub mod parser_helper;
pub use parser_helper::parse_file;
pub mod print_visitor;
pub mod visitor;
pub mod walker;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[rustfmt::skip]
    modelica,
    "/s1_parser/modelica.rs"
);
