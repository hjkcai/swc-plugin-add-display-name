use swc_core::common::{DUMMY_SP};
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    parser::{Syntax, TsConfig},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

struct HasJSXVisitor {
    has_jsx: bool,
}

impl HasJSXVisitor {
    fn test(node: &mut impl VisitMutWith<Self>) -> bool {
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
}

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        let mut a: Vec<(usize, JsWord)> = Vec::new();
        stmts.iter_mut().enumerate().for_each(|(i, stmt)| {
            let mut export_var_decl = || {
                let var_decls = stmt.as_mut_module_decl()?.as_mut_export_decl()?.decl.as_mut_var()?.as_mut();
                if var_decls.decls.len() != 1 { return None };

                let var_decl = &mut var_decls.decls[0];
                let has_jsx = HasJSXVisitor::test(var_decl);
                if !has_jsx { return None };

                let name = &var_decl.name.as_ident()?.id;
                Some((i, name.sym.clone()))
            };

            if let Some(result) = export_var_decl() { a.push(result) }
        });

        a.iter().for_each(|(i, name)| {
            stmts.insert(*i + 1, ModuleItem::Stmt(set_display_name_stmt(name)));
        })
    }
}

fn set_display_name_stmt(target: &JsWord) -> Stmt {
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(target.clone(), DUMMY_SP))),
                prop: MemberProp::Ident(Ident::new(JsWord::from("displayName").into(), DUMMY_SP))
            }))),
            right: Box::new(Expr::Lit(Lit::Str(Str::from(target.clone()))))
        }))
    })
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test!(
    Syntax::Typescript(TsConfig { tsx: true, decorators: false, dts: false, no_early_errors: false }),
    |_| as_folder(TransformVisitor),
    boo,
    // Input codes
    r#"
        export const NAME: FC = memo(() => <div />));
    "#,
    // Output codes after transformed with plugin
    r#"
        export const NAME: FC = memo(() => <div />);
        NAME.displayName = "NAME";
    "#
);
