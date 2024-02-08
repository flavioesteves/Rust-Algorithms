pub fn bs_list(haystack: &Vec<i32>, needle: i32) -> bool {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        println!("In the loop Start");
        let mid_point = (low + (high - low) / 2) as usize;
        let value = haystack[mid_point];

        if value == needle {
            return true;
        } else if value > needle {
            high = mid_point;
        } else {
            low = mid_point + 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::linear_search::bs_list;
    #[test]
    fn test_binary_search_array() {
        // Arrange
        let foo = vec![1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

        // Act
        let bs_true_1 = bs_list(&foo, 69);
        let bs_false_1 = bs_list(&foo, 1336);
        let bs_true_2 = bs_list(&foo, 69420);
        let bs_false_2 = bs_list(&foo, 69421);
        let bs_true_3 = bs_list(&foo, 1);
        let bs_false_3 = bs_list(&foo, 0);

        // Assert
        assert_eq!(bs_true_1, true);
        assert_eq!(bs_true_2, true);
        assert_eq!(bs_true_3, true);
        assert_eq!(bs_false_1, false);
        assert_eq!(bs_false_2, false);
        assert_eq!(bs_false_3, false);
    }
}
