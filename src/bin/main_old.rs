use rand::{self, Rng};

fn main() {
    merge_sort_helper(5);
}

fn merge_sort_helper(size: usize) -> (Vec<u8>, Vec<u8>, Vec<(usize, usize, bool)>) {
    // let unsorted: Vec<u8> = rand::random();
    let mut rng = rand::thread_rng();

    let unsorted: Vec<u8> = (0..size).map(|_| rng.gen()).collect::<Vec<u8>>();

    println!(" {:?}\n", unsorted);
    let unsorted_with_index: Vec<(u8, usize)> =
        unsorted.iter().enumerate().map(|(i, v)| (*v, i)).collect();

    let mut swap_tracker: Vec<(usize, usize, bool)> = Vec::new();

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
    swap_tracker: &mut Vec<(usize, usize, bool)>,
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

        // println!("{} {} {}", bottom_index, mid_index, top_index);
        merge(&mut fist_half, &mut second_half, swap_tracker)
    } else {
        list.to_vec()
    }
}

fn merge(
    left: &mut Vec<(u8, usize)>,
    right: &mut Vec<(u8, usize)>,
    move_tracker: &mut Vec<(usize, usize, bool)>,
) -> Vec<(u8, usize)> {
    // let mut list_c: Vec<u8> = Vec::default();
    let mut merged: Vec<(u8, usize)> = Vec::new();

    let size_left = left.len();
    let size_right = right.len();
    let (mut i, mut j) = (0, 0);

    // println!(
    //     "left_starting_index {} , right_starting_index {}",
    //     left_starting_index, right_starting_index
    // );
    // println!("{:?} ⤋ {:?}", left, right);
    // println!(
    //     "({}) ({})",
    //     left_starting_index + i,
    //     right_starting_index + j
    // );

    while i < size_left && j < size_right {
        if left[i].0 < right[j].0 {
            println!("{:?} {:?}", left[i], right[j]);
            merged.push(left[i]);
            println!(" {:?}\n", merged);
            move_tracker.push((left[i].1, right[j].1, false));
            println!("{:?} {:?}", left[i], right[j]);
            println!("({} {} {})", left[i].1, right[j].1, false);
            println!("{:?}", move_tracker);
            i += 1;
        } else {
            println!("{:?} {:?}", left[i], right[j]);

            move_tracker.push((left[i].1, right[j].1, true));
            // let tmp = b.1;
            right[j].1 = left[i].1;
            for (local_idx, (_, actual_idx)) in left.iter_mut().enumerate() {
                if local_idx > i {
                    *actual_idx += 1;
                }
            }
            left[i].1 += 1;

            merged.push(right[j]);
            println!(" {:?}\n", merged);
            println!("{:?} {:?}", left[i], right[j]);
            println!("({} {} {})", left[i].1, right[j].1, true);
            println!("{:?}", move_tracker);
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
    println!(" {:?}\n", merged);

    merged
}

#[cfg(test)]
mod tests {
    use crate::merge_sort_helper;

    #[test]
    fn replay_animation() {
        let (unsorted, sorted, animations) = merge_sort_helper(7);
        println!("\n{:?}\n", animations);
        let mut tmp = unsorted.clone();
        // println!("{:?}\n{:?}\n{:?}", unsorted, sorted, tmp);
        for (i, j, b) in animations.iter() {
            let i = *i as usize;
            let j = *j as usize;
            println!("\n{:?}", tmp);
            println!("{} {} {}", i, j, b);
            // tmp[i] = unsorted[j];
            if *b && i != j {
                let temp = tmp[j];
                println!("{} {} {}", i, j, temp);
                tmp.remove(j);
                tmp.insert(i, temp);
                println!("{:?}", tmp);
            }
        }
        println!(
            "\nunsorted {:?}\nsorted   {:?}\nreconstr {:?}\n",
            unsorted, sorted, tmp
        );
        assert_eq!(tmp, sorted);
    }
}
