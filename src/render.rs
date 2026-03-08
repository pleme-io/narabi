//! Tabline string renderer.
//!
//! Builds the Neovim `'tabline'` string from a list of [`BufferInfo`]
//! entries. Uses `%#HlGroup#` syntax for highlight groups and
//! `%N@SwitchBuf@` syntax for clickable mouse support.

use crate::buffers::BufferInfo;
use crate::theme;

/// Render the full tabline string from a list of buffer info entries.
///
/// Each buffer is rendered as a clickable tab segment with:
/// - Buffer ordinal (1-indexed position in the tabline)
/// - File icon from kamon
/// - Short filename
/// - Modified indicator (`[+]`) when the buffer has unsaved changes
///
/// The `%N@NarabiSwitchBuf@` syntax makes each tab clickable — Neovim
/// calls the `NarabiSwitchBuf` function with the buffer handle when
/// the user clicks on the tab.
#[must_use]
pub fn render_tabline(buffers: &[BufferInfo]) -> String {
    if buffers.is_empty() {
        return format!("%#{}#", theme::HL_FILL);
    }

    let mut s = String::with_capacity(buffers.len() * 80);

    for (idx, buf) in buffers.iter().enumerate() {
        let ordinal = idx + 1;

        // Highlight groups depend on active state.
        let (hl_tab, hl_num, hl_icon, hl_mod) = if buf.active {
            (
                theme::HL_ACTIVE,
                theme::HL_NUMBER_ACTIVE,
                theme::HL_ICON_ACTIVE,
                theme::HL_MODIFIED_ACTIVE,
            )
        } else {
            (
                theme::HL_INACTIVE,
                theme::HL_NUMBER_INACTIVE,
                theme::HL_ICON_INACTIVE,
                theme::HL_MODIFIED_INACTIVE,
            )
        };

        // Look up file icon via kamon.
        let (icon_glyph, _icon_color) = kamon::icon_and_color(&buf.name);

        // Start clickable region: %N@FuncName@ where N is the buffer handle.
        // Neovim passes the handle as the minwid argument to the function.
        s.push_str(&format!("%{}@NarabiSwitchBuf@", buf.handle));

        // Separator before tab (except for the first one).
        if idx > 0 {
            s.push_str(&format!("%#{}# ", theme::HL_SEPARATOR));
        }

        // Buffer ordinal number.
        s.push_str(&format!("%#{}# {} ", hl_num, ordinal));

        // File icon.
        s.push_str(&format!("%#{}#{} ", hl_icon, icon_glyph));

        // Filename.
        s.push_str(&format!("%#{}#{}", hl_tab, buf.name));

        // Modified indicator.
        if buf.modified {
            s.push_str(&format!(" %#{}#[+]", hl_mod));
        }

        // Trailing space inside the tab.
        s.push_str(&format!("%#{}# ", hl_tab));

        // End clickable region.
        s.push_str("%X");
    }

    // Fill the rest of the tabline.
    s.push_str(&format!("%#{}#", theme::HL_FILL));

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_buf(handle: i32, name: &str, modified: bool, active: bool) -> BufferInfo {
        BufferInfo {
            handle,
            name: name.to_string(),
            full_path: format!("/project/src/{name}"),
            modified,
            active,
        }
    }

    #[test]
    fn empty_buffer_list_renders_fill() {
        let result = render_tabline(&[]);
        assert_eq!(result, format!("%#{}#", theme::HL_FILL));
    }

    #[test]
    fn single_active_buffer() {
        let bufs = [make_buf(1, "main.rs", false, true)];
        let result = render_tabline(&bufs);

        // Should contain the active highlight group.
        assert!(result.contains(theme::HL_ACTIVE));
        // Should contain the filename.
        assert!(result.contains("main.rs"));
        // Should contain clickable region start.
        assert!(result.contains("%1@NarabiSwitchBuf@"));
        // Should end clickable region.
        assert!(result.contains("%X"));
        // Should end with fill.
        assert!(result.ends_with(&format!("%#{}#", theme::HL_FILL)));
        // Should NOT contain modified indicator.
        assert!(!result.contains("[+]"));
    }

    #[test]
    fn modified_buffer_shows_indicator() {
        let bufs = [make_buf(3, "config.toml", true, true)];
        let result = render_tabline(&bufs);

        assert!(result.contains("[+]"));
        assert!(result.contains(theme::HL_MODIFIED_ACTIVE));
    }

    #[test]
    fn inactive_buffer_uses_inactive_highlights() {
        let bufs = [make_buf(2, "lib.rs", false, false)];
        let result = render_tabline(&bufs);

        assert!(result.contains(theme::HL_INACTIVE));
        assert!(!result.contains(theme::HL_ACTIVE));
    }

    #[test]
    fn multiple_buffers_have_separators() {
        let bufs = [
            make_buf(1, "main.rs", false, true),
            make_buf(2, "lib.rs", false, false),
            make_buf(3, "mod.rs", true, false),
        ];
        let result = render_tabline(&bufs);

        // Separator highlight should appear between buffers.
        assert!(result.contains(theme::HL_SEPARATOR));
        // All filenames present.
        assert!(result.contains("main.rs"));
        assert!(result.contains("lib.rs"));
        assert!(result.contains("mod.rs"));
        // Ordinals should be sequential.
        assert!(result.contains(&format!("%#{}# 1 ", theme::HL_NUMBER_ACTIVE)));
        assert!(result.contains(&format!("%#{}# 2 ", theme::HL_NUMBER_INACTIVE)));
        assert!(result.contains(&format!("%#{}# 3 ", theme::HL_NUMBER_INACTIVE)));
    }

    #[test]
    fn clickable_regions_use_buffer_handles() {
        let bufs = [
            make_buf(5, "a.rs", false, true),
            make_buf(12, "b.rs", false, false),
        ];
        let result = render_tabline(&bufs);

        assert!(result.contains("%5@NarabiSwitchBuf@"));
        assert!(result.contains("%12@NarabiSwitchBuf@"));
    }

    #[test]
    fn modified_inactive_uses_correct_highlight() {
        let bufs = [make_buf(1, "dirty.rs", true, false)];
        let result = render_tabline(&bufs);

        assert!(result.contains(theme::HL_MODIFIED_INACTIVE));
        assert!(!result.contains(theme::HL_MODIFIED_ACTIVE));
    }

    #[test]
    fn icon_is_present_for_known_file() {
        let bufs = [make_buf(1, "main.rs", false, true)];
        let result = render_tabline(&bufs);

        // kamon should resolve a Rust icon glyph.
        let (glyph, _) = kamon::icon_and_color("main.rs");
        assert!(result.contains(glyph));
    }

    #[test]
    fn no_name_buffer() {
        let bufs = [make_buf(1, "[No Name]", false, true)];
        let result = render_tabline(&bufs);

        assert!(result.contains("[No Name]"));
    }
}
