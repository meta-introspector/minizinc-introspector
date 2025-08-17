enum TestType = {A, B};

record TestRecord = {
    string name,
    TestType type
};

set of int: RECORD_INDICES = 1..2; % Explicitly define the index set

array[RECORD_INDICES] of TestRecord all_records = [
    (name: "Record1", type: A),
    (name: "Record2", type: B)
];

solve satisfy;

output [show(all_records)];