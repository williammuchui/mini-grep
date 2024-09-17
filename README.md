# Mini-grep

## Arguments
- __needle__
- __haystack__

```bash
cargo run -- "lorem" lorem
```

## Ignorecase
Case sensitivity is  enabled by default.
To disable case sensitivity set environment variable __ignore_case__ to __true__.

```bash
IGNORE_CASE=1 cargo run -- 'lorem' lorem
```

## Runnnig Tests
```bash
cargo test
```

