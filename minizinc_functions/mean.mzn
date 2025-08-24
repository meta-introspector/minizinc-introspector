% mean.mzn
% Computes the mean of a list of real numbers
type Meme = array[int] of float;
function float: mean(xs: Meme) = sum(xs) / int2float(card(xs));