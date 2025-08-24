#!/bin/bash

MINIZINC_EXE="/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc"
LOG_DIR="/data/data/com.termux/files/home/storage/github/libminizinc/logs/mzn_tests"
mkdir -p "$LOG_DIR"

# Find all .mzn files, excluding standard library and documentation examples
find /data/data/com.termux/files/home/storage/github/libminizinc -name "*.mzn" \
  -not -path "*/share/minizinc/std/*" \
  -not -path "*/share/minizinc/geas/*" \
  -not -path "*/share/minizinc/gecode_presolver/*" \
  -not -path "*/share/minizinc/linear/*" \
  -not -path "*/docs/*" \
  -print0 | while IFS= read -r -d $'\0' mzn_file; do
    echo "Testing: $mzn_file"
    
    # Determine corresponding .dzn file
    base_name=$(basename "$mzn_file" .mzn)
    dir_name=$(dirname "$mzn_file")
    dzn_file="$dir_name/$base_name.dzn"
    
    output_log="$LOG_DIR/$(basename "$mzn_file" .mzn)_output.log"
    error_log="$LOG_DIR/$(basename "$mzn_file" .mzn)_error.log"
    
    if [ -f "$dzn_file" ]; then
        "$MINIZINC_EXE" "$mzn_file" "$dzn_file" > "$output_log" 2> "$error_log"
    else
        "$MINIZINC_EXE" "$mzn_file" > "$output_log" 2> "$error_log"
    fi
    
    if [ $? -eq 0 ]; then
        echo "  SUCCESS"
        rm "$error_log" # Remove empty error logs
    else
        echo "  FAILURE - Check $error_log for details"
        cat "$error_log"
    fi
    echo ""
done

echo "MiniZinc testing complete. Check $LOG_DIR for logs."
