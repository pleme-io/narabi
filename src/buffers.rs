//! Buffer information collection.
//!
//! Gathers metadata about listed Neovim buffers: filename, modified state,
//! buffer number, and whether the buffer is currently active.

use std::path::Path;

/// Metadata for a single buffer shown in the tabline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferInfo {
    /// The Neovim buffer handle (1-indexed).
    pub handle: i32,
    /// Short display name (filename without the directory path).
    pub name: String,
    /// Full path, if available.
    pub full_path: String,
    /// Whether the buffer has unsaved changes.
    pub modified: bool,
    /// Whether this buffer is the currently active one.
    pub active: bool,
}

/// Extract the short display name from a full path or buffer name.
///
/// Returns `"[No Name]"` for empty strings.
#[must_use]
pub fn short_name(full: &str) -> String {
    if full.is_empty() {
        return "[No Name]".to_string();
    }
    Path::new(full)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(full)
        .to_string()
}

/// Collect listed buffer information from Neovim.
///
/// Filters to only loaded, listed (`buflisted`) buffers and marks the
/// current buffer as active.
pub fn collect_buffers() -> Vec<BufferInfo> {
    use nvim_oxi::api;
    use nvim_oxi::api::opts::OptionOpts;

    let current = api::get_current_buf();
    let current_handle = current.handle();

    let mut result = Vec::new();

    for buf in api::list_bufs() {
        if !buf.is_loaded() {
            continue;
        }

        // Check if the buffer is listed (skip unlisted/scratch buffers).
        let listed: bool = api::get_option_value(
            "buflisted",
            &OptionOpts::builder().buffer(buf.clone()).build(),
        )
        .unwrap_or(false);

        if !listed {
            continue;
        }

        // Skip special buffer types (terminal, quickfix, etc.).
        let buftype: String = api::get_option_value(
            "buftype",
            &OptionOpts::builder().buffer(buf.clone()).build(),
        )
        .unwrap_or_default();

        if !buftype.is_empty() {
            continue;
        }

        let full_path = buf
            .get_name()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let modified: bool = api::get_option_value(
            "modified",
            &OptionOpts::builder().buffer(buf.clone()).build(),
        )
        .unwrap_or(false);

        let handle = buf.handle();

        result.push(BufferInfo {
            handle,
            name: short_name(&full_path),
            full_path,
            modified,
            active: handle == current_handle,
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_name_extracts_filename() {
        assert_eq!(short_name("/home/user/project/src/main.rs"), "main.rs");
    }

    #[test]
    fn short_name_relative_path() {
        assert_eq!(short_name("src/lib.rs"), "lib.rs");
    }

    #[test]
    fn short_name_bare_filename() {
        assert_eq!(short_name("README.md"), "README.md");
    }

    #[test]
    fn short_name_empty_returns_no_name() {
        assert_eq!(short_name(""), "[No Name]");
    }

    #[test]
    fn short_name_trailing_slash() {
        // Path ending with separator has no file_name component.
        let result = short_name("/some/dir/");
        // On Unix, Path::file_name returns None for trailing slash.
        assert!(!result.is_empty());
    }

    #[test]
    fn buffer_info_equality() {
        let a = BufferInfo {
            handle: 1,
            name: "main.rs".to_string(),
            full_path: "/src/main.rs".to_string(),
            modified: false,
            active: true,
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
