use std::ffi::{CStr, CString};
use crate::types::{MiniZincModel, MiniZincItem, MiniZincSolveItem, MiniZincOutputItem};
use crate::ffi_bindings::{minizinc_model_free, model_get_filename, model_get_filepath, minizinc_string_free, model_get_num_items, model_get_item_at_index, minizinc_model_get_doc_comment, minizinc_model_get_parent, minizinc_model_get_solve_item, minizinc_model_get_output_item};

/// A newtype wrapper for C-allocated strings (`char*`) that ensures proper memory deallocation
/// when the Rust `MiniZincString` goes out of scope. This is crucial for FFI safety.
pub struct MiniZincString(*mut CString);

impl Drop for MiniZincString {
    /// The `drop` implementation for `MiniZincString`. This is automatically called when
    /// the `MiniZincString` instance goes out of scope. It calls the C FFI function
    /// `minizinc_string_free` to deallocate the memory owned by the C-string.
    fn drop(&mut self) {
        // SAFETY: We are assuming that `self.0` is a valid, non-null pointer to a C-string
        // allocated by the MiniZinc C++ library, and that `minizinc_string_free` is the
        // correct function to deallocate it. This is a critical assumption for FFI safety.
        unsafe { minizinc_string_free((*self.0).as_ptr() as *mut std::os::raw::c_char) };
    }
}

impl MiniZincModel {
    /// Retrieves the filename associated with the MiniZinc model.
    /// This calls the C FFI function `model_get_filename`.
    pub fn filename(&self) -> String {
        // SAFETY: We are calling a C FFI function that returns a `const char*`.
        // We assume this pointer is valid and points to a null-terminated C-string.
        // The memory for this string is owned by the C++ side and is not managed by Rust.
        let c_str = unsafe { model_get_filename(self.0) };
        // Convert the C-string to a Rust `String`. `CStr::from_ptr` creates a safe
        // Rust slice from the C-string, and `to_str().unwrap().to_string()` converts
        // it to an owned Rust `String`. This involves copying the data.
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    /// Retrieves the filepath associated with the MiniZinc model.
    /// This calls the C FFI function `model_get_filepath`, which returns a C-allocated string.
    /// The returned string is immediately converted to a Rust `String`, and the C-allocated
    /// memory is freed using `minizinc_string_free`.
    ///
    /// This approach ensures that Rust owns the `String` data, and the C-allocated memory
    /// is promptly released, preventing memory leaks.
    pub fn filepath(&self) -> String {
        // SAFETY: We are calling a C FFI function that returns a `char*`.
        // We assume this pointer is valid and points to a null-terminated C-string
        // allocated on the C++ heap. This memory is now owned by Rust and must be freed.
        let c_char_ptr = unsafe { model_get_filepath(self.0) };
        // Convert the C-string to a Rust `String`. This involves copying the data.
        // `CStr::from_ptr` creates a safe Rust slice from the C-string.
        let rust_string = unsafe { CStr::from_ptr(c_char_ptr).to_str().unwrap().to_string() };
        // SAFETY: We are calling a C FFI function to free the memory pointed to by `c_char_ptr`.
        // This is crucial to prevent memory leaks, as the C++ side allocated this string.
        unsafe { minizinc_string_free(c_char_ptr) }; // Free the C-allocated string immediately
        rust_string
    }

    pub fn num_items(&self) -> u32 {
        unsafe { model_get_num_items(self.0) }
    }

    pub fn get_item_at_index(&self, index: u32) -> Option<MiniZincItem> {
        let item_ptr = unsafe { model_get_item_at_index(self.0, index) };
        if item_ptr.is_null() {
            None
        } else {
            Some(MiniZincItem(item_ptr as *mut std::os::raw::c_void))
        }
    }

    pub fn doc_comment(&self) -> String {
        let c_str = unsafe { minizinc_model_get_doc_comment(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn parent(&self) -> Option<MiniZincModel> {
        let parent_ptr = unsafe { minizinc_model_get_parent(self.0) };
        if parent_ptr.is_null() {
            None
        } else {
            Some(MiniZincModel(parent_ptr))
        }
    }

    pub fn solve_item(&self) -> Option<MiniZincSolveItem> {
        let solve_item_ptr = unsafe { minizinc_model_get_solve_item(self.0) };
        if solve_item_ptr.is_null() {
            None
        } else {
            Some(MiniZincSolveItem(solve_item_ptr))
        }
    }

    pub fn output_item(&self) -> Option<MiniZincOutputItem> {
        let output_item_ptr = unsafe { minizinc_model_get_output_item(self.0) };
        if output_item_ptr.is_null() {
            None
        } else {
            Some(MiniZincOutputItem(output_item_ptr))
        }
    }
}
