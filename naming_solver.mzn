string: literal_value;
string: suggested_name;

string: llm_prompt = "Given the literal value '" ++ literal_value ++ "' and a suggested name '" ++ suggested_name ++ "', please provide a more descriptive and concise name for a Rust constant. Respond only with the new name.";

string: final_constant_name = suggested_name; % Placeholder for now, will be updated by LLM response

output [
  "llm_prompt = \"" ++ llm_prompt ++ \";\n",
  "final_constant_name = \"" ++ final_constant_name ++ \";\n"
];
