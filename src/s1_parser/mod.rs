pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[rustfmt::skip]
    pub modelica,
    "/s1_parser/modelica.rs"
);
