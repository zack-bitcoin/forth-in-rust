# Forth In Rust

An implementation of the forth programming language written in rust.

[The opcodes are documented](opcodes.md)

The [forth compiler I wrote in erlang from a previous project](erlang_compiler.md). All the compiled code is provided, you don't need erlang to test the vm. 

You can compile the vm with cargo like this:

```sh
$ cargo build --release
```

There are a [couple examples](examples) of compiled forth code with suffix ".bytes"

Try running one like this:

```
./target/release/vm examples/euler001.bytes
```


