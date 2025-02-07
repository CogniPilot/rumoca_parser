pub mod ast;
pub mod parser_helper;
pub use ast::NodeData;
pub use parser_helper::{parse, parse_file};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[rustfmt::skip]
    pub modelica,
    "/s1_parser/modelica.rs"
);
