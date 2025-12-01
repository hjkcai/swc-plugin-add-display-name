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
    use swc_core::ecma::transforms::testing::{test_inline, Tester};
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
            visit_mut_pass(super::AddDisplayNameVisitor::default()),
        )
    }

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ basic_export,
        /* Input */ r#"
            export const Component = () => <div />;
        "#,
        /* Output */
        r#"
            export const Component = () => <div />;
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ basic_non_export,
        /* Input */ r#"
            const Component = () => <div />;
        "#,
        /* Output */
        r#"
            const Component = () => <div />;
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ fn_expression_export,
        /* Input */
        r#"
            export const Component = function() { return <div />; }
        "#,
        /* Output */
        r#"
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ fn_expression_export_multiline,
        /* Input */
        r#"
            const a = {};
            export const Component = function() {
                return <div />;
            }
            export default Component;
        "#,
        /* Output */
        r#"
            const a = {};
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
            export default Component;
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ fn_declaration_export,
        /* Input */ r#"
            export function Component() { return <div />; }
        "#,
        /* Output */
        r#"
            export function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ fn_declaration_default_export,
        /* Input */
        r#"
            export default function Component() { return <div />; }
        "#,
        /* Output */
        r#"
            export default function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ fn_declaration,
        /* Input */ r#"
            function Component() { return <div />; }
        "#,
        /* Output */
        r#"
            function Component() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ memo,
        /* Input */ r#"
            export const Component = memo(() => <div />);
        "#,
        /* Output */
        r#"
            export const Component = memo(() => <div />);
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ forward_ref,
        /* Input */
        r#"
            export const Component = forwardRef((props, ref) => <div />);
        "#,
        /* Output */
        r#"
            export const Component = forwardRef((props, ref) => <div />);
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ returning_fragment,
        /* Input */ r#"
            export const Component = () => <></>;
        "#,
        /* Output */
        r#"
            export const Component = () => <></>;
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ two_components,
        /* Input */
        r#"
            export const Foo = () => <div />;
            export const Bar = memo(() => <div />);
        "#,
        /* Output */
        r#"
            export const Foo = () => <div />;
            Foo.displayName = "Foo";
            export const Bar = memo(() => <div />);
            Bar.displayName = "Bar";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ mix_var_export,
        /* Input */
        r#"
            const Foo = () => <div />;
            export const Bar = memo(() => <div />);
        "#,
        /* Output */
        r#"
            const Foo = () => <div />;
            Foo.displayName = "Foo";
            export const Bar = memo(() => <div />);
            Bar.displayName = "Bar";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ three_components,
        /* Input */
        r#"
            export const Foo = () => <div />;
            export const Bar = memo(() => <div />);
            export const Baz = observer(() => <div />);
        "#,
        /* Output */
        r#"
            export const Foo = () => <div />;
            Foo.displayName = "Foo";
            export const Bar = memo(() => <div />);
            Bar.displayName = "Bar";
            export const Baz = observer(() => <div />);
            Baz.displayName = "Baz";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ var_let_const,
        /* Input */
        r#"
            var Foo = () => <div />;
            let Bar = memo(() => <div />);
            const Baz = observer(() => <div />);
        "#,
        /* Output */
        r#"
            var Foo = () => <div />;
            Foo.displayName = "Foo";
            let Bar = memo(() => <div />);
            Bar.displayName = "Bar";
            const Baz = observer(() => <div />);
            Baz.displayName = "Baz";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ one_const_statement_multiple_exprs,
        /* Input */
        r#"
            const Foo = () => <div />, Bar = memo(() => <div />);
        "#,
        /* Output */
        r#"
            const Foo = () => <div />, Bar = memo(() => <div />);
            Foo.displayName = "Foo";
            Bar.displayName = "Bar";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_work_on_normal_fn,
        /* Input */ r#"
            export const fn = () => console.log();
        "#,
        /* Output */ r#"
            export const fn = () => console.log();
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_work_on_default_export,
        /* Input */ r#"
            export default (() => <div />);
        "#,
        /* Output */ r#"
            export default (() => <div />);
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_work_on_non_top_level_fns,
        /* Input */
        r#"
            const fn = () => {
                const render = () => <div />;
                return render;
            };
        "#,
        /* Output */
        r#"
            const fn = () => {
                const render = () => <div />;
                return render;
            };
            fn.displayName = "fn";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_work_on_direct_jsx_element,
        /* Input */
        r#"
            const foo = <div />;
            const bar = <></>;
        "#,
        /* Output */
        r#"
            const foo = <div />;
            const bar = <></>;
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_work_on_object_literal,
        /* Input */ r#"
            const foo = { bar: <div /> };
        "#,
        /* Output */ r#"
            const foo = { bar: <div /> };
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */
        should_not_work_on_non_top_level, // https://github.com/hjkcai/swc-plugin-add-display-name/issues/7
        /* Input */
        r#"
            test("should not work", () => {
                const ref = render(<App />);
            });
        "#,
        /* Output */
        r#"
            test("should not work", () => {
                const ref = render(<App />);
            });
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_duplicate_existed,
        /* Input */
        r#"
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
        "#,
        /* Output */
        r#"
            export const Component = function() { return <div />; }
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_rewrite_existed,
        /* Input */
        r#"
            export const Component = function() { return <div />; }
            Component.displayName = "CustomName";
        "#,
        /* Output */
        r#"
            export const Component = function() { return <div />; }
            Component.displayName = "CustomName";
        "#
    );

    // Tests for JSX Runtime functions and createElement

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ jsx_runtime_call,
        /* Input */ r#"
            export const Component = () => jsx("div", { children: "hello" });
        "#,
        /* Output */ r#"
            export const Component = () => jsx("div", { children: "hello" });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ jsxs_runtime_call,
        /* Input */ r#"
            export const Component = () => jsxs("div", { children: ["child1", "child2"] });
        "#,
        /* Output */ r#"
            export const Component = () => jsxs("div", { children: ["child1", "child2"] });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ jsxdev_runtime_call,
        /* Input */ r#"
            export const Component = () => jsxDEV("div", { children: "hello" }, void 0, false);
        "#,
        /* Output */ r#"
            export const Component = () => jsxDEV("div", { children: "hello" }, void 0, false);
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ create_element_call,
        /* Input */ r#"
            import React from "react";
            export const Component = () => React.createElement("div", null, "hello");
        "#,
        /* Output */ r#"
            import React from "react";
            export const Component = () => React.createElement("div", null, "hello");
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ create_element_with_props,
        /* Input */ r#"
            import React from "react";
            export const Component = () => React.createElement("div", { className: "foo" }, "content");
        "#,
        /* Output */ r#"
            import React from "react";
            export const Component = () => React.createElement("div", { className: "foo" }, "content");
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ create_element_multiple_children,
        /* Input */ r#"
            import React from "react";
            export const Component = () => React.createElement("div", { className: "foo" }, child1, child2);
        "#,
        /* Output */ r#"
            import React from "react";
            export const Component = () => React.createElement("div", { className: "foo" }, child1, child2);
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ namespace_jsx_call,
        /* Input */ r#"
            import * as React from "react";
            export const Component = () => React.jsx("div", { children: "hello" });
        "#,
        /* Output */ r#"
            import * as React from "react";
            export const Component = () => React.jsx("div", { children: "hello" });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ namespace_jsxs_call,
        /* Input */ r#"
            import * as React from "react";
            export const Component = () => React.jsxs("div", { children: ["child1", "child2"] });
        "#,
        /* Output */ r#"
            import * as React from "react";
            export const Component = () => React.jsxs("div", { children: ["child1", "child2"] });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ namespace_create_element_call,
        /* Input */ r#"
            import * as React from "react";
            export const Component = () => React.createElement("div", null, "hello");
        "#,
        /* Output */ r#"
            import * as React from "react";
            export const Component = () => React.createElement("div", null, "hello");
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ mixed_jsx_and_react_calls,
        /* Input */ r#"
            import React from "react";
            export const ComponentA = () => <div>traditional jsx</div>;
            export const ComponentB = () => React.createElement("div", null, "createElement");
            export const ComponentC = () => jsx("div", { children: "runtime jsx" });
        "#,
        /* Output */ r#"
            import React from "react";
            export const ComponentA = () => <div>traditional jsx</div>;
            ComponentA.displayName = "ComponentA";
            export const ComponentB = () => React.createElement("div", null, "createElement");
            ComponentB.displayName = "ComponentB";
            export const ComponentC = () => jsx("div", { children: "runtime jsx" });
            ComponentC.displayName = "ComponentC";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ conditional_jsx_calls,
        /* Input */ r#"
            export const Component = ({ condition }) =>
                condition ? jsx("div", { children: "yes" }) : jsx("span", { children: "no" });
        "#,
        /* Output */ r#"
            export const Component = ({ condition }) =>
                condition ? jsx("div", { children: "yes" }) : jsx("span", { children: "no" });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ nested_function_patterns,
        /* Input */ r#"
            export const Component = () => {
                const helper = () => jsx("div", { children: "helper" });
                return jsx("div", { children: "main" });
            };
        "#,
        /* Output */ r#"
            export const Component = () => {
                const helper = () => jsx("div", { children: "helper" });
                return jsx("div", { children: "main" });
            };
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ jsx_calls_in_object_literals,
        /* Input */ r#"
            const config = {
                renderer: () => jsx("div", { children: "rendered" })
            };
        "#,
        /* Output */ r#"
            const config = {
                renderer: () => jsx("div", { children: "rendered" })
            };
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ aliased_jsx_imports,
        /* Input */ r#"
            import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
            export const Component = () => _jsx("div", { children: "test" });
        "#,
        /* Output */ r#"
            import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
            export const Component = ()=>_jsx("div", {
                    children: "test"
                });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ mixed_jsx_and_jsx_runtime,
        /* Input */ r#"
            export const Component = () => jsx("div", {
                children: [
                    <span>Traditional JSX</span>,
                    jsx("span", { children: "Runtime JSX" })
                ]
            });
        "#,
        /* Output */ r#"
            export const Component = () => jsx("div", {
                children: [
                    <span>Traditional JSX</span>,
                    jsx("span", { children: "Runtime JSX" })
                ]
            });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ jsx_calls_in_array_map,
        /* Input */ r#"
            export const Component = ({ items }) => jsx("div", {
                children: items.map(item => jsx("span", { key: item.id, children: item.name }))
            });
        "#,
        /* Output */ r#"
            export const Component = ({ items }) => jsx("div", {
                children: items.map(item => jsx("span", { key: item.id, children: item.name }))
            });
            Component.displayName = "Component";
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ should_not_work_on_other_function_calls,
        /* Input */ r#"
            export const Component = () => someOtherFunction("div", { children: "hello" });
        "#,
        /* Output */ r#"
            export const Component = () => someOtherFunction("div", { children: "hello" });
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ direct_createelement_call,
        /* Input */ r#"
            export const Component = () => createElement("div", null, "hello");
        "#,
        /* Output */ r#"
            export const Component = () => createElement("div", null, "hello");
            Component.displayName = "Component";
        "#
    );
}
