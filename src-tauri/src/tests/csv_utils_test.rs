use fileflow::fileflow::utils::csv_utils::{find_separator, read_first_line};
use crate::tests::utils_tests::{generate_csv_file, remove_csv_file};

#[tokio::test]
async fn test_detect_separator() {
    let test_cases = [
        ("header1,header2", Some(',')),
        ("header1;header2", Some(';')),
        ("header1\theader2", Some('\t')),
        ("header1|header2", Some('|')),
        ("header1 header2", Some(' ')),
        ("header1\0header2", Some('\0')),
        ("header1header2", None),
    ];

    for (input, expected) in test_cases {
        match expected {
            Some(separator) => assert_eq!(find_separator(input).unwrap(), separator),
            None => assert!(find_separator(input).is_err()),
        }
    }
}

#[tokio::test]
async fn test_read_first_line() {
    let csv_file_path: String =
        generate_csv_file("test_read_first_line").expect("Failed to generate csv file");
    let first_line: String = read_first_line(&csv_file_path).expect("Failed to read first line");
    assert_eq!(first_line, "header1,header2");
    remove_csv_file("test_read_first_line").expect("Failed to remove csv file");
}
