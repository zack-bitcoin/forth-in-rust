 : fib dup pushn 0 == if drop else pushn 1 - >r dup rot + r> pushn recurse call then ; 

pushn 1 pushn 1 pushn 20 pushn fib call print 