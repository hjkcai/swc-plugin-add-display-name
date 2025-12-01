# swc-plugin-add-display-name

Automatically add `displayName` to *top-level* React *functional* components.
- ✅ `const Component = () => <jsx />`
- ✅ `function Component() { return <jsx /> }`
- ✅ `const Component = () => jsx("div", { children: "hello" })` (Compiled JSX code)
- ✅ `const Component = () => React.createElement("div", null, "hello")` (Compiled or hand-written JSX code)
- ❌ `const Context = createContext()`

> If you have other situations that needs to add `displayName`, feel free to open an issue or PR!

## Installation

Install with your favorite package manager as devDependency.

```bash
npm i -D swc-plugin-add-display-name
```

Add plugin to wherever you have an SWC config (e.g. `.swcrc` file, `swc-loader` config, etc). Make sure either `jsx` or `tsx` option is turned on depending on the syntax you are using.

```js
// JavaScript
{
  jsc: {
    parser: {
      jsx: true,
    },
    experimental: {
      plugins: [
        ['swc-plugin-add-display-name', {}],
      ],
    },
  },
}

// TypeScript
{
  jsc: {
    parser: {
      syntax: 'typescript',
      tsx: true,
    },
    experimental: {
      plugins: [
        ['swc-plugin-add-display-name', {}],
      ],
    }
  },
}
```

## Configuration

This plugin currently has no configuration. However you have to leave an empty object to meet SWC's API schema.

If you'd like to disable this plugin in production build, remove this plugin from the plugins list.

## Examples

```tsx
// Before
export const Component = () => <div />;

// After
export const Component = () => <div />;
Component.displayName = "Component";
```

```tsx
// Before
const Component = forwardRef((props, ref) => <div />);

// After
const Component = forwardRef((props, ref) => <div />);
Component.displayName = "Component";
```

```tsx
// Before
export function Component() { return <div />; }

// After
export function Component() { return <div />; }
Component.displayName = "Component";
```

```tsx
// Before
export const Component = () => jsx("div", { children: "hello" });

// After
export const Component = () => jsx("div", { children: "hello" });
Component.displayName = "Component";
```

## License

MIT
