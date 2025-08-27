% test_clamp01.mzn
include "clamp01.mzn";

float: test_val1 = 0.5;
float: test_val2 = 1.5;
float: test_val3 = -0.5;

solve satisfy;
output [
  "clamp01(", show(test_val1), ") = ", show(clamp01(test_val1)), "\n",
  "clamp01(", show(test_val2), ") = ", show(clamp01(test_val2)), "\n",
  "clamp01(", show(test_val3), ") = ", show(clamp01(test_val3)), "\n"
];