use super::has_jsx::HasJSXVisitor;
use std::collections::HashSet;
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
                    left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::Ident(Ident::new(self.name.clone(), Span { ctxt: self.ctx, ..DUMMY_SP }))),
                        prop: MemberProp::Ident(Ident::new(JsWord::from("displayName").into(), DUMMY_SP))
                    })),
                    right: Box::new(Expr::Lit(Lit::Str(Str::from(self.name.clone()))))
                }))
            })
        )
    }
}

#[derive(Default)]
pub struct AddDisplayNameVisitor;

impl VisitMut for AddDisplayNameVisitor {
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        let mut components: Vec<Component> = Vec::new();
        let mut components_names_with_display_name: HashSet<JsWord> = HashSet::new();

        stmts.iter_mut().enumerate().for_each(|(pos, stmt)| {
            if let Some(var_decl) = to_var_decl(stmt) {
                var_decl.decls.iter_mut().for_each(|var_declarator| {
                    if let Some(comp) = process_var_declarator(var_declarator) {
                        components.push(comp.with_pos(pos))
                    }
                })
            }

            if let Some(fn_decl) = to_fn_decl(stmt) {
                if let Some(comp) = process_fn_decl(fn_decl) {
                    components.push(comp.with_pos(pos))
                }
            }

            if let Some(fn_expr) = to_fn_expr(stmt) {
                if let Some(comp) = process_fn_expr(fn_expr) {
                    components.push(comp.with_pos(pos))
                }
            }

            if let Some(assign_expr) = to_assignment_expr(stmt) {
                if let Some(component_name) = process_assignment_expr(assign_expr) {
                    components_names_with_display_name.insert(component_name);
                }
            }
        });

        components.iter().enumerate().for_each(|(i, comp)| {
            let index = i + comp.pos + 1;

            
            if components_names_with_display_name.contains(&comp.name) {
                return;
            }

            if index < stmts.len() {
                stmts.insert(index, comp.create_display_name_stmt());
            } else {
                stmts.push(comp.create_display_name_stmt());
            }
        })
    }
}

fn to_var_decl(stmt: &mut ModuleItem) -> Option<&mut VarDecl> {
    match stmt {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { span: _, decl })) => {
            match decl {
                Decl::Var(var_decl) => Some(var_decl),
                _ => None
            }
        },
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => Some(var_decl),
        _ => None
    }
}

fn process_var_declarator(var_decl: &mut VarDeclarator) -> Option<Component> {
    if let Some(init) = &var_decl.init {
        if init.is_jsx_element() || init.is_jsx_fragment() || init.is_paren() || init.is_object() {
            return None;
        }
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

fn to_fn_expr(stmt: &mut ModuleItem) -> Option<&mut FnExpr> {
    match stmt {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(ExportDefaultDecl { span: _, decl })) => {
            match decl {
                DefaultDecl::Fn(fn_expr) => Some(fn_expr),
                _ => None,
            }
        },
        _ => None
    }
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

fn to_fn_decl(stmt: &mut ModuleItem) -> Option<&mut FnDecl> {
    match stmt {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { span: _, decl })) => {
            match decl {
                Decl::Fn(fn_decl) => Some(fn_decl),
                _ => None
            }
        },
        ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => Some(fn_decl),
        _ => None
    }
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

fn to_assignment_expr(stmt: &mut ModuleItem) -> Option<&mut AssignExpr> {
    match stmt {
        ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) => {
            match &mut **expr {
                Expr::Assign(assign_expr) => Some(assign_expr),
                _ => None
            }
        }
        _ => None
    }
}

fn process_assignment_expr(expr: &mut AssignExpr) -> Option<JsWord> {
    if expr.op != AssignOp::Assign {
        return None;
    }

    match &expr.left {
        AssignTarget::Simple(SimpleAssignTarget::Member(
            MemberExpr { prop: MemberProp::Ident(ident), obj, .. }
        )) => {


            if &*ident.sym != "displayName" {
                return None
            }

            let obj = obj.as_ident()?;

            Some(obj.sym.clone())
        },
        _ => None
    }
}