# rs-markdown-parser

A Node.js module for converting Markdown files to HTML using Rust, powered by the `pulldown-cmark` library and Neon bindings.

## Features

- Fast Markdown to HTML conversion using Rust's `pulldown-cmark`.
- Optional GitHub Flavored Markdown (GFM) support for tables, footnotes, strikethrough, task lists, and heading attributes.
- Exports HTML as an ES module for easy integration.
- TypeScript support with type definitions.

## Installation

1. **Install the package**:

   ```bash
   npm install rs-markdown-parser
   ```

2. **Build the module** (if modifying source):

   - For development (debug build):
     ```bash
     npm run debug
     ```
   - For production (release build):
     ```bash
     npm run build
     ```
   - For cross-compilation (if targeting different platforms):
     ```bash
     npm run cross
     ```

   The build process compiles the Rust code into a native Node.js module (`index.node`).

## Usage

The module exports a single function, `processMarkdown`, which converts a Markdown file to HTML and returns it as an ES module string.

### Parameters

- `filePath` (string): The file path to the Markdown file (e.g., `"./test.md"`). Must be a valid path to an existing file.
- `gfm` (boolean): Enables GitHub Flavored Markdown features. Set to `true` to support tables, footnotes, strikethrough, task lists, and heading attributes; `false` for standard Markdown.

### JavaScript Example

```javascript
const { processMarkdown } = require("rs-markdown-parser");
const { join } = require("path");

const filePath = join(__dirname, "./test.md");
const result = processMarkdown(filePath, false);

console.log(result); // Outputs: export default `<html_content>`;
```

### TypeScript Example

```typescript
import { processMarkdown } from "rs-markdown-parser";
import { join } from "path";

const filePath: string = join(__dirname, "./test.md");
const result: string = processMarkdown(filePath, false);

console.log(result); // Outputs: export default `<html_content>`;
```

### Example Markdown File (`test.md`)

```markdown
# Hello, World!

This is a **Markdown** file.

- Item 1
- Item 2

# Product Comparison

| Product      | Price   | Features                   | In Stock |
|--------------|---------|----------------------------|----------|
| Phone A      | $299    | 64GB, 12MP Camera, 4G LTE  | Yes      |
| Phone B      | $399    | 128GB, 48MP Camera, 5G     | No       |
| Phone C      | $499    | 256GB, 108MP Camera, 5G    | Yes      |
```

### Output

```javascript
export default `<h1>Hello, World!</h1>\n<p>This is a <strong>Markdown</strong> file.</p>\n<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>\n<h1>Product Comparison</h1>\n
<table>\n<thead>\n<tr><th>Product</th><th>Price</th><th>Features</th><th>In Stock</th></tr>\n</thead>\n<tbody>\n<tr><td>Phone A</td><td>$299</td><td>64GB, 12MP Camera, 4G LTE</td><td>Yes</td></tr>\n<tr><td>Phone B</td><td>$399</td><td>128GB, 48MP Camera, 5G</td><td>No</td></tr>\n<tr><td>Phone C</td><td>$499</td><td>256GB, 108MP Camera, 5G</td><td>Yes</td></tr>\n</tbody>\n</table>`;
```

### Using with GFM

To enable GitHub Flavored Markdown features:

```javascript
const result = processMarkdown(filePath, true);
```

This enables support for:

- Tables
- Footnotes
- Strikethrough
- Task lists
- Heading attributes

## TypeScript Support

This module includes TypeScript type definitions in `index.d.ts`. To use with TypeScript:

1. Ensure TypeScript is installed:

   ```bash
   npm install -g typescript
   ```

2. Include the module in your TypeScript project:

   ```bash
   npm install rs-markdown-parser
   ```

3. The type definitions are automatically picked up from `index.d.ts` when you import the module.

## Development

- **Run tests**:

  ```bash
  npm run test
  ```

- **Rebuild after changes**:
  Modify the Rust code in `src/lib.rs` or TypeScript definitions in `index.d.ts` and run `npm run debug` or `npm run build`.

## License

MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/aliezzahn/rs-markdown-parser).

## Issues

Report bugs or request features on the [GitHub Issues page](https://github.com/aliezzahn/rs-markdown-parser/issues).
