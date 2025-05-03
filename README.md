# rs-markdown-parser

A Node.js module for converting Markdown files to HTML using Rust, powered by the `pulldown-cmark` library and Neon bindings.

## Features

- Fast Markdown to HTML conversion using Rust's `pulldown-cmark`.
- Optional GitHub Flavored Markdown (GFM) support for tables, footnotes, strikethrough, task lists, and heading attributes.
- Exports HTML as an ES module for easy integration.

## Installation

1. **Prerequisites**:

   - [Node.js](https://nodejs.org/) (v14 or later recommended).
   - [Rust](https://www.rust-lang.org/tools/install) and Cargo.
   - A C++ compiler (e.g., `build-essential` on Ubuntu, or Visual Studio Build Tools on Windows).

2. **Clone the repository**:

   ```bash
   git clone https://github.com/aliezzahn/rs-markdown-parser.git
   cd rs-markdown-parser
   ```

3. **Install dependencies**:

   ```bash
   npm install
   ```

4. **Build the module**:

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

- `filePath` (string): Path to the Markdown file.
- `gfm` (boolean): Enable GitHub Flavored Markdown features (tables, footnotes, strikethrough, task lists, heading attributes).

### Example

```javascript
const { processMarkdown } = require("rs-markdown-parser");
const { join } = require("path");

// Convert a Markdown file to HTML
const filePath = join(__dirname, "./test.md");
const result = processMarkdown(filePath, false);

console.log(result); // Outputs: export default `<html_content>`;
```

### Example Markdown File (`test.md`)

```markdown
# Hello, World!

This is a **Markdown** file.

- Item 1
- Item 2
```

### Output

```javascript
export default `<h1>Hello, World!</h1>\n<p>This is a <strong>Markdown</strong> file.</p>\n<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>`;
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

## Development

- **Run tests**:

  ```bash
  npm run test
  ```

- **Rebuild after changes**:
  Modify the Rust code in `src/lib.rs` and run `npm run debug` or `npm run build`.

## License

MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/aliezzahn/rs-markdown-parser).

## Issues

Report bugs or request features on the [GitHub Issues page](https://github.com/aliezzahn/rs-markdown-parser/issues).
