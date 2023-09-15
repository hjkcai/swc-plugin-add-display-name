mod add_display_name;
mod has_jsx;

use add_display_name::AddDisplayNameVisitor;
use swc_core::ecma::ast::Program;
use swc_core::ecma::visit::{as_folder, FoldWith};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(AddDisplayNameVisitor::default()))
}

#[cfg(test)]
mod test {
    use swc_core::common::{chain, Mark};
    use swc_core::ecma::transforms::base::resolver;
    use swc_core::ecma::transforms::testing::Tester;
    use swc_core::ecma::{
        parser::{Syntax, TsConfig},
        transforms::testing::test,
        visit::{as_folder, Fold},
    };

    const SYNTAX: Syntax = Syntax::Typescript(TsConfig {
        tsx: true,
        decorators: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: true,
    });

    fn runner(_: &mut Tester) -> impl Fold {
        chain!(
            resolver(Mark::new(), Mark::new(), false),
            as_folder(super::AddDisplayNameVisitor::default())
        )
    }

    test!(SYNTAX, runner,
        /* Name */ basic_export,
        /* Input */ r#"
            export const Component = () => <div />;
        "#,
        /* Output */ r#"
            export const Component = () => <div />;
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ basic_non_export,
        /* Input */ r#"
            const Component = () => <div />;
        "#,
        /* Output */ r#"
            const Component = () => <div />;
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ fn_expression_export,
        /* Input */ r#"
            export const Component = function() { return <div />; }
        "#,
        /* Output */ r#"
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ fn_declaration_export,
        /* Input */ r#"
            export function Component() { return <div />; }
        "#,
        /* Output */ r#"
            export function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ fn_declaration_default_export,
        /* Input */ r#"
            export default function Component() { return <div />; }
        "#,
        /* Output */ r#"
            export default function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ fn_declaration,
        /* Input */ r#"
            function Component() { return <div />; }
        "#,
        /* Output */ r#"
            function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ memo,
        /* Input */ r#"
            export const Component = memo(() => <div />);
        "#,
        /* Output */ r#"
            export const Component = memo(() => <div />);
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ forward_ref,
        /* Input */ r#"
            export const Component = forwardRef((props, ref) => <div />);
        "#,
        /* Output */ r#"
            export const Component = forwardRef((props, ref) => <div />);
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ returning_fragment,
        /* Input */ r#"
            export const Component = () => <></>;
        "#,
        /* Output */ r#"
            export const Component = () => <></>;
            Component.displayName = "Component";
        "#
    );

    test!(SYNTAX, runner,
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

    test!(SYNTAX, runner,
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

    test!(SYNTAX, runner,
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

    test!(SYNTAX, runner,
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

    test!(SYNTAX, runner,
        /* Name */ should_not_work_on_normal_fn,
        /* Input */ r#"
            export const fn = () => console.log();
        "#,
        /* Output */ r#"
            export const fn = () => console.log();
        "#
    );

    test!(SYNTAX, runner,
        /* Name */ should_not_work_on_default_export,
        /* Input */ r#"
            export default (() => <div />);
        "#,
        /* Output */ r#"
            export default (() => <div />);
        "#
    );

    test!(SYNTAX, runner,
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

    test!(SYNTAX, runner,
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
}
