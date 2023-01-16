# swc-plugin-add-display-name

Automatically add `displayName` to *top-level* React *functional* components.

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

The plugin currently has no configuration. However you have to leave an empty object to meet SWC's API schema.

## License

MIT
