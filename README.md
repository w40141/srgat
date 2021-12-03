# srgat

ファイル内のコメントに書かれたタグを管理する CLI

[![Rust](https://github.com/w40141/srgat/actions/workflows/rust.yml/badge.svg)](https://github.com/w40141/srgat/actions/workflows/rust.yml)
![GitHub](https://img.shields.io/github/license/w40141/srgat)
![Codecov](https://img.shields.io/codecov/c/github/w40141/srgat)

## Installation

```
brew install srgat
```

```
cargo install srgat
```

## How to use

- Display documentation

```
srgat -h (or --help)
```

- Display tags in the files (fuga.rs)

```
srgat -f (or --file) fuga.rs
```

- Display tags in the directory (./scr)

```
srgat -r (--recursively) ./scr
```

- Ignore the files (hoge.rs)

```
srgat -i hoge.rs
```

- Display the all saved tags in default file or target file

```
srgat -s (or --show) (target_file)
```

- Display the saved tags in the current directory

```
srgat -d (or --dump) save_file -f fuga.rs
```

## Tags

- TODO:
- INFO:
- FIX:

## Configuration
