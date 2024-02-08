pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let len = arr.len();
    let pivot_idx = partition(arr, 0, len - 1);
    quick_sort(&mut arr[0..pivot_idx]);
    quick_sort(&mut arr[pivot_idx + 1..len]);
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut idx = low as isize - 1;

    for i in low..high {
        if arr[i] <= pivot {
            idx += 1;
            arr.swap(i, idx as usize);
        }
    }

    idx += 1;
    arr.swap(high, idx as usize);
    idx as usize
}

#[cfg(test)]
mod tests {
    use crate::quicksort::quick_sort;
    #[test]
    fn test_quick_sort() {
        //Arrange
        let mut arr = vec![9, 3, 7, 4, 69, 420, 42];
        //Act
        quick_sort(&mut arr);
        //Assert
        assert_eq!(arr, vec![3, 4, 7, 9, 42, 69, 420]);
    }
}
