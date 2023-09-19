use super::has_jsx::HasJSXVisitor;
use swc_core::common::{DUMMY_SP, SyntaxContext, Span};
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{VisitMut, VisitMutWith},
};

struct Component {
    pos: usize,
    name: JsWord,
    ctx: SyntaxContext,
}

impl Component {
    fn with_pos(self, pos: usize) -> Component {
        Component { pos, ..self }
    }
}

impl Component {
    pub fn create_display_name_stmt(&self) -> ModuleItem {
        ModuleItem::Stmt(
            Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Assign(AssignExpr {
                    span: DUMMY_SP,
                    op: AssignOp::Assign,
                    left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::Ident(Ident::new(self.name.clone(), Span { ctxt: self.ctx, ..DUMMY_SP }))),
                        prop: MemberProp::Ident(Ident::new(JsWord::from("displayName").into(), DUMMY_SP))
                    }))),
                    right: Box::new(Expr::Lit(Lit::Str(Str::from(self.name.clone()))))
                }))
            })
        )
    }
}

#[derive(Default)]
pub struct AddDisplayNameVisitor {
    components: Vec<Component>,
}

impl VisitMut for AddDisplayNameVisitor {
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        self.components.iter().enumerate().for_each(|(i, comp)| {
            let index = i + comp.pos + 1;
            if index < stmts.len() {
                stmts.insert(index, comp.create_display_name_stmt());
            } else {
                stmts.push(comp.create_display_name_stmt());
            }
        })
    }

    fn visit_mut_var_declarator(&mut self, n: &mut VarDeclarator) {
        if let Some(comp) = process_var_declarator(n) {
            self.components.push(comp.with_pos(self.components.len()))
        }
    }

    fn visit_mut_fn_expr(&mut self, n: &mut FnExpr) {
        if let Some(comp) = process_fn_expr(n) {
            self.components.push(comp.with_pos(self.components.len()))
        }
    }

    fn visit_mut_fn_decl(&mut self, n: &mut FnDecl) {
        if let Some(comp) = process_fn_decl(n) {
            self.components.push(comp.with_pos(self.components.len()))
        }
    }
}

fn process_var_declarator(var_decl: &mut VarDeclarator) -> Option<Component> {
    if let Some(init) = &var_decl.init {
        if init.is_jsx_element() || init.is_jsx_fragment() || init.is_paren() { return None; }
    }

    let has_jsx = HasJSXVisitor::test(var_decl);
    if !has_jsx { return None };

    let name = &var_decl.name.as_ident()?.id;
    Some(Component {
        pos: 0,
        name: name.sym.clone(),
        ctx: name.span.ctxt
    })
}

fn process_fn_expr(fn_expr: &mut FnExpr) -> Option<Component> {
    let has_jsx = HasJSXVisitor::test(fn_expr);
    if !has_jsx { return None };

    if let Some(name) = &fn_expr.ident {
        return Some(Component {
            pos: 0,
            name: name.sym.clone(),
            ctx: name.span.ctxt
        })
    }
    return None
}

fn process_fn_decl(fn_decl: &mut FnDecl) -> Option<Component> {
    let has_jsx = HasJSXVisitor::test(fn_decl);
    if !has_jsx { return None };

    let name = &fn_decl.ident;
    Some(Component {
        pos: 0,
        name: name.sym.clone(),
        ctx: name.span.ctxt
    })
}
