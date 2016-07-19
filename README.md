# forth-in-rust

an implementation of the forth programming language written in rust.

[the opcodes are documented](opcodes.md)

you can compile the vm with rustc like this:

```
rustc vm.rs
```

There are a [couple](power.fs) [examples](code.fs) of forth code that compiles correctly.

to compile the code, open erlang interpreter with `erl`
Now you can compile the compiler. It was written in erlang.

```
1> c(compiler).
{ok,compiler)
```

now that the compiler is compiled, you can use it to compile forth code, like the "power.fs" file.
After it is compiled, erlang uses "vm" to run the code, and displays the answer.

```
2> compiler:doit("power.fs").
2048
ok
3>
```
