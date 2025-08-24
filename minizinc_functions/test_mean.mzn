% test_mean.mzn
include "mean.mzn";

type Meme = array[int] of float;

Meme: test_meme1 = [1.0, 2.0, 3.0];
Meme: test_meme2 = [10.0, 20.0];
Meme: test_meme3 = [];

solve satisfy;
output [
  "mean(", show(test_meme1), ") = ", show(mean(test_meme1)), "\n",
  "mean(", show(test_meme2), ") = ", show(mean(test_meme2)), "\n",
  "mean(", show(test_meme3), ") = ", show(mean(test_meme3)), "\n"
];
