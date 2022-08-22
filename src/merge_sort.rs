use rand::Rng;

pub fn merge_sort_helper(size: usize) -> (Vec<u8>, Vec<u8>, Vec<u32>) {
    let mut rng = rand::thread_rng();

    let unsorted: Vec<u8> = (0..size).map(|_| rng.gen()).collect::<Vec<u8>>();

    let unsorted_with_index: Vec<(u8, usize)> =
        unsorted.iter().enumerate().map(|(i, v)| (*v, i)).collect();

    let mut swap_tracker: Vec<u32> = Vec::new();

    let sorted: Vec<u8> = merge_sort(
        &unsorted_with_index.to_vec(),
        0,
        size - 1,
        &mut swap_tracker,
    )
    .iter()
    .map(|(v, _)| *v)
    .collect();

    (unsorted, sorted, swap_tracker)
}

fn merge_sort(
    list: &Vec<(u8, usize)>,
    bottom_index: usize,
    top_index: usize,
    swap_tracker: &mut Vec<u32>,
) -> Vec<(u8, usize)> {
    let (low, size) = (0, list.len());

    if size > 1 {
        let mid = (low + size) / 2;

        let mid_index = (bottom_index + top_index + 1) / 2;

        let mut fist_half = merge_sort(
            &list[0..mid].to_vec(),
            bottom_index,
            mid_index - 1,
            swap_tracker,
        );
        let mut second_half = merge_sort(&list[mid..].to_vec(), mid_index, top_index, swap_tracker);

        merge(&mut fist_half, &mut second_half, swap_tracker)
    } else {
        list.to_vec()
    }
}

fn merge(
    left: &mut Vec<(u8, usize)>,
    right: &mut Vec<(u8, usize)>,
    move_tracker: &mut Vec<u32>,
) -> Vec<(u8, usize)> {
    // let mut list_c: Vec<u8> = Vec::default();
    let mut merged: Vec<(u8, usize)> = Vec::new();

    let size_left = left.len();
    let size_right = right.len();
    let (mut i, mut j) = (0, 0);

    while i < size_left && j < size_right {
        if left[i].0 < right[j].0 {
            merged.push(left[i]);
            move_tracker.push(left[i].1 as u32);
            move_tracker.push(right[j].1 as u32);
            move_tracker.push(0);
            i += 1;
        } else {
            move_tracker.push(left[i].1 as u32);
            move_tracker.push(right[j].1 as u32);
            move_tracker.push(1);

            // let tmp = b.1;
            right[j].1 = left[i].1;
            for (local_idx, (_, actual_idx)) in left.iter_mut().enumerate() {
                if local_idx > i {
                    *actual_idx += 1;
                }
            }
            left[i].1 += 1;

            merged.push(right[j]);
            j += 1;
        }
    }

    while i < size_left {
        left[i].1 = merged.last().unwrap().1 + 1;
        merged.push(left[i]);
        i += 1;
    }
    while j < size_right {
        right[j].1 = merged.last().unwrap().1 + 1;
        merged.push(right[j]);
        j += 1;
    }

    merged
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::merge_sort::merge_sort_helper;
    use rand::Rng;

    #[test]
    fn test_merge_sort_animations() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let size: usize = rng.gen_range(7..999);
            let (unsorted, sorted, animations) = merge_sort_helper(size);
            let mut tmp = unsorted.clone();
            for slice in animations.chunks(3) {
                let i = &slice[0];
                let j = &slice[1];
                let b = &slice[2];
                if *b == 1 && i != j {
                    let temp = tmp[*j as usize];
                    tmp.remove(*j as usize);
                    tmp.insert(*i as usize, temp);
                }
            }

            assert_eq!(tmp, sorted);
        }
    }
}
