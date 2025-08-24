% test_variance.mzn
include "variance.mzn";

type Meme = array[int] of float;

Meme: test_meme1 = [1.0, 2.0, 3.0]; % Mean = 2, Variance = (1+0+1)/3 = 0.666...
Meme: test_meme2 = [10.0, 10.0, 10.0]; % Mean = 10, Variance = 0
Meme: test_meme3 = [];

solve satisfy;
output [
  "variance(", show(test_meme1), ") = ", show(variance(test_meme1)), "\n",
  "variance(", show(test_meme2), ") = ", show(variance(test_meme2)), "\n",
  "variance(", show(test_meme3), ") = ", show(variance(test_meme3)), "\n"
];
