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

impl<'a> Visitor<'a> for PrintVisitor {
    fn enter_any(&mut self) {
        self.level += 1;
    }

    fn exit_any(&mut self) {
        self.level -= 1;
    }

    fn enter_stored_definition(&mut self, _def: &'a ast::StoredDefinition) {
        self.print("Stored Definition");
    }

    fn enter_class_definition(&mut self, class: &'a ast::ClassDefinition) {
        if let ast::ClassSpecifier::Long { name, .. } = &class.specifier {
            self.print(&format!("class {}", name));
        }
    }

    fn exit_class_definition(&mut self, _class: &'a ast::ClassDefinition) {
        println!("\n");
    }

    fn enter_expression(&mut self, expr: &'a ast::Expression) {
        match expr {
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
                self.print(&format!("{:?}", val));
            }
            ast::Expression::UnsignedReal(val) => {
                self.print(&format!("{:?}", val));
            }
            ast::Expression::Boolean(val) => {
                self.print(&format!("{:?}", val));
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

    fn enter_equation(&mut self, eq: &'a ast::Equation) {
        match eq {
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

    fn exit_element(&mut self, elem: &'a ast::Element) {
        match elem {
            ast::Element::ComponentClause { clause, .. } => {
                for comp in clause.components.iter() {
                    self.print(&format!("component: {}", comp.declaration.name));
                }
            }
            ast::Element::ImportClause { name, .. } => {
                self.print(&format!("import: {}", name.ident.join(".")));
            }
            ast::Element::ClassDefinition { .. } => {}
            ast::Element::ExtendsClause { .. } => {}
        }
    }

    fn exit_component_reference(&mut self, comp: &'a ast::ComponentReference) {
        let mut s: String = "".to_string();
        for (index, part) in comp.parts.iter().enumerate() {
            if index != 0 || comp.local {
                s += ".";
            }
            s += &part.name;
        }
        self.print(&s);
    }
}
