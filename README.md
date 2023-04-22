# actix-web benchmark

Simple actix-web benchmark for [this](https://yrashk.com/blog/2023/02/16/what-happens-if-you-put-http-server-inside-postgres) blog post

## How to build and run the server

To build an optimized production version, run:

```shell
cargo build --release
```

Then run the server:

```shell
./target/release/actix-benchmark
```
