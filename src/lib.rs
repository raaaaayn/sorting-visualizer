//https://github.com/clementmihailescu/Sorting-Visualizer-Tutorial/blob/master/src/SortingVisualizer/SortingVisualizer.jsx
// rust implementation of above merge sort algorithm
mod merge_sort;

use js_sys;
use merge_sort::merge_sort_helper;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// pub fn get_merge_sort_animations(size: usize) -> (Vec<u8>, Vec<u8>, Vec<(usize, usize)>) {
pub fn get_merge_sort_animations(size: usize) -> js_sys::Array {
    let (unsorted, sorted, animations) = merge_sort_helper(size);

    let array_to_be_exported = js_sys::Array::default();

    let unsorted_as_u32: Vec<u32> = unsorted.iter().map(|x| *x as u32).collect();
    let sorted_as_u32: Vec<u32> = sorted.iter().map(|x| *x as u32).collect();
    let animations_as_u32: Vec<[u32; 3]> = animations
        .iter()
        .map(|(a, b, c)| match c {
            true => [*a as u32, *b as u32, 1],
            false => [*a as u32, *b as u32, 0],
        })
        .collect();

    let unsorted_as_js_array = js_sys::Uint32Array::from(&unsorted_as_u32[..]);
    let sorted_as_js_array = js_sys::Uint32Array::from(&sorted_as_u32[..]);
    // let tempp3 = js_sys::Uint32Array::from(&tmp3as32[..]);
    let animations_as_js_array = js_sys::JsString::from(
        serde_json::to_string(&animations_as_u32).expect("Failed to convert to str"),
    );

    array_to_be_exported.push(&unsorted_as_js_array);
    array_to_be_exported.push(&sorted_as_js_array);
    array_to_be_exported.push(&animations_as_js_array);

    array_to_be_exported
}

#[cfg(test)]
mod tests {
    #[test]
    fn replay_animation() {
        todo!()
    }
}
