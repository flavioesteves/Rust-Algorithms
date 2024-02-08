pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::bubble_sort;

    #[test]
    fn test_buble_sort() {
        // Arrange
        let mut arr = vec![9, 3, 7, 4, 69, 420, 42];
        // Act
        bubble_sort(&mut arr);
        // Assert
        assert_eq!(arr, vec![3, 4, 7, 9, 42, 69, 420]);
    }
}
