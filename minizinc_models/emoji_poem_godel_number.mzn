% MiniZinc model to encode the emoji poem as a Gödel number

% Array of primes corresponding to the emoji sequence
array[1..41] of int: primes = [
    2, 3, 5,      %   
    7, 11, 13,    %   
    17, 19, 23,   %   
    29, 31, 37,   %   
    41, 11, 43, 47, 37, 2, 13, 7, 19,  % ✍️   ➡️     
    47, 13, 53, 59,                    % ➡️   
    61, 67, 47, 71, 37, 73,           % ️  ➡️   
    79, 83, 89, 97, 101, 103,         %  ‍    
    107, 11, 113, 127, 109, 53         % ➕     
];

% Array of exponents (position indices, 1-based)
array[1..41] of int: exponents = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41];

% Variable to store the Gödel number
var int: godel_number;

% Constraint to compute the Gödel number as the product of prime^exponent
constraint godel_number = prod(i in 1..41)(pow(primes[i], exponents[i]));

% Solve to compute the Gödel number
solve satisfy;

% Output the result
output ["Gödel Number: ", show(godel_number)];