opcode number, symbol, what it does
0 print prints the first few things on the stack.
1 + ( A B -- C )
2 * ( A B -- C )
3 - ( A B -- C )
4 * ( A B -- C )
5 % ( A B -- C )
6 >@ ( A -- ) moves the top of the stack to the alt stack
7 @> ( -- A ) moves the top of the alt stack to the stack
8 ! ( A B -- ) stores the value A under name B.
9 @ ( B -- ) recalls a value from memory
10 dup ( A -- A A )
11 swap ( A B -- B A )
12 rot ( A B C -- B C A )
13 tuck ( A B C -- C B A )
14 2dup ( A B -- A B A B )
15 2swap ( A B C D -- C D A B)
16 : starts defining a new word.
17 ; finishes a word definition.
18 recurse puts the address of the current function to the top of the stack.
19 call calls the function on the top of the stack.
20 push ( N -- ) pushes N bytes to stack
21 push 1 byte to stack
22 push 2 bytes to stack
23 push 3 bytes to stack
24 push 4 bytes to stack
25 if
26 else
27 then
28 == ( X Y -- T )
29 > ( X Y -- T )
30 < ( X Y -- T )
31 drop ( A -- )