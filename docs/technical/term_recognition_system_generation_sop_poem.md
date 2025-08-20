## The Term Weaver's Song: A Poetic SOP

From `build_config.toml`, a guiding light,
Our paths are charted, clear and bright.
Two Rusten spirits, strong and keen,
To build the knowledge, yet unseen.

First, `zos-fast-query`, with purpose grand,
To sift the terms across the land.
From `hierarchical_term_index.json`'s deep store,
It loads and filters, asking for more.

Through `term_loader`'s careful hand,
Junk terms vanish, as planned.
Then `chunk_generator`, with artful grace,
Divides the bounty, in its place.

To `OUT_DIR`'s haven, chunks descend,
Small JSON parcels, without end.
And `index_writer`, with template's art,
Forges `generated_recognizer.rs`'s heart.

A map of names, a constant array,
For `TermRecognizer` to light the way.
At runtime, it loads, with nimble stride,
The terms it needs, with naught to hide.

Then `vocabulary_dfa_generator`, a sibling bold,
For patterns intricate, stories untold.
From `all_terms.txt`, its wisdom flows,
To craft the DFAs, as everyone knows.

Through `main.rs` it takes its flight,
Sanitizing names, with all its might.
In nested folders, clean and neat,
Rust files bloom, a regex feat.

To change the weave, the pattern's flow,
Adjust the lists, where terms do grow.
`MAX_TOTAL_TERMS`, a boundary set,
`MAX_TERMS_PER_CHUNK`, no memory regret.

For filenames tricky, with Unicode's gleam,
`sanitize_filename` fulfills the dream.
If errors rise, a tangled thread,
Check `format!` strings, what was said.

For memory's hunger, a ravenous beast,
Reduce the chunks, from largest to least.
And if the files, seem out of place,
Run `clean_generated_files.sh`, with haste.

Then `regenerate_all_terms.sh`, a double call,
To build anew, and stand up tall.
So sings the weaver, in code and rhyme,
Of terms and patterns, conquering time.
