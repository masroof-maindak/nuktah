# nuktah

![Banner](.github/assets/logo.svg)

Nuktah is a programming language. It is quite possibly the most based one to ever exist. Here's what a sample looks like:

```
fn ginti some_fn() {
	duhrao (ginti a = 0 . a < 10 . a = a + 1) {
		agar (a == 5) { toro } warna {}
	}

	wapsi 5 .
} .
```

## Installation & Usage

- Ensure you have [`cargo` and `rust`](https://www.rust-lang.org/tools/install) (build dependencies) installed and added to your $PATH

```bash
git clone https://github.com/masroof-maindak/nuktah.git
cd nuktah
cargo build -r
./target/release/nktc <src.nkt>
```

## TODO

- [x] Support for comments
- [x] Fix string literal tokenization
- [x] Add UTF8 support for language; deprecate use of `bytes`
- [x] Convert concrete syntax tree to AST
- [x] AST pretty-printing
- [x] Transliterated Urdu keywords
- [x] Lexer: Combine `T_INT T_DOT T_INT` to `T_FLOAT`
- [x] Break keyword - `toro`
- [x] Allow empty expressions so `duhrao (..)` is valid
- [x] Void type - `khali`
- [ ] Init semantic analyser
	- [ ] Scope
	- [ ] Type-checker
	- [ ] ???
- [ ] Init IR generation
- [ ] Arrays
- [ ] Structs
- [ ] Rewrite expression printing rules (for the AST) w/ macros
- [ ] Eliminate Rust anti-pattern: String cloning w.r.t `Token::StringLit`
- [ ] Combine all expression precedence functions into one and use a table
