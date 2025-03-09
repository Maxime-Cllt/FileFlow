use crate::fileflow::utils::fileflowlib::{escaped_values, find_separator, read_first_line};
use crate::tests::utils::{generate_csv_file, remove_csv_file};
use csv::StringRecord;

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
async fn test_escape_values() {
    let record: StringRecord = StringRecord::from(vec!["value1", "value2"]);
    let values: String = escaped_values(record);
    assert_eq!(values, "'value1', 'value2'");

    let record: StringRecord = StringRecord::from(vec![
        "INSERT INTO test_table VALUES (1,2);",
        "UPDATE test_table SET column1 = 1;",
        "DELETE FROM test_table WHERE column1 = 1;",
        "SELECT * FROM test_table;",
    ]);
    let values: String = escaped_values(record);
    assert_eq!(values, "'INSERT INTO test_table VALUES (1,2);', 'UPDATE test_table SET column1 = 1;', 'DELETE FROM test_table WHERE column1 = 1;', 'SELECT * FROM test_table;'");
}

#[tokio::test]
async fn test_read_first_line() {
    let csv_file_path: String = generate_csv_file("test_read_first_line").expect("Failed to generate csv file");
    let first_line: String = read_first_line(&csv_file_path).expect("Failed to read first line");
    assert_eq!(first_line, "header1,header2");
    let _ = remove_csv_file("test_read_first_line");
}
