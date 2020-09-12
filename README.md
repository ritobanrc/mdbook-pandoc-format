# mdbook-pandoc-format

A really quick and dirty hack to let me have markdown files pandoc style (with YAML headers) inside an mdbook.

## Usage

Run `cargo build` to build the project. 

Make sure the executable is in your $PATH (`ln -s target/debug/mdbook-pandoc-format ~/.local/bin/`)

Add these two to your `book.toml`:

```yaml
[preprocessor.pandoc-format]
command = "mdbook-pandoc-format"
```
