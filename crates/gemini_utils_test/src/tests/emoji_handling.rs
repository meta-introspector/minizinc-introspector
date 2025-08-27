use gemini_utils::gemini_eprintln;

#[test]
fn test_named_argument_with_emoji() {
    let my_value = 42;
    // The emoji needs a named placeholder in the format string
    gemini_eprintln!("The answer is: :magnifying_glass:", magnifying_glass = my_value);
}

#[test]
fn test_positional_argument_with_emoji() {
    let my_string = "hello world";
    // The emoji needs a named placeholder in the format string
    gemini_eprintln!("A message: :magnifying_glass:", magnifying_glass = my_string);
}

#[test]
fn test_named_emojis_and_explicit_naming() {
    // Emojis need explicit named placeholders in the format string
    gemini_eprintln!("This is a test with named emojis. :sparkles: :brickwall: ::newline:: Another line.",
        sparkles = "", // Dummy argument
        brickwall = "", // Dummy argument
    );
    gemini_eprintln!(":rocket: Launching simulation... :hourglass_flowing_sand:",
        rocket = "", // Dummy argument
        hourglass_flowing_sand = "", // Dummy argument
    );
}

#[test]
fn test_multiple_same_emojis_explicit_naming() {
    // Multiple same emojis need distinct named placeholders
    gemini_eprintln!("Inspect this: :magnifying_glass_1: :magnifying_glass_2: :magnifying_glass_3:",
        magnifying_glass_1 = "first",
        magnifying_glass_2 = "second",
        magnifying_glass_3 = "third"
    );
    gemini_eprintln!("Rocket launch sequence: :rocket_1: :rocket_2: :rocket_3:",
        rocket_1 = "stage 1",
        rocket_2 = "stage 2",
        rocket_3 = "stage 3"
    );
}

#[test]
fn test_mixed_named_args_and_emojis() {
    let task_id = 123;
    let status = "completed";
    gemini_eprintln!(":white_check_mark: Task :task_id: :rocket: status: :status: :sparkles:",
        white_check_mark = "", // Dummy argument
        task_id = task_id,
        rocket = "launch",
        status = status,
        sparkles = "success"
    );
}
