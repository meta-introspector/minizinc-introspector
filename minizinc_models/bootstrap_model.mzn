% bootstrap_model.mzn

% Define the set of bootstrap tasks
enum Task = {
    BUILD_GECODE,
    BUILD_LIBMINIZINC,
    GENERATE_FFI_DECLARATIONS,
    BUILD_CPP_FFI_WRAPPER,
    BUILD_RUST_FFI_CRATE,
    SETUP_SOLVERS,
    TEST_C_ABI,
    TEST_RUST_FFI,
    TEST_MINIZINC_MODELS,
    RUN_INITIAL_EMBEDDING
};

set of Task: all_tasks = Task;

% Variables: start time for each task
array[Task] of var int: start_time;
array[Task] of var int: end_time;
array[Task] of int: duration; % Estimated duration for each task

% Constraints: Dependencies
constraint start_time[BUILD_LIBMINIZINC] >= end_time[BUILD_GECODE];
constraint start_time[BUILD_CPP_FFI_WRAPPER] >= end_time[BUILD_LIBMINIZINC];
constraint start_time[BUILD_RUST_FFI_CRATE] >= end_time[BUILD_CPP_FFI_WRAPPER];
constraint start_time[SETUP_SOLVERS] >= end_time[BUILD_LIBMINIZINC]; % Solvers depend on libminizinc
constraint start_time[TEST_C_ABI] >= end_time[BUILD_CPP_FFI_WRAPPER];
constraint start_time[TEST_RUST_FFI] >= end_time[BUILD_RUST_FFI_CRATE];
constraint start_time[TEST_MINIZINC_MODELS] >= end_time[BUILD_LIBMINIZINC];
constraint start_time[RUN_INITIAL_EMBEDDING] >= end_time[TEST_RUST_FFI];
constraint start_time[RUN_INITIAL_EMBEDDING] >= end_time[TEST_MINIZINC_MODELS];

% Objective: Minimize total time
var int: total_bootstrap_time = max(t in all_tasks)(end_time[t]);
solve minimize total_bootstrap_time;

% Output the schedule
output [
    "Task: " ++ show(t) ++ ", Start: " ++ show(start_time[t]) ++ ", End: " ++ show(end_time[t]) ++ "\n"
    | t in all_tasks
] ++ ["Total Bootstrap Time: " ++ show(total_bootstrap_time) ++ "\n"];

