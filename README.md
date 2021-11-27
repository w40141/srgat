# srgat

ファイル内のコメントに書かれたタグを管理する CLI

## Installation

```
brew install srgat
```

```
cargo install srgat
```

## How to use.

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

- Display the all saved tags

```
srgat -s (or --show)
```

- Display the saved tabs in the current directory

```
srgat -d (or --dump) -f fuga.rs
```

## Configuration
