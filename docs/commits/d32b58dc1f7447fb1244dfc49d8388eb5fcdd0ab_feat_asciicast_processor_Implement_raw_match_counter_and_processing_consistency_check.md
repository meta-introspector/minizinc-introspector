feat(asciicast_processor): Implement raw match counter and processing consistency check

This commit implements a raw match counter and a processing consistency check within the `asciicast_processor` crate, significantly enhancing its debugging and verification capabilities.

Key changes include:
- Introduction of the `count-raw` subcommand for direct regex pattern matching in raw asciicast input.
- Implementation of an implicit panic in the `Filter` subcommand: if a regex pattern is found in the raw input but not in the processed output, the application will panic, ensuring data integrity.
- Refactoring of `raw_parser.rs` to provide a utility function (`check_raw_matches`) for checking raw matches, separating it from printing logic.

This work is crucial for ensuring the reliability and accuracy of asciicast processing, particularly in identifying discrepancies between raw and cleaned data.