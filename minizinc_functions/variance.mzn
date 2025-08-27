% variance.mzn
% Computes the variance of a list of real numbers (simplified)
type Meme = array[int] of float;
function float: variance(xs: Meme) =
    let {
        float_mean = sum(xs) / int2float(card(xs))
    } in
    sum(i in index_set(xs)) (pow(xs[i] - float_mean, 2)) / int2float(card(xs));