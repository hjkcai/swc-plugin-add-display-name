mod add_display_name;
mod has_jsx;

use add_display_name::AddDisplayNameVisitor;
use swc_core::ecma::ast::Program;
use swc_core::ecma::visit::visit_mut_pass;
use swc_core::plugin::plugin_transform;
use swc_core::plugin::proxies::TransformPluginProgramMetadata;

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(&mut visit_mut_pass(AddDisplayNameVisitor::default()))
}

#[cfg(test)]
mod test {
    use swc_core::common::Mark;
    use swc_core::ecma::ast::Pass;
    use swc_core::ecma::transforms::base::resolver;
    use swc_core::ecma::transforms::testing::{Tester,test_inline};
    use swc_core::ecma::visit::visit_mut_pass;
    use swc_core::ecma::{
        parser::{Syntax, TsSyntax},
        transforms::testing::test,
    };

    const SYNTAX: Syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: true,
    });

    fn runner(_: &mut Tester) -> impl Pass {
        (
            resolver(Mark::new(), Mark::new(), false),
            visit_mut_pass(super::AddDisplayNameVisitor::default())
        )
    }

    test_inline!(SYNTAX, runner,
        /* Name */ basic_export,
        /* Input */ r#"
            export const Component = () => <div />;
        "#,
        /* Output */ r#"
            export const Component = () => <div />;
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ basic_non_export,
        /* Input */ r#"
            const Component = () => <div />;
        "#,
        /* Output */ r#"
            const Component = () => <div />;
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ fn_expression_export,
        /* Input */ r#"
            export const Component = function() { return <div />; }
        "#,
        /* Output */ r#"
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
        "#
    );


    test_inline!(SYNTAX, runner,
        /* Name */ fn_expression_export_multiline,
        /* Input */ r#"
            const a = {};
            export const Component = function() {
                return <div />;
            }
            export default Component;
        "#,
        /* Output */ r#"
            const a = {};
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
            export default Component;
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ fn_declaration_export,
        /* Input */ r#"
            export function Component() { return <div />; }
        "#,
        /* Output */ r#"
            export function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ fn_declaration_default_export,
        /* Input */ r#"
            export default function Component() { return <div />; }
        "#,
        /* Output */ r#"
            export default function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ fn_declaration,
        /* Input */ r#"
            function Component() { return <div />; }
        "#,
        /* Output */ r#"
            function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ memo,
        /* Input */ r#"
            export const Component = memo(() => <div />);
        "#,
        /* Output */ r#"
            export const Component = memo(() => <div />);
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ forward_ref,
        /* Input */ r#"
            export const Component = forwardRef((props, ref) => <div />);
        "#,
        /* Output */ r#"
            export const Component = forwardRef((props, ref) => <div />);
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ returning_fragment,
        /* Input */ r#"
            export const Component = () => <></>;
        "#,
        /* Output */ r#"
            export const Component = () => <></>;
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ two_components,
        /* Input */ r#"
            export const Foo = () => <div />;
            export const Bar = memo(() => <div />);
        "#,
        /* Output */ r#"
            export const Foo = () => <div />;
            Foo.displayName = "Foo";
            export const Bar = memo(() => <div />);
            Bar.displayName = "Bar";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ mix_var_export,
        /* Input */ r#"
            const Foo = () => <div />;
            export const Bar = memo(() => <div />);
        "#,
        /* Output */ r#"
            const Foo = () => <div />;
            Foo.displayName = "Foo";
            export const Bar = memo(() => <div />);
            Bar.displayName = "Bar";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ three_components,
        /* Input */ r#"
            export const Foo = () => <div />;
            export const Bar = memo(() => <div />);
            export const Baz = observer(() => <div />);
        "#,
        /* Output */ r#"
            export const Foo = () => <div />;
            Foo.displayName = "Foo";
            export const Bar = memo(() => <div />);
            Bar.displayName = "Bar";
            export const Baz = observer(() => <div />);
            Baz.displayName = "Baz";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ var_let_const,
        /* Input */ r#"
            var Foo = () => <div />;
            let Bar = memo(() => <div />);
            const Baz = observer(() => <div />);
        "#,
        /* Output */ r#"
            var Foo = () => <div />;
            Foo.displayName = "Foo";
            let Bar = memo(() => <div />);
            Bar.displayName = "Bar";
            const Baz = observer(() => <div />);
            Baz.displayName = "Baz";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ one_const_statement_multiple_exprs,
        /* Input */ r#"
            const Foo = () => <div />, Bar = memo(() => <div />);
        "#,
        /* Output */ r#"
            const Foo = () => <div />, Bar = memo(() => <div />);
            Foo.displayName = "Foo";
            Bar.displayName = "Bar";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_work_on_normal_fn,
        /* Input */ r#"
            export const fn = () => console.log();
        "#,
        /* Output */ r#"
            export const fn = () => console.log();
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_work_on_default_export,
        /* Input */ r#"
            export default (() => <div />);
        "#,
        /* Output */ r#"
            export default (() => <div />);
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_work_on_non_top_level_fns,
        /* Input */ r#"
            const fn = () => {
                const render = () => <div />;
                return render;
            };
        "#,
        /* Output */ r#"
            const fn = () => {
                const render = () => <div />;
                return render;
            };
            fn.displayName = "fn";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_work_on_direct_jsx_element,
        /* Input */ r#"
            const foo = <div />;
            const bar = <></>;
        "#,
        /* Output */ r#"
            const foo = <div />;
            const bar = <></>;
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_work_on_object_literal,
        /* Input */ r#"
            const foo = { bar: <div /> };
        "#,
        /* Output */ r#"
            const foo = { bar: <div /> };
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_work_on_non_top_level, // https://github.com/hjkcai/swc-plugin-add-display-name/issues/7
        /* Input */ r#"
            test("should not work", () => {
                const ref = render(<App />);
            });
        "#,
        /* Output */ r#"
            test("should not work", () => {
                const ref = render(<App />);
            });
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_duplicate_existed,
        /* Input */ r#"
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
        "#,
        /* Output */ r#"
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(SYNTAX, runner,
        /* Name */ should_not_rewrite_existed,
        /* Input */ r#"
            export const Component = function() { return <div />; }
            Component.displayName = "CustomName";
        "#,
        /* Output */ r#"
            export const Component = function() { return <div />; }
            Component.displayName = "CustomName";
        "#
    );
}
