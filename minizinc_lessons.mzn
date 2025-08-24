% minizinc_lessons.mzn
% This file demonstrates good practices and common patterns in MiniZinc.

% Lesson 1: Correct Function Declaration Syntax
% Functions in MiniZinc are declared as: return_type: function_name(param_type: param_name)
% Example:
function int: add_two_numbers(int: a, int: b) = a + b;

% Lesson 2: Using Enums for Discrete Sets
% Enums provide a clear way to define a fixed set of named values.
enum Color = {Red, Green, Blue};

% Lesson 3: Using Records for Structured Data
% MiniZinc supports record types, which allow you to group named fields of different types.
% You can define a record type using the 'type' keyword.
type Point = record(var int: x, var int: y);

% Declare a variable of the Point record type
Point: my_point_record;

% Lesson 4: Using Arrays
% Arrays are used to store collections of elements of the same type.
array[1..3] of int: my_list = [10, 20, 30];

% Lesson 5: Basic Solve and Output
% solve satisfy; finds any solution.
% solve minimize/maximize expression; finds an optimal solution.
% output statements control what is printed.

var int: result_add_two;
constraint result_add_two = add_two_numbers(5, 7);

var Color: chosen_color;
constraint chosen_color = Green;

% Assigning values to the record fields
constraint my_point_record.x = my_list[1];
constraint my_point_record.y = my_list[3];

solve satisfy;

output [
    "Result of add_two_numbers(5, 7): ", show(result_add_two), "\n",
    "Chosen Color: ", show(chosen_color), "\n",
    "My Point: (", show(my_point_record.x), ", ", show(my_point_record.y), ")\n"
];
