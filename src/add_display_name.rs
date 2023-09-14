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

pub struct AddDisplayNameVisitor;

impl VisitMut for AddDisplayNameVisitor {
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        let mut components: Vec<Component> = Vec::new();
        stmts.iter_mut().enumerate().for_each(|(i, stmt)| {
            if let Some(comp) = export_var_decl(stmt) { components.push(comp.with_pos(i)) }
            if let Some(comp) = var_decl_stmt(stmt) { components.push(comp.with_pos(i)) }
            if let Some(comp) = default_export_fn_decl(stmt) { components.push(comp.with_pos(i)) }
            if let Some(comp) = export_fn_decl(stmt) { components.push(comp.with_pos(i)) }
            if let Some(comp) = bare_fn_decl(stmt) { components.push(comp.with_pos(i)) }
        });

        components.iter().enumerate().for_each(|(i, comp)| {
            let index = i + comp.pos + 1;
            stmts.insert(index, ModuleItem::Stmt(set_display_name_stmt(comp)));
        })
    }
}

fn export_var_decl(stmt: &mut ModuleItem) -> Option<Component> {
    let var_decls = stmt.as_mut_module_decl()?.as_mut_export_decl()?.decl.as_mut_var()?.as_mut();
    process_var_decls(var_decls)
}

fn var_decl_stmt(stmt: &mut ModuleItem) -> Option<Component> {
    let var_decls = stmt.as_mut_stmt()?.as_mut_decl()?.as_mut_var()?.as_mut();
    process_var_decls(var_decls)
}

fn process_var_decls(var_decls: &mut VarDecl) -> Option<Component> {
    if var_decls.decls.len() != 1 { return None };
    let var_decl = &mut var_decls.decls[0];

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

fn default_export_fn_decl(stmt: &mut ModuleItem) -> Option<Component> {
    let fn_expr = stmt.as_mut_module_decl()?.as_mut_export_default_decl()?.decl.as_mut_fn_expr()?;
    process_fn_expr(fn_expr)
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

fn export_fn_decl(stmt: &mut ModuleItem) -> Option<Component> {
    let fn_decl = stmt.as_mut_module_decl()?.as_mut_export_decl()?.decl.as_mut_fn_decl()?;
    process_fn_decl(fn_decl)
}

fn bare_fn_decl(stmt: &mut ModuleItem) -> Option<Component> {
    let fn_decl = stmt.as_mut_stmt()?.as_mut_decl()?.as_mut_fn_decl()?;
    process_fn_decl(fn_decl)
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

fn set_display_name_stmt(comp: &Component) -> Stmt {
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(comp.name.clone(), Span { ctxt: comp.ctx, ..DUMMY_SP }))),
                prop: MemberProp::Ident(Ident::new(JsWord::from("displayName").into(), DUMMY_SP))
            }))),
            right: Box::new(Expr::Lit(Lit::Str(Str::from(comp.name.clone()))))
        }))
    })
}
