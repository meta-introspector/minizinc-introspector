use std::ffi::CStr;
use crate::types::{MiniZincModel, MiniZincItem, MiniZincSolveItem, MiniZincOutputItem};
use crate::ffi_bindings::{minizinc_model_free, model_get_filename, model_get_filepath, model_get_num_items, model_get_item_at_index, minizinc_model_get_doc_comment, minizinc_model_get_parent, minizinc_model_get_solve_item, minizinc_model_get_output_item};

impl Drop for MiniZincModel {
    fn drop(&mut self) {
        unsafe { minizinc_model_free(self.0) };
    }
}

impl MiniZincModel {
    pub fn filename(&self) -> String {
        let c_str = unsafe { model_get_filename(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn filepath(&self) -> String {
        let c_str = unsafe { model_get_filepath(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn num_items(&self) -> u32 {
        unsafe { model_get_num_items(self.0) }
    }

    pub fn get_item_at_index(&self, index: u32) -> Option<MiniZincItem> {
        let item_ptr = unsafe { model_get_item_at_index(self.0, index) };
        if item_ptr.is_null() {
            None
        } else {
            Some(MiniZincItem(item_ptr))
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
