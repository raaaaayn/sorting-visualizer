//https://github.com/clementmihailescu/Sorting-Visualizer-Tutorial/blob/master/src/SortingVisualizer/SortingVisualizer.jsx
// rust implementation of above merge sort algorithm

use js_sys;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// pub fn get_merge_sort_animations(size: usize) -> (Vec<u8>, Vec<u8>, Vec<(usize, usize)>) {
pub fn get_merge_sort_animations(size: usize) -> js_sys::Array {
    let mut rng = rand::thread_rng();
    let mut array: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
    let unsorted = array.clone();

    let mut animations: Vec<(usize, usize, Option<usize>)> = Vec::new();
    let arr_len = array.len();
    if arr_len <= 1 {
        todo!()
    };

    let mut auxiliary_array = array.clone();
    merge_sort_helper(
        &mut array,
        0,
        arr_len - 1,
        &mut auxiliary_array,
        &mut animations,
    );
    // let tmp = js_sys::Uint32Array::default();
    let tmp2 = js_sys::Array::default();

    let tmp1as32: Vec<u32> = array.iter().map(|x| *x as u32).collect();
    let tmp2as32: Vec<u32> = unsorted.iter().map(|x| *x as u32).collect();
    let tmp3as32: Vec<Vec<u32>> = animations
        .iter()
        .map(|(a, b, c)| {
            match c {
                Some(c) => vec![*a as u32, *b as u32, *c as u32],
                None => vec![*a as u32, *b as u32],
            }
            // return vec![*a as u32, *b as u32, c.into()];
        })
        // .flatten()
        .collect();

    let tempp1 = js_sys::Uint32Array::from(&tmp1as32[..]);
    let tempp2 = js_sys::Uint32Array::from(&tmp2as32[..]);
    // let tempp3 = js_sys::Uint32Array::from(&tmp3as32[..]);
    let tempp3 = js_sys::JsString::from(serde_json::to_string(&tmp3as32).expect("Failed to convert to str"));

    tmp2.push(&tempp1);
    tmp2.push(&tempp2);
    tmp2.push(&tempp3);

    tmp2
}

fn merge_sort_helper(
    main_array: &mut Vec<u8>,
    start_idx: usize,
    end_idx: usize,
    auxiliary_array: &mut Vec<u8>,
    animations: &mut Vec<(usize, usize, Option<usize>)>,
) {
    if start_idx == end_idx {
        return;
    };
    let middle_idx = (start_idx + end_idx) / 2;
    merge_sort_helper(
        auxiliary_array,
        start_idx,
        middle_idx,
        main_array,
        animations,
    );
    merge_sort_helper(
        auxiliary_array,
        middle_idx + 1,
        end_idx,
        main_array,
        animations,
    );
    do_merge(
        main_array,
        start_idx,
        middle_idx,
        end_idx,
        auxiliary_array,
        animations,
    );
}

fn do_merge(
    main_array: &mut Vec<u8>,
    start_idx: usize,
    middle_idx: usize,
    end_idx: usize,
    auxiliary_array: &mut Vec<u8>,
    animations: &mut Vec<(usize, usize, Option<usize>)>,
) {
    let mut k = start_idx;
    let mut i = start_idx;
    let mut j = middle_idx + 1;
    while i <= middle_idx && j <= end_idx {
        // These are the values that we're comparing; we push them once
        // to change their color.
        animations.push((i, j, None));
        // These are the values that we're comparing; we push them a second
        // time to revert their color.
        animations.push((i, j, None));
        if auxiliary_array[i] <= auxiliary_array[j] {
            // We overwrite the value at index k in the original array with the
            // value at index i in the auxiliary array.
            animations.push((k, auxiliary_array[i] as usize, Some(i)));
            main_array[k] = auxiliary_array[i];
            k += 1;
            i += 1;
        } else {
            // We overwrite the value at index k in the original array with the
            // value at index j in the auxiliary array.
            animations.push((k, auxiliary_array[j] as usize, Some(j)));
            main_array[k] = auxiliary_array[j];

            k += 1;
            j += 1;
        }
    }
    while i <= middle_idx {
        // These are the values that we're comparing; we push them once
        // to change their color.
        animations.push((i, i, None));
        // These are the values that we're comparing; we push them a second
        // time to revert their color.
        animations.push((i, i, None));
        // We overwrite the value at index k in the original array with the
        // value at index i in the auxiliary array.
        animations.push((k, auxiliary_array[i] as usize, Some(i)));
        main_array[k] = auxiliary_array[i];
        k += 1;
        i += 1;
    }
    while j <= end_idx {
        // These are the values that we're comparing; we push them once
        // to change their color.
        animations.push((j, j, None));
        // These are the values that we're comparing; we push them a second
        // time to revert their color.
        animations.push((j, j, None));
        // We overwrite the value at index k in the original array with the
        // value at index j in the auxiliary array.
        animations.push((k, auxiliary_array[j] as usize, Some(j)));
        main_array[k] = auxiliary_array[j];
        k += 1;
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::get_merge_sort_animations;

    #[test]
    fn replay_animation() {
        let (mut unsorted, sorted, _animations) = get_merge_sort_animations(7);
        println!("{:?}\n{:?}", unsorted, sorted);
        unsorted.sort();
        assert_eq!(unsorted, sorted);
    }
}
