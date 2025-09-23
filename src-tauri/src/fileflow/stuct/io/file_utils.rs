use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read};
use std::path::Path;

/// File separator detection and first line reading utilities
pub struct FileUtils;

impl FileUtils {
    /// Common separators ordered by likelihood of occurrence
    const SEPARATORS: [char; 6] = [',', ';', '\t', '|', ' ', '\0'];

    /// Maximum bytes to read for separator detection (optimization)
    const MAX_DETECTION_BYTES: usize = 4096;

    /// Detects the most likely separator character from a text line
    pub fn find_separator(line: &str) -> Result<char, String> {
        Self::validate_input(line)?;

        // Find the first separator that appears in the line
        Self::SEPARATORS
            .iter()
            .find(|&&sep| line.contains(sep))
            .copied()
            .ok_or_else(|| "Could not detect a valid separator".to_string())
    }

    /// Reads the first line of a file efficiently
    pub fn read_first_line<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
        let file = Self::open_file(file_path)?;
        let mut reader = BufReader::new(file);

        Self::extract_first_line(&mut reader)
    }

    /// Detects separator from the first line of a file
    pub fn detect_separator_from_file<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<char, Box<dyn std::error::Error>> {
        let first_line = Self::read_first_line(file_path)?;
        Self::find_separator(&first_line).map_err(|e| e.into())
    }

    /// Reads first line with separator detection in one operation
    pub fn read_first_line_with_separator<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<(String, char), Box<dyn std::error::Error>> {
        let first_line = Self::read_first_line(file_path)?;
        let separator = Self::find_separator(&first_line)?;
        Ok((first_line, separator))
    }

    /// Validates that the input line is not empty or just whitespace
    fn validate_input(line: &str) -> Result<(), String> {
        if line.trim().is_empty() {
            return Err("Input line cannot be empty".to_string());
        }
        Ok(())
    }

    /// Opens a file with error handling
    fn open_file<P: AsRef<Path>>(file_path: P) -> io::Result<File> {
        let path = file_path.as_ref();

        if !path.exists() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            ));
        }

        if !path.is_file() {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Path is not a file: {}", path.display()),
            ));
        }

        File::open(path)
    }

    /// Extracts the first line from a buffered reader
    fn extract_first_line(reader: &mut BufReader<File>) -> io::Result<String> {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => Err(io::Error::new(ErrorKind::NotFound, "File is empty")),
            Ok(_) => {
                // Remove trailing newline characters
                Self::clean_line_ending(&mut line);
                Ok(line)
            }
            Err(e) => Err(e),
        }
    }

    /// Cleans trailing newline characters from a line
    fn clean_line_ending(line: &mut String) {
        // Remove trailing \r\n or \n
        while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }
    }

    /// Alternative implementation using byte-by-byte reading (for comparison)
    pub fn read_first_line_bytes<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
        let file = Self::open_file(file_path)?;
        let mut reader = BufReader::new(file);

        let mut line_bytes = Vec::with_capacity(256); // Pre-allocate reasonable capacity
        let mut byte = [0u8; 1];
        let mut bytes_read = 0;

        while bytes_read < Self::MAX_DETECTION_BYTES {
            match reader.read_exact(&mut byte) {
                Ok(()) => {
                    bytes_read += 1;
                    match byte[0] {
                        b'\n' => break,    // End of line found
                        b'\r' => continue, // Skip carriage return
                        b => line_bytes.push(b),
                    }
                }
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    if line_bytes.is_empty() {
                        return Err(io::Error::new(ErrorKind::NotFound, "File is empty"));
                    }
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        // Convert to string with proper error handling
        String::from_utf8(line_bytes)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, format!("Invalid UTF-8: {}", e)))
    }

    /// Advanced separator detection with confidence scoring
    pub fn detect_separator_with_confidence(line: &str) -> Result<(char, f32), String> {
        Self::validate_input(line)?;

        let mut best_separator = None;
        let mut best_score = 0.0;

        for &separator in &Self::SEPARATORS {
            let score = Self::calculate_separator_score(line, separator);
            if score > best_score {
                best_score = score;
                best_separator = Some(separator);
            }
        }

        best_separator
            .map(|sep| (sep, best_score))
            .ok_or_else(|| "Could not detect a valid separator".to_string())
    }

    /// Calculates a score for how well a separator fits the line
    /// Higher scores indicate a better fit
    fn calculate_separator_score(line: &str, separator: char) -> f32 {
        if !line.contains(separator) {
            return 0.0;
        }

        let count = line.matches(separator).count() as f32;
        let length = line.len() as f32;

        // Normalize by line length and apply separator preference
        let base_score = count / length;
        let preference_multiplier = match separator {
            ',' => 1.0,  // Highest preference
            ';' => 0.9,  // High preference
            '\t' => 0.8, // Medium preference
            '|' => 0.7,  // Lower preference
            ' ' => 0.3,  // Low preference (spaces are often part of data)
            _ => 0.1,    // Lowest preference
        };

        base_score * preference_multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        file
    }

    #[tokio::test]
    async fn test_find_separator() {
        assert_eq!(FileUtils::find_separator("a,b,c").unwrap(), ',');
        assert_eq!(FileUtils::find_separator("a;b;c").unwrap(), ';');
        assert_eq!(FileUtils::find_separator("a\tb\tc").unwrap(), '\t');
        assert_eq!(FileUtils::find_separator("a|b|c").unwrap(), '|');
        assert_eq!(FileUtils::find_separator("a b c").unwrap(), ' ');
        assert_eq!(FileUtils::find_separator("a\0b\0c").unwrap(), '\0');

        // Test error case
        assert!(FileUtils::find_separator("abc").is_err());
        assert!(FileUtils::find_separator("").is_err());
        assert!(FileUtils::find_separator("   ").is_err());
    }

    #[tokio::test]
    async fn test_read_first_line() {
        let file = create_test_file("first line\nsecond line\nthird line");
        let result = FileUtils::read_first_line(file.path()).unwrap();
        assert_eq!(result, "first line");
    }

    #[tokio::test]
    async fn test_read_first_line_with_carriage_return() {
        let file = create_test_file("first line\r\nsecond line");
        let result = FileUtils::read_first_line(file.path()).unwrap();
        assert_eq!(result, "first line");
    }

    #[tokio::test]
    async fn test_read_first_line_single_line() {
        let file = create_test_file("only line");
        let result = FileUtils::read_first_line(file.path()).unwrap();
        assert_eq!(result, "only line");
    }

    #[tokio::test]
    async fn test_read_first_line_empty_file() {
        let file = create_test_file("");
        let result = FileUtils::read_first_line(file.path());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_detect_separator_from_file() {
        let file = create_test_file("name,age,city\nJohn,25,NYC");
        let separator = FileUtils::detect_separator_from_file(file.path()).unwrap();
        assert_eq!(separator, ',');
    }

    #[tokio::test]
    async fn test_read_first_line_with_separator() {
        let file = create_test_file("name;age;city\nJohn;25;NYC");
        let (line, separator) = FileUtils::read_first_line_with_separator(file.path()).unwrap();
        assert_eq!(line, "name;age;city");
        assert_eq!(separator, ';');
    }

    #[tokio::test]
    async fn test_detect_separator_with_confidence() {
        let (sep, confidence) = FileUtils::detect_separator_with_confidence("a,b,c,d").unwrap();
        assert_eq!(sep, ',');
        assert!(confidence > 0.0);

        // Test that comma has higher confidence than space for mixed content
        let (sep1, conf1) = FileUtils::detect_separator_with_confidence("a, b, c, d").unwrap();
        let (sep2, conf2) = FileUtils::detect_separator_with_confidence("a b c d").unwrap();

        if sep1 == ',' && sep2 == ' ' {
            assert!(conf1 > conf2);
        }
    }
}
