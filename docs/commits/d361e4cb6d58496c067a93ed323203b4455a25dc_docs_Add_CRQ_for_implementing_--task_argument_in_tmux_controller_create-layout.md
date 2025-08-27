docs: Add CRQ for implementing --task argument in tmux_controller create-layout
This commit introduces a Change Request (CRQ) document outlining the
proposal to enhance the `tmux_controller create-layout` command. The
proposed enhancement involves adding a `--task` argument, allowing
users to specify a task (e.g., `crq-updater`) to be automatically
executed in a designated tmux pane upon layout creation.

This CRQ details the objective, scope, impact, verification steps, and
a high-level proposed implementation for this new feature.