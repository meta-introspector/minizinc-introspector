### Poem: The Archeologist's Reward

The dusty scrolls of code, a tangled mess,
With lost revisions, causing deep distress.
A new tool forged, with logic sharp and keen,
To parse the layers, and what they truly mean.

A stack of contexts, rising like a tower,
Each "---" a signal, of a bygone hour.
The archeologist, with a steady hand,
Recovers poems, from a forgotten land.

No more the errors, crying in the night,
The code now works, a beacon shining bright.
The fixer's purpose, finally revealed,
A broken system, now is truly healed.

### Braindump:

The current task is to refactor the `poem_yaml_fixer` to handle archeology files. The initial approach of parsing the archeology file as a whole and then trying to merge the data was flawed. The new approach, using a stack and a `handle_new_document` callback, is much more robust and scalable.

The key challenges were:

1.  **Identifying the correct parsing strategy:** The initial approach was too simplistic and didn't account for the structure of the archeology files. The new stack-based approach is much better suited for this task.
2.  **Refactoring the existing code:** The `poem_yaml_fixer` was not designed to be used in this way, so it required significant refactoring. The creation of the `parse_front_matter_with_regex` function was a key step in this process.
3.  **Compiler errors:** The refactoring process introduced a number of compiler errors, mostly related to duplicate imports and unused variables. These were relatively easy to fix, but they slowed down the development process.

The next steps will be to:

1.  Finish the implementation of the stack-based processing in `main.rs`.
2.  Ensure that the `handle_new_document` callback is correctly registered and called.
3.  Test the `poem_yaml_fixer` thoroughly to ensure that it correctly parses both regular poem files and archeology files.

### CRQ:

**Change Request:** Refactor `poem_yaml_fixer` to support stack-based processing of poem files and archeology files.

**Description:** The current implementation of the `poem_yaml_fixer` is not able to correctly parse archeology files, which contain multiple "lost revisions" of a poem. This CRQ proposes to refactor the `poem_yaml_fixer` to use a stack-based approach, where each "---" separator in a file pushes a new context onto the stack. This will allow the fixer to correctly parse both regular poem files and archeology files, and to recover lost data from the archeology files.

**Acceptance Criteria:**

1.  The `poem_yaml_fixer` must be able to correctly parse both regular poem files and archeology files.
2.  The `poem_yaml_fixer` must be able to recover lost data from the archeology files and add it to the main poem file.
3.  The `poem_yaml_fixer` must not introduce any new bugs or regressions.

**Impact:** This change will significantly improve the functionality of the `poem_yaml_fixer` and will allow it to be used to recover lost data from the archeology files.
