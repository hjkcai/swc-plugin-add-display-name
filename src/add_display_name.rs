use super::has_jsx::HasJSXVisitor;
use swc_core::common::{DUMMY_SP, SyntaxContext, Span};
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{VisitMut, VisitMutWith},
};

pub struct AddDisplayNameVisitor;

impl VisitMut for AddDisplayNameVisitor {
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        let mut a: Vec<(usize, JsWord, SyntaxContext)> = Vec::new();
        stmts.iter_mut().enumerate().for_each(|(i, stmt)| {
            let mut export_var_decl = || {
                let var_decls = stmt.as_mut_module_decl()?.as_mut_export_decl()?.decl.as_mut_var()?.as_mut();
                if var_decls.decls.len() != 1 { return None };

                let var_decl = &mut var_decls.decls[0];
                let has_jsx = HasJSXVisitor::test(var_decl);
                if !has_jsx { return None };

                let name = &var_decl.name.as_ident()?.id;
                Some((i, name.sym.clone(), name.span.ctxt))
            };

            if let Some(result) = export_var_decl() { a.push(result) }
        });

        a.iter().for_each(|(i, name, ctxt)| {
            stmts.insert(*i + 1, ModuleItem::Stmt(set_display_name_stmt(name, ctxt)));
        })
    }
}

fn set_display_name_stmt(target: &JsWord, ctxt: &SyntaxContext) -> Stmt {
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(target.clone(), Span { ctxt: *ctxt, ..DUMMY_SP }))),
                prop: MemberProp::Ident(Ident::new(JsWord::from("displayName").into(), DUMMY_SP))
            }))),
            right: Box::new(Expr::Lit(Lit::Str(Str::from(target.clone()))))
        }))
    })
}
