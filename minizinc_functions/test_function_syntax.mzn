% Test function syntax with float types based on official docs

function var float: add_one_float(var float: x) = x + 1.0;

solve satisfy;

output ["Result: ", show(add_one_float(5.0)), "\n"];