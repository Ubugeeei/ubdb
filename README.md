# ubdb

A Toy RDBMS written in Rust.

## Usage

```bash
$ cargo run -- -i

> SET a = 1;

> SELECT a;
a: 1

> SET a = 2, b = 3, c = 999;

> SELECT *;
a: 2
b: 3
c: 999

> SELECT a, b;
a: 2
b: 3
```

# Log

https://github.com/Ubugeeei/work-log/discussions/197
