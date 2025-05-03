
# 🚀 rs-markdown-parser

A blazing-fast Node.js module for converting Markdown to HTML — powered by Rust’s [`pulldown-cmark`](https://docs.rs/pulldown-cmark) and [Neon](https://neon-bindings.com/) for seamless Node integration.

---

## ✨ Features

- ⚡ **Ultra-fast conversion** using Rust’s performance and `pulldown-cmark`.
- 🧩 **Optional GitHub Flavored Markdown (GFM)** support — tables, footnotes, strikethrough, task lists, and heading attributes.
- 📦 **ES module export** — easily import HTML in your frontend projects.
- 🛡️ **TypeScript ready** — includes full type definitions.

---

## 📦 Installation

```bash
npm install rs-markdown-parser
````

> 💡 *Note: To modify or rebuild the native module, follow the build instructions below.*

---

## 🛠️ Building the Module

If you're editing the Rust source or cross-compiling, use one of the following scripts:

| Mode        | Command         | Notes                            |
| ----------- | --------------- | -------------------------------- |
| Debug       | `npm run debug` | Fast builds for development      |
| Release     | `npm run build` | Optimized builds for production  |
| Cross-build | `npm run cross` | Build for other target platforms |

The build will generate a native Node.js addon (`index.node`).

---

## 🚀 Usage

### Importing the Module

#### JavaScript

```js
const { processMarkdown } = require("rs-markdown-parser");
const { join } = require("path");

const filePath = join(__dirname, "test.md");
const result = processMarkdown(filePath, false);

console.log(result); // => export default `<html_content>`;
```

#### TypeScript

```ts
import { processMarkdown } from "rs-markdown-parser";
import { join } from "path";

const filePath: string = join(__dirname, "test.md");
const result: string = processMarkdown(filePath, false);

console.log(result); // => export default `<html_content>`;
```

---

## 🧪 Example

### Sample Markdown (`test.md`)

```markdown
# Hello, World!

This is a **Markdown** file.

- Item 1
- Item 2
```

### Output (ES Module String)

```js
export default `<h1>Hello, World!</h1>\n<p>This is a <strong>Markdown</strong> file.</p>\n<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>`;
```

---

## 🧬 GitHub Flavored Markdown (GFM)

Enable extended Markdown features with the `gfm` flag:

```js
const result = processMarkdown(filePath, true);
```

Supported GFM features:

* ✔️ Task lists
* 📑 Footnotes
* 🔡 Strikethrough (`~~text~~`)
* 🧮 Tables
* 🏷️ Heading attributes (`## Title {#id}`)

---

## 🔷 TypeScript Support

This module includes `index.d.ts` for full TypeScript support. To use:

1. Ensure TypeScript is installed:

   ```bash
   npm install --save-dev typescript
   ```

2. Import the module as usual. Type definitions are automatically applied.

---

## 🧪 Development & Testing

### Run tests:

```bash
npm run test
```

### Rebuild after source changes:

* Edit Rust: `src/lib.rs`
* Edit Types: `index.d.ts`

Then:

```bash
npm run debug
# or
npm run build
```

---

## 📄 License

MIT License — see [LICENSE](./LICENSE).

---

## 🤝 Contributing

Contributions welcome! Please open an issue or PR:

👉 [GitHub Repository](https://github.com/aliezzahn/rs-markdown-parser)

---

## 🐞 Issues & Support

Found a bug or need a feature?

📬 Report it on [GitHub Issues](https://github.com/aliezzahn/rs-markdown-parser/issues)

## 👥 Contributors

Thanks to these awesome people for their work on this project:

<table>
  <tr>
    <td align="center">
      <a href="https://github.com/aliezzahn">
        <img src="https://avatars.githubusercontent.com/u/164005474?v=4" width="100px;" alt="aliezzahn" />
        <br /><sub><b>aliezzahn</b></sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/0xre2a">
        <img src="https://avatars.githubusercontent.com/u/90304241?v=4" width="100px;" alt="0xre2a" />
        <br /><sub><b>0xre2a</b></sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/morihn">
        <img src="https://avatars.githubusercontent.com/u/191381570?v=4" width="100px;" alt="morihn" />
        <br /><sub><b>morihn</b></sub>
      </a>
    </td>
  </tr>
</table>

