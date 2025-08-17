% git_history_model.mzn

% --- Data Structures ---

% Represents the type of change to a file
enum ChangeType = {Added, Deleted, Modified};

% Represents a single file change within a commit
record FileChange = {
    string file_path,
    ChangeType change_type
};

% Represents a single commit
record Commit = {
    string hash,
    string author,
    string date,
    string subject,
    array[int] of FileChange changes
};

% --- Commit Data (from git_history_data.dzn) ---
include "git_history_data.dzn";
array[int] of Commit: all_commits;

% --- Conceptual Queries/Analysis ---

% Find all commits by a specific author
function array[int] of Commit: commits_by_author(string target_author) =
    [c | c in all_commits where c.author = target_author];

% Find all commits that modified a specific file
function array[int] of Commit: commits_modifying_file(string target_file_path) =
    [c | c in all_commits, f in c.changes where f.file_path = target_file_path /\ f.change_type = Modified];

% Find all commits that added a new file
function array[int] of Commit: commits_adding_file() =
    [c | c in all_commits, f in c.changes where f.change_type = Added];

% Find all commits that deleted a file
function array[int] of Commit: commits_deleting_file() =
    [c | c in all_commits, f in c.changes where f.change_type = Deleted];

% --- Solve and Output ---
solve satisfy;

output [
    "--- Git History Analysis ---\\n",
    "Total Commits: ", show(length(all_commits)), "\\n",
    "\\nCommits by Mike Dupont:\\n"
] ++
[
    "  Hash: ", c.hash, "\\n",
    "  Subject: ", c.subject, "\\n",
    "  Date: ", c.date, "\\n",
    "  Changes: ", show(c.changes), "\\n\\n"
    | c in commits_by_author("mike dupont")
] ++
[
    "\\nCommits that added new files:\\n"
] ++
[
    "  Hash: ", c.hash, "\\n",
    "  Subject: ", c.subject, "\\n",
    "  Files Added: ", show([f.file_path | f in c.changes where f.change_type = Added]), "\\n\\n"
    | c in commits_adding_file()
] ++
[
    "\\nCommits that deleted files:\\n"
] ++
[
    "  Hash: ", c.hash, "\\n",
    "  Subject: ", c.subject, "\\n",
    "  Files Deleted: ", show([f.file_path | f in c.changes where f.change_type = Deleted]), "\\n\\n"
    | c in commits_deleting_file()
];