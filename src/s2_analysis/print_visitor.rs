use crate::s1_parser::ast;
use crate::s2_analysis::visitor::Visitor;

pub struct PrintVisitor {
    level: usize,
    indent: String,
}

impl PrintVisitor {
    pub fn print(&self, s: &str) {
        println!("{} {}", self.indent.repeat(self.level - 1), s);
    }
}

impl Default for PrintVisitor {
    fn default() -> Self {
        PrintVisitor {
            level: 0,
            indent: "  ".to_string(),
        }
    }
}

impl Visitor for PrintVisitor {
    fn enter_any(&mut self) {
        self.level += 1;
    }

    fn exit_any(&mut self) {
        self.level -= 1;
    }

    fn enter_stored_definition(&mut self, _def: &mut ast::StoredDefinition) {
        self.print("Stored Definition");
    }

    fn enter_class_definition(&mut self, def: &mut ast::ClassDefinition) {
        if let ast::ClassSpecifier::Long { name, .. } = &def.specifier {
            self.print(&format!("class {}", name));
        }
    }

    fn exit_class_definition(&mut self, _def: &mut ast::ClassDefinition) {
        println!("\n");
    }

    fn enter_expression(&mut self, def: &mut ast::Expression) {
        match def {
            ast::Expression::Binary { op, .. } => {
                self.print(&format!("{:?}", op));
            }
            ast::Expression::Unary { op, .. } => {
                self.print(&format!("{:?}", op));
            }
            ast::Expression::Ref { .. } => {
                self.print("ref");
            }
            ast::Expression::UnsignedInteger(val) => {
                self.print(&val.to_string());
            }
            ast::Expression::UnsignedReal(val) => {
                self.print(&val.to_string());
            }
            ast::Expression::Boolean(val) => {
                self.print(&format!("{}", val));
            }
            ast::Expression::If { .. } => {
                self.print("if");
            }
            ast::Expression::ArrayArguments { .. } => {
                self.print("array_args");
            }
            ast::Expression::FunctionCall { .. } => {
                self.print("function call");
            }
            ast::Expression::Der { .. } => {
                self.print("der");
            }
        }
    }

    fn enter_equation(&mut self, def: &mut ast::Equation) {
        match def {
            ast::Equation::Connect { .. } => {
                self.print("connect");
            }
            ast::Equation::Simple { .. } => self.print("equation"),
            ast::Equation::If { .. } => {
                self.print("if");
            }
            ast::Equation::For { .. } => {
                self.print("for");
            }
        }
    }

    fn enter_component_declaration(&mut self, def: &mut ast::ComponentDeclaration) {
        self.print(&format!("component: {}", def.declaration.name));
    }

    fn exit_component_reference(&mut self, def: &mut ast::ComponentReference) {
        let mut s: String = "".to_string();
        for (index, part) in def.parts.iter().enumerate() {
            if index != 0 || def.local {
                s += ".";
            }
            s += &part.name;
        }
        self.print(&s);
    }
}
