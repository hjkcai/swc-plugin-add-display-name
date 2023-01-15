use swc_core::ecma::{
    ast::*,
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith}, atoms::JsWord,
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::common::{DUMMY_SP};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);
        stmts.insert(1, ModuleItem::Stmt(set_display_name_stmt("A", "asdf")))
    }
}

fn set_display_name_stmt(target: &str, display_name: &str) -> Stmt {
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(JsWord::from(target).into(), DUMMY_SP))),
                prop: MemberProp::Ident(Ident::new(JsWord::from("displayName").into(), DUMMY_SP))
            }))),
            right: Box::new(Expr::Lit(Lit::Str(Str::from(display_name))))
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
    Default::default(),
    |_| as_folder(TransformVisitor),
    boo,
    // Input codes
    r#"export{}; console.log("transform");"#,
    // Output codes after transformed with plugin
    r#"export{}; console.log("transform");"#
);
