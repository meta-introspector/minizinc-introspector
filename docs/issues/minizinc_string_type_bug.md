# Critical Issue: MiniZinc `string` Type Parsing Error in Record Definitions

## Description
The MiniZinc compiler/parser is failing to correctly recognize the `string` type when used within record definitions. This issue manifests as a `syntax error, unexpected string, expecting identifier` even when the MiniZinc code adheres to the documented syntax for record field declarations.

## Impact
This bug is currently blocking further progress on the QA and compilation of MiniZinc models that utilize records with string fields, including core models like `gemini_self_model.mzn`.

## Error Message
```
Error: syntax error, unexpected string, expecting identifier
```

## Minimal Reproducible Example (`temp_string_test.mzn`)
```minizinc
type TestRecord = record(
    my_string: string
);

var TestRecord: my_instance;

solve satisfy;
```

## Steps to Reproduce
1. Save the above content as `temp_string_test.mzn`.
2. Compile using the MiniZinc executable:
   `/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc -c --solver Gecode /data/data/com.termux/files/home/storage/github/libminizinc/temp_string_test.mzn`

## Expected Outcome
The model should compile successfully without syntax errors.

## Actual Outcome
The compilation fails with the `syntax error, unexpected string, expecting identifier` error.

## Proposed Investigation Steps
1. Verify the MiniZinc version (`minizinc --version`).
2. Attempt to rebuild MiniZinc from source to ensure a clean installation.
3. Consult MiniZinc documentation and community forums for known issues or workarounds related to `string` type parsing in record definitions.
4. Consider if there are any environment-specific factors (e.g., Android/Termux environment) that might be contributing to this parsing issue.

---
*This issue documentation is part of the "QA MiniZinc Models" Change Request and highlights a critical blocking problem.*
