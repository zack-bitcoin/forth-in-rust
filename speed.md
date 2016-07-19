The rust one was built for speed. The erlang one was built to be the virtual machine for a cryptocurrency. I expect it to be at least 2x slower because of the cryptocurrency features it supports.

measuring speed for rust version

zack@iloSona:~/Hacking/rust/forth$ time ./target/release/vm 
233168

real	0m0.007s
user	0m0.004s
sys	0m0.000s


measureing speed for erlang version

4> {ok, A} = file:read_file("src/vm/euler_problems/001.fs").
{ok,<<"( If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of"...>>}
5> B = compiler:compiler(A).
** exception error: undefined function compiler:compiler/1
6> B = compiler:compile(A).
[36,40,11,39,
 {integer,0},
 35,17,18,40,11,39,
 {integer,3},
 43,
 {integer,0},
 35,40,11,39,
 {integer,5},
 43,
 {integer,0},
 35,21,17,40,11,39,2,18|...]
7> timer:tc(language, run [B, 1000000000]).
* 1: syntax error before: '['
7> timer:tc(language, run, [B, 1000000000]).
{24900,[233168]}


erlang is measuring microseconds, so that is the same as
0.0249 seconds.

The rust version was
0.004

So the rust version is 6.25 times faster at solving this problem.

The problem being solved was the first project euler problem which is:
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
-> Find the sum of all the multiples of 3 or 5 below 1000. 

The code for the forth vm written in erlang looked like this:

```
: f r@ integer 0 == if else
  r@ integer 3 rem integer 0 ==
  r@ integer 5 rem integer 0 ==
  or if r@ + else then
  r> integer 1 - >r recurse call 
then ;
integer 999 >r integer 0 f call
```

The code for the forth vm written in rust looked like this:

```
: g if r@ + else then ;

: f r@ pushn 0 == if else
  r@ pushn 3 %  pushn 0 ==
  r@ pushn 5 % pushn 0 ==
  or pushn g call
  r> pushn 1 - >r pushn recurse call then ;
push1 231 3 0 0 >r pushn 0 pushn f call print
( pushn 9 >r pushn 0 pushn f call print )
```