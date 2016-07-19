: power dup pushn 0 == if drop else >r pushn 2 * r> pushn 1 - pushn recurse call then ;
pushn 2 pushn 10 pushn power call print