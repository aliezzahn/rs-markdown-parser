use neon::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use std::fs;

fn process_markdown(mut cx: FunctionContext) -> JsResult<JsString> {
    // Get the markdown file path and options from the loader
    let file_path = cx.argument::<JsString>(0)?.value(&mut cx);
    let gfm = cx.argument::<JsBoolean>(1)?.value(&mut cx);

    // Read the markdown file content
    let markdown = fs::read_to_string(&file_path)
        .or_else(|e| cx.throw_error(format!("Failed to read file {}: {}", file_path, e)))?;

    // Configure pulldown-cmark options
    let mut opts = Options::empty();
    if gfm {
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TASKLISTS);
        opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }

    // Parse markdown to HTML
    let parser = Parser::new_ext(&markdown, opts);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Export as ES module
    let js_output = format!("export default `{}`;", html_output);

    Ok(cx.string(js_output))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("processMarkdown", process_markdown)?;
    Ok(())
}