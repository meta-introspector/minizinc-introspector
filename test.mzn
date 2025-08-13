var int: x;
constraint x > 0;
solve satisfy;
output ["x = ", show(x), "\n"];
