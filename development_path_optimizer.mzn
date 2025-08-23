% --- Parameters ---
% Number of tasks in our simplified development path
int: num_tasks;
set of int: TASKS = 1..num_tasks;

% Duration of each task (e.g., in hours or abstract units)
array[TASKS] of int: task_duration;

% Dependencies: task_i depends on task_j if depends[i,j] is true
array[TASKS, TASKS] of bool: depends;

% Resource consumption per task (e.g., LLM agent time, CPU time)
array[TASKS] of int: llm_agent_time_cost;
array[TASKS] of int: cpu_time_cost;

% --- Decision Variables ---
% Start time of each task
array[TASKS] of var int: start_time;

% End time of each task
array[TASKS] of var int: end_time;

% --- Constraints ---
constraint forall(i in TASKS) (
    end_time[i] = start_time[i] + task_duration[i]
);

% Dependency constraints: If task i depends on task j, task i must start after task j ends
constraint forall(i in TASKS, j in TASKS where depends[i,j]) (
    start_time[i] >= end_time[j]
);

% All tasks must start at or after time 0
constraint forall(i in TASKS) (
    start_time[i] >= 0
);

% --- Objective Function ---
% Minimize the total project duration (makespan)
var int: total_project_duration = max(i in TASKS)(end_time[i]);
solve minimize total_project_duration;

% --- Output ---
output [
    "Task Schedule:\n",
    "----------------\n"
] ++ 
[
    "Task " ++ show(i) ++ ": Start = " ++ show(start_time[i]) ++ ", End = " ++ show(end_time[i]) ++ "\n"
    | i in TASKS
] ++ 
[
    "----------------\n",
    "Total Project Duration: " ++ show(total_project_duration) ++ "\n"
];
