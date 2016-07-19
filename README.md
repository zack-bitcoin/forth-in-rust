# Forth In Rust

An implementation of the forth programming language written in rust.

[The opcodes are documented](opcodes.md)

You can compile the vm with cargo like this:

```sh
$ cargo build --release
```

There are a [couple examples](examples) of forth code that compiles correctly.

To compile the code, open erlang interpreter with `erl`
Now you can compile the compiler. It was written in erlang.

```
1> c(compiler).
{ok,compiler)
```

Now when the compiler is compiled, you can use it to compile forth code, like the "power.fs" file.
After it is compiled, erlang uses "vm" to run the code, and displays the answer.

```
2> compiler:doit("examples/power.fs").
2048
ok
3>
```
