pub fn linear_search_list(haystack: &Vec<i32>, needle: i32) -> bool {
    for i in 0..haystack.len() {
        if haystack[i] == needle {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::linear_search::linear_search_list;

    #[test]
    fn test_linear_search() {
        // Arrange
        let foo = vec![1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

        // Act
        let ls_true_1 = linear_search_list(&foo, 69);
        let ls_false_1 = linear_search_list(&foo, 1336);
        let ls_true_2 = linear_search_list(&foo, 69420);
        let ls_false_2 = linear_search_list(&foo, 69421);
        let ls_true_3 = linear_search_list(&foo, 1);
        let ls_false_3 = linear_search_list(&foo, 0);

        // Assert
        assert_eq!(ls_true_1, true);
        assert_eq!(ls_true_2, true);
        assert_eq!(ls_true_3, true);
        assert_eq!(ls_false_1, false);
        assert_eq!(ls_false_2, false);
        assert_eq!(ls_false_3, false);
    }
}
