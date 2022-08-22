use rand::Rng;

pub fn quick_sort_helper(size: usize) -> (Vec<u8>, Vec<u8>, Vec<u32>) {
    let mut rng = rand::thread_rng();

    let mut to_be_sorted: Vec<u8> = (0..size).map(|_| rng.gen()).collect::<Vec<u8>>();

    let unsorted_clone: Vec<u8> = to_be_sorted.clone();

    let mut swap_tracker: Vec<u32> = Vec::new();

    quick_sort(&mut to_be_sorted, 0, (size - 1) as isize, &mut swap_tracker);

    (unsorted_clone, to_be_sorted, swap_tracker)
}

fn quick_sort(list: &mut Vec<u8>, low: isize, high: isize, move_tracker: &mut Vec<u32>) {
    if low < high {
        let pi = partition(list, low, high, move_tracker);

        quick_sort(list, low, pi - 1, move_tracker);
        quick_sort(list, pi + 1, high, move_tracker);
    } else if low == high {
        move_tracker.push(low as u32);
        move_tracker.push(low as u32);
        move_tracker.push(high as u32);
        move_tracker.push(2);
    }
}

fn partition(arr: &mut Vec<u8>, low: isize, high: isize, move_tracker: &mut Vec<u32>) -> isize {
    let (mut i, mut j) = (low as usize, high as usize);

    let pivot = arr[low as usize];
    let pivot_idx = low;

    while i < j {
        loop {
            move_tracker.push(pivot_idx as u32);
            move_tracker.push(i as u32);
            move_tracker.push(j as u32);
            move_tracker.push(0);

            if i == high as usize || arr[i] > pivot {
                break;
            };
            i += 1;
        }
        loop {
            move_tracker.push(pivot_idx as u32);
            move_tracker.push(i as u32);
            move_tracker.push(j as u32);
            move_tracker.push(0);

            if j == low as usize || arr[j] <= pivot {
                break;
            }
            j -= 1;
        }

        if i < j {
            move_tracker.push(pivot_idx as u32);
            move_tracker.push(i as u32);
            move_tracker.push(j as u32);
            move_tracker.push(1);
            let tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        }
    }

    move_tracker.push(pivot_idx as u32);
    move_tracker.push(low as u32);
    move_tracker.push(j as u32);
    move_tracker.push(2);

    let tmp = arr[low as usize];
    arr[low as usize] = arr[j];
    arr[j] = tmp;

    j as isize
}

#[cfg(test)]
mod tests {
    use crate::quick_sort::quick_sort_helper;
    use rand::Rng;

    #[test]
    fn quick_sort_animations() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let size: usize = rng.gen_range(7..999);
            let (mut unsorted, _sorted_by_quick_sort, animations) = quick_sort_helper(size);

            let mut reconstructed = unsorted.clone();

            for slice in animations.chunks(4) {
                let _pivot_idx = &slice[0];
                let i = &slice[1];
                let j = &slice[2];
                let to_exchange = &slice[3];

                if *to_exchange == 1 || *to_exchange == 2 {
                    let temp = reconstructed[*i as usize];
                    reconstructed[*i as usize] = reconstructed[*j as usize];
                    reconstructed[*j as usize] = temp;
                }
            }

            unsorted.sort();
            assert_eq!(unsorted, reconstructed);
        }
    }

    #[test]
    fn quick_sort_implementation() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let size: usize = rng.gen_range(7..999);
            let (mut unsorted, sorted_by_quick_sort, _animations) = quick_sort_helper(size);

            unsorted.sort();

            assert_eq!(unsorted, sorted_by_quick_sort);
        }
    }
}
