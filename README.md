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

- The shrimplest way to get up and running with the Nuktah compiler is downloading an automatically generated release (x86_64 Linux only)
- To build locally, ensure you have [`cargo` and `rust`](https://www.rust-lang.org/tools/install) installed and added to your $PATH

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
- [x] Scope Analyser
    - [x] SpaghettiStack skeleton
    - [x] Analyse declarations
    - [x] Analyse blocks
    - [x] Analyse for/if
    - [x] Analyse `PrimaryExpr::Ident`
    - [x] Iterate up stack - parent 'climbing'? -> store Ids rather than references
- [x] Refactor into separate sub-modules
- [x] CI/CD -> Build/Release
- [x] Use macros for error conversions
- [x] Type-checker
    - [x] Store whether symbol belongs to var/func in symbol table
    - [x] Store type of scope in ScopeMap
    - [x] Variable declarations -> Literals' type == token type
    - [x] Function definitions -> return type == token type
    - [x] Function to get id of nth scope-type child, of a given node
    - [x] Type-checking of for/if
    - [x] Recursive-descent expression chain
    - [x] Type-check arguments to functions
- [x] **bug**: void type semantic analysis
- [x] **bug**: parsing of string declarations
- [x] Add a boolean type
- [ ] Eliminate `mod.rs` files
- [ ] Unit tests
- [ ] `ir_gen`
- [ ] `asm_gen` -> ARM? MIPS?
- [ ] Arrays
- [ ] Structs
- [ ] Rewrite expression printing rules (for the AST) w/ macros
- [ ] Eliminate Rust anti-pattern: String cloning w.r.t `Token::StringLit`

## Acknowledgements

#### Parsing

- [Compiler Construction - Niklaus Wirth (Chapter #4.1)](https://people.inf.ethz.ch/wirth/CompilerConstruction/CompilerConstruction1.pdf)
- The Dragon Book
- [C's grammar](https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.htm)
- [Simple but Powerful Pratt Parsing - Matklad](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [Parsing Expressions by Precedence Climbing - Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing)
- [Parsing Expressions by Recursive Descent - Theodore Norvell](https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm)
