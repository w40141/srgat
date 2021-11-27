# srgat

ファイル内のコメントに書かれたタグを管理する CLI

## How to use.

- ヘルプを表示する。

```
srgat -h (or --help)
```

- 指定したファイル内 (fuga.rs) のタグを表示する。

```
srgat -f (or --file) fuga.rs
```

- 指定したディレクトリにあるファイル内のタグを表示する。

```
srgat -r (--recursively) ./scr
```

- 指定したファイル (hoge.rs) を無視する。

```
srgat -i hoge.rs
```

- 保存されているタグを一覧で表示する。

```
srgat -s (or --show)
```

- カレントディレクトリにある保存されている

```
srgat -d (or --dump) -f fuga.rs
```
