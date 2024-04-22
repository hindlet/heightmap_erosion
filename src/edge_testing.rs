

#[cfg(test)]
mod edge_check_function_tests {
    #[test]
    fn three_by_three_tests() {
        let width = 3;
        let height = 3;

        // 0, 1, 2
        // 3, 4, 5
        // 6, 7, 8

        assert_eq!(check_edges(0, width, height), (true, false, true, false));
        assert_eq!(check_edges(1, width, height), (false, false, true, false));
        assert_eq!(check_edges(2, width, height), (false, true, true, false));

        assert_eq!(check_edges(3, width, height), (true, false, false, false));
        assert_eq!(check_edges(4, width, height), (false, false, false, false));
        assert_eq!(check_edges(5, width, height), (false, true, false, false));

        assert_eq!(check_edges(6, width, height), (true, false, false, true));
        assert_eq!(check_edges(7, width, height), (false, false, false, true));
        assert_eq!(check_edges(8, width, height), (false, true, false, true));
    }

    #[test]
    fn four_by_four_tests() {
        let width = 4;
        let height = 4;

        // 0, 1, 2, 3
        // 4, 5, 6, 7
        // 8, 9, 10, 11
        // 12, 13, 14, 15

        assert_eq!(check_edges(0, width, height), (true, false, true, false));
        assert_eq!(check_edges(1, width, height), (false, false, true, false));
        assert_eq!(check_edges(2, width, height), (false, false, true, false));
        assert_eq!(check_edges(3, width, height), (false, true, true, false));

        assert_eq!(check_edges(4, width, height), (true, false, false, false));
        assert_eq!(check_edges(5, width, height), (false, false, false, false));
        assert_eq!(check_edges(6, width, height), (false, false, false, false));
        assert_eq!(check_edges(7, width, height), (false, true, false, false));

        assert_eq!(check_edges(8, width, height), (true, false, false, false));
        assert_eq!(check_edges(9, width, height), (false, false, false, false));
        assert_eq!(check_edges(10, width, height), (false, false, false, false));
        assert_eq!(check_edges(11, width, height), (false, true, false, false));

        assert_eq!(check_edges(12, width, height), (true, false, false, true));
        assert_eq!(check_edges(13, width, height), (false, false, false, true));
        assert_eq!(check_edges(14, width, height), (false, false, false, true));
        assert_eq!(check_edges(15, width, height), (false, true, false, true));
    }
}

fn check_edges(index: i32, width: i32, height: i32) -> (bool, bool, bool, bool) {
    (
        index % width == 0,
        (index + 1) % width == 0,
        index / width == 0,
        (index + width) / width == height
    )
}