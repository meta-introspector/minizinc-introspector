function int: test_func(int: x) = x + 1;

int: a = test_func(5);

solve satisfy;

output [show(a)];