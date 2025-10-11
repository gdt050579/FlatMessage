use std::fs;
use std::path::Path;

/// Normalize whitespace to ignore minor formatting differences
fn normalize_whitespace(content: &str) -> String {
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate a simple diff between two strings
fn generate_diff(left: &str, right: &str) -> String {
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();

    let mut diff = String::new();
    let max_lines = left_lines.len().max(right_lines.len());

    for i in 0..max_lines {
        let left_line = left_lines.get(i).unwrap_or(&"");
        let right_line = right_lines.get(i).unwrap_or(&"");

        if left_line != right_line {
            diff.push_str(&format!("Line {}:\n", i + 1));
            diff.push_str(&format!("  - flat_message:     {}\n", left_line));
            diff.push_str(&format!("  - flat_message_proc_macro: {}\n", right_line));
            diff.push('\n');
        }
    }

    diff
}

/// Test that ensures the common code files are identical between flat_message and flat_message_proc_macro crates.
/// This prevents the duplicated code from getting out of sync.
#[test]
fn test_common_files_are_identical() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    // Define the common files to compare
    let common_files = ["constants.rs", "data_format.rs", "hashes.rs"];

    for file_name in &common_files {
        let flat_message_path = workspace_root
            .join("flat_message")
            .join("src")
            .join("common")
            .join(file_name);

        let proc_macro_path = workspace_root
            .join("flat_message_proc_macro")
            .join("src")
            .join("common")
            .join(file_name);

        // Read both files
        let flat_message_content = fs::read_to_string(&flat_message_path).unwrap_or_else(|_| {
            panic!(
                "Failed to read flat_message common file: {}",
                flat_message_path.display()
            )
        });

        let proc_macro_content = fs::read_to_string(&proc_macro_path).unwrap_or_else(|_| {
            panic!(
                "Failed to read flat_message_proc_macro common file: {}",
                proc_macro_path.display()
            )
        });

        // Compare the content (ignoring whitespace differences)
        let flat_message_normalized = normalize_whitespace(&flat_message_content);
        let proc_macro_normalized = normalize_whitespace(&proc_macro_content);

        if flat_message_normalized != proc_macro_normalized {
            // Generate a helpful diff message
            let diff = generate_diff(&flat_message_normalized, &proc_macro_normalized);

            panic!(
                    "Common file '{}' differs between flat_message and flat_message_proc_macro crates.\n\
                    This indicates the duplicated common code is out of sync.\n\
                    Please update both files to match.\n\
                    \n\
                    flat_message path: {}\n\
                    proc_macro path: {}\n\
                    \n\
                    DIFF:\n\
                    {}",
                    file_name,
                    flat_message_path.display(),
                    proc_macro_path.display(),
                    diff
                );
        }
    }
}
