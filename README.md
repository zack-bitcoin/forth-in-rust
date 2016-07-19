# forth-in-rust

an implementation of the forth programming language written in rust.

[the opcodes are documented](opcodes.md)

you can compile the vm with rustc like this:

```
rustc vm.rs
```

There are a [couple](power.fs) [examples](code.fs) of forth code that compiles correctly.

to try running the code, open erlang interpreter with `erl`

```
1> compiler:doit("power.fs").
"2048\n"

2> compiler:doit("euler001.fs"). %takes several minutes
"223168\n"
```