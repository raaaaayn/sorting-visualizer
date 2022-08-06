mod merge_sort;

// use js_sys;
use merge_sort::merge_sort_helper;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sort {
    array: Vec<u32>,
    animations: Vec<u32>,
}

#[wasm_bindgen]
impl Sort {
    pub fn new(size: Option<usize>) -> Sort {
        let size = size.unwrap_or(40);
        let (unsorted, _sorted, animations) = merge_sort_helper(size);
        let unsorted: Vec<u32> = unsorted.iter().map(|x| *x as u32).collect();

        Sort {
            array: unsorted,
            animations,
        }
    }

    pub fn animations_length(&self) -> u32 {
        self.animations.len() as u32
    }

    pub fn get_unsorted(&self) -> *const u32 {
        self.array.as_slice().as_ptr()
    }

    #[wasm_bindgen]
    pub fn get_merge_sort_animations(&self) -> *const u32 {
        self.animations.as_slice().as_ptr()
    }
}
