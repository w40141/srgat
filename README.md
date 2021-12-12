# srgat

ファイル内のコメントに書かれたタグを管理する CLI

[![Rust](https://github.com/w40141/srgat/actions/workflows/rust.yml/badge.svg)](https://github.com/w40141/srgat/actions/workflows/rust.yml)
![GitHub](https://img.shields.io/github/license/w40141/srgat)
![Codecov](https://img.shields.io/codecov/c/github/w40141/srgat)
![creato]

## Installation

```
brew install srgat
```

```
cargo install srgat
```

## How to use

- Prints the all saved tags in default file or target file

```
srgat (target.file)
```

- Prints help information

```
srgat -h (or --help)
```

- Prints tags in the file(s) (fuga.rs)

```
srgat -f (or --files) fuga.rs
```

- Prints tags in the directory (./scr)

```
srgat -r (--recursively) ./scr
```

- Ignore the file(s) (hoge.rs)  
  NOTE: If you use only this flag, srgat run `srgat`.

```
srgat -f ./scr -i hoge.txt
```

- TODO: Prints the all saved tags in default file or target file

```
srgat -t (or --type) (json, csv, table)
```

## Tags

- TODO:
- INFO:
- FIX:
- WARNING:
- NOTE:
- HACK:
- PERF:

## Configuration
