# swc-plugin-add-display-name

> **PLEASE GIVE A STAR IF YOU LIKE THIS PROJECT!**

![NPM Version](https://img.shields.io/npm/v/swc-plugin-add-display-name?style=for-the-badge)
![NPM Downloads](https://img.shields.io/npm/dm/swc-plugin-add-display-name?style=for-the-badge)

Automatically add `displayName` to *top-level* React *functional* components:
- `const Component = () => <jsx />`
- `function Component() { return <jsx /> }`
- `const Component = () => jsx("div", { children: "hello" })` (Compiled JSX code)
- `const Component = () => React.createElement("div", null, "hello")` (Compiled or hand-written JSX code)

And some API calls that produce component:
- `const Context = createContext()` (React Context)
- `const StyledButton = styled.button` (Styled Components)
- `const ObservedComponent = observer(() => <jsx />)` (MobX observer)
- `const ConnectedComponent = connect(...)(() => <jsx />)` (Redux connect)

If you have other situations that needs to add `displayName`, feel free to open an issue or PR!

## Installation

Install with your favorite package manager as devDependency.

```bash
npm i -D swc-plugin-add-display-name
```

## Configuration

Add this plugin to wherever you have an SWC config.

This plugin currently has no configuration. However you have to pass an empty object to meet SWC's API schema.

If you'd like to disable this plugin in production build, remove this plugin from the plugins list.

### [`.swcrc`](https://swc.rs/docs/configuration/compilation#jscexperimentalplugins)

You may configure SWC directly via `.swcrc`.
Or pass the configuration as options to loaders (e.g. `swc-loader`, Rspack's `builtin:swc-loader`).

Make sure either `jsx` or `tsx` option is turned on depending on the syntax you are using.

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["swc-plugin-add-display-name", {}]
      ]
    }
  }
}
```

### [Rsbuild](https://rsbuild.rs/guide/configuration/swc#register-swc-plugin)

Simply configure SWC via `tools.swc`.

```js
export default {
  tools: {
    swc: {
      jsc: {
        experimental: {
          plugins: [
            ['swc-plugin-add-display-name', {}],
          ],
        },
      },
    },
  },
};
```

### [Next.js](https://nextjs.org/docs/architecture/nextjs-compiler)

If you’re using the Next.js Compiler powered by SWC, add this plugin to your next.config.js.

```js
module.exports = {
  experimental: {
    swcPlugins: [['swc-plugin-add-display-name', {}]],
  },
};
```

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

```tsx
// Before
import { createContext } from 'react';
export const ThemeContext = createContext('light');

// After
import { createContext } from 'react';
export const ThemeContext = createContext('light');
ThemeContext.displayName = "ThemeContext";
```

```tsx
// Before
import styled from 'styled-components';
export const StyledButton = styled.button`
  color: red;
`;

// After
import styled from 'styled-components';
export const StyledButton = styled.button`
  color: red;
`;
StyledButton.displayName = "StyledButton";
```

```tsx
// Before
import { observer } from 'mobx-react-lite';
export const ObservedComponent = observer(() => <div />);

// After
import { observer } from 'mobx-react-lite';
export const ObservedComponent = observer(() => <div />);
ObservedComponent.displayName = "ObservedComponent";
```

```tsx
// Before
import { connect } from 'react-redux';
export const ConnectedComponent = connect(() => ({}))(() => null);

// After
import { connect } from 'react-redux';
export const ConnectedComponent = connect(() => ({}))(() => null);
ConnectedComponent.displayName = "ConnectedComponent";
```

## Troubleshooting

### SWC Versions

SWC plugins may encounter compatibility issues across different SWC versions.
If your app won't compile after configuring this plugin,
please try to find a compatible version between this plugin and SWC.

You may refer to the following docs:
- [Selecting the version](https://swc.rs/docs/plugin/selecting-swc-core)
- [SWC compatibility table](https://plugins.swc.rs/versions/range)
- [SWC plugin version mismatch](https://rspack.rs/errors/swc-plugin-version)

| swc-plugin-add-display-name | swc_core |
|-----------------------------|----------|
| >0.9.0                      | 54.0.0   |
| >=0.7.0 <0.9.0              | 46.0.3   |
| ^0.6.0                      | 31.1.0   |

### Incorrect Results

Please file an issue if this plugin:
- Causes runtime errors (e.g. `Cannot add property displayName, object is not extensible`, `Cannot create property 'displayName'`).
- Adds `displayName` where it shouldn't.
- Fails to add `displayName` on certain cases.

## License

MIT
