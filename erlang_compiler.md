To compile the code, open erlang interpreter with `erl`
Now you can compile the compiler. It was written in erlang.

```
1> c(compiler).
{ok,compiler)
```

Now when the compiler is compiled, you can use it to compile forth code, like the "power.fs" file.
After it is compiled, erlang uses "vm" to run the code, and displays the answer.

```
2> compiler:doit("examples/power.fs", "examples/power.bytes").
2048
ok
3>
```

There are a [couple examples](examples) of forth code that compiles correctly