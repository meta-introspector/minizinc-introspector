# 🧪🏗️ -- MiniZinc Test Suite Makefile -- ⏱️📊
# This Makefile orchestrates the execution of individual MiniZinc model tests.
# It leverages file dependencies to re-run tests only when necessary.

.PHONY: all clean

# Paths relative to tests/minizinc_tests/
MINIZINC_MODELS_DIR := ../../minizinc_models
MINIZINC_DATA_DIR := ../../minizinc_data
TEST_SCRIPT := ../../scripts/run_minizinc_test.sh

# Function to get data files
define get_data_files
$(if $(filter partitioned_manifold.mzn,$(notdir $(1))),$(MINIZINC_DATA_DIR)/partitioned_manifold_minimal.dzn) \
$(if $(filter test_unit_norm.mzn,$(notdir $(1))),$(MINIZINC_DATA_DIR)/partitioned_manifold_minimal.dzn) \
$(if $(filter embedding_sphere_test_master.mzn,$(notdir $(1))),$(MINIZINC_DATA_DIR)/embedding_sphere_test_master_minimal.dzn) 
$(if $(filter embedding_sphere_test_v1.mzn,$(notdir $(1))),$(MINIZINC_DATA_DIR)/embedding_sphere_test_master_minimal.dzn)
endef

# Define a pattern rule for .mzn files
# The target will be the time.log file for each test
%/time.log: $(MINIZINC_MODELS_DIR)/%.mzn
	@echo "Running test for $<"
	@$(TEST_SCRIPT) $< $(call get_data_files,$<)

# Main target to run all specified tests
all: test/time.log \
     test_func/time.log \
     simple_manifold/time.log \
     partitioned_manifold/time.log \
     test_unit_norm/time.log \
     embedding_sphere_test_master/time.log \
     embedding_sphere_test_v1/time.log

clean:
	rm -rf */time.log */stdout.log */stderr.log
