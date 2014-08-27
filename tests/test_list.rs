extern crate adt;

#[cfg(test)]
mod test {
    use adt::{
        DList,
        List
    };

    #[test]
    fn test_new_dlist() {
        // Initialize the test list.
        let list: DList<int> = DList::new();

        // Verify that the list is empty.
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_prepend_dlist() {
        // Initialize the test list.
        let mut list: DList<int> = DList::new();

        list.prepend(10);
        list.prepend(20);
        list.prepend(11);

        // Verify that the list contains 3 elements.
        assert!(!list.is_empty());
        assert_eq!(list.size(), 3);

        assert_eq!(list.head(), Some(11));
        assert_eq!(list.tail(), Some(10));
    }

    #[test]
    fn test_append_dlist() {
        // Initialize the test list.
        let mut list: DList<int> = DList::new();

        list.append(10);
        list.append(20);
        list.append(11);

        // Verify that the list contains 3 elements.
        assert!(!list.is_empty());
        assert_eq!(list.size(), 3);
    }
}
