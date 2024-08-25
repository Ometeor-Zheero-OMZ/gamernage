# 文字列をログ出力したい場合

```rust
logger::log(logger::Header::INFO, "called get_todos");
```

# DB から取得した値をログ出力したい場合

```rust
logger::log(logger::Header::INFO, &format!("{:?}", row));
```
