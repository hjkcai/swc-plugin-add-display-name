use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};

pub struct HasJSXVisitor {
    has_jsx: bool,
}

impl HasJSXVisitor {
    pub fn test(node: &mut impl VisitMutWith<Self>) -> bool {
        let mut visitor = HasJSXVisitor { has_jsx: false };
        node.visit_mut_children_with(&mut visitor);
        visitor.has_jsx
    }
}

impl VisitMut for HasJSXVisitor {
    fn visit_mut_jsx_element(&mut self, el: &mut JSXElement) {
        el.visit_mut_children_with(self);
        self.has_jsx = true;
    }

    fn visit_mut_jsx_fragment(&mut self, el: &mut JSXFragment) {
        el.visit_mut_children_with(self);
        self.has_jsx = true;
    }

    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        // Check if this is a React JSX runtime call or createElement call
        if self.is_react_call(call_expr) {
            self.has_jsx = true;
        }
    }
}

impl HasJSXVisitor {
    fn is_react_call(&self, call_expr: &CallExpr) -> bool {
        match &call_expr.callee {
            Callee::Expr(expr) => match &**expr {
                // JSX Runtime functions: jsx("div", { children: "..." })
                // Legacy createElement: createElement("div", { ... })
                Expr::Ident(ident) => {
                    matches!(ident.sym.as_ref(), "jsx" | "jsxs" | "_jsx" | "_jsxs" | "jsxDEV" | "_jsxDEV" | "createElement")
                }

                // Namespace calls: React.createElement("div", { ... })
                Expr::Member(member_expr) => {
                    if let MemberProp::Ident(prop_ident) = &member_expr.prop {
                        matches!(prop_ident.sym.as_ref(), "jsx" | "jsxs" | "_jsx" | "_jsxs" | "jsxDEV" | "_jsxDEV" | "createElement")
                    } else {
                        false
                    }
                }

                _ => false,
            },
            Callee::Super(_) | Callee::Import(_) => false,
        }
    }
}
