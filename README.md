<h1 align="center">
    arith-eval
</h1>

> Evaluates arithmetic expressions

## Table of Contents
- [Requirements](##requirements)
- [Install](##install)
- [Usage](##usage)
- [Contributing](##contributing)
- [License](##license)

## Requirements
- [Rust](https://www.rust-lang.org/)

## Install
```
git clone https://github.com/TrAyZeN/arith-eval.git
cd arith-eval
cargo build --release
```

## Usage
```
./arith-eval 12 * (5 - 2) % 4 + 8
```

## Grammar
The following grammar is read by the parser. Terminals are single-quoted.

```rust
expr ::= term term_follow

term_follow ::= '+' term term_follow
              | '-' term term_follow
              | Ɛ

term ::= factor factor_follow

factor_follow ::= '*' factor factor_follow
                | '/' factor factor_follow
                | '%' factor factor_follow
                | Ɛ

factor ::= '(' expr ')'
         | number

number ::= digit | digit number

digit ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
```

## Contributing
Contribution are welcomed

## License
license TrAyZeN
