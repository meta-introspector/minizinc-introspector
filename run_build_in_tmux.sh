#!/bin/bash

# Ensure launchpad is built
cargo build --package launchpad

# Split the window
cargo run --package launchpad -- tmux split-window -h

# Send the build command
cargo run --package launchpad -- tmux send-keys "cargo build --package crq_updater" C-m

# Switch back to the original pane
cargo run --package launchpad -- tmux select-pane -t :.-1
