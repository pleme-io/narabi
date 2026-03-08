//! Highlight group definitions for the narabi tabline.
//!
//! Defines highlight groups for active, inactive, and modified buffer
//! tabs using the tane SDK.

use tane::highlight::Highlight;

/// Highlight group name for the tabline background / fill area.
pub const HL_FILL: &str = "NarabiFill";

/// Highlight group name for the active (current) buffer tab.
pub const HL_ACTIVE: &str = "NarabiActive";

/// Highlight group name for inactive buffer tabs.
pub const HL_INACTIVE: &str = "NarabiInactive";

/// Highlight group name for the modified indicator on the active tab.
pub const HL_MODIFIED_ACTIVE: &str = "NarabiModifiedActive";

/// Highlight group name for the modified indicator on inactive tabs.
pub const HL_MODIFIED_INACTIVE: &str = "NarabiModifiedInactive";

/// Highlight group name for the buffer number on the active tab.
pub const HL_NUMBER_ACTIVE: &str = "NarabiNumberActive";

/// Highlight group name for the buffer number on inactive tabs.
pub const HL_NUMBER_INACTIVE: &str = "NarabiNumberInactive";

/// Highlight group name for the file icon on the active tab.
pub const HL_ICON_ACTIVE: &str = "NarabiIconActive";

/// Highlight group name for the file icon on inactive tabs.
pub const HL_ICON_INACTIVE: &str = "NarabiIconInactive";

/// Highlight group name for the separator between tabs.
pub const HL_SEPARATOR: &str = "NarabiSeparator";

/// Register all narabi highlight groups with Neovim.
pub fn setup_highlights() -> tane::Result<()> {
    // Fill / background.
    Highlight::new(HL_FILL)
        .fg("#6e7681")
        .bg("#1e2228")
        .apply()?;

    // Active buffer tab.
    Highlight::new(HL_ACTIVE)
        .fg("#e6edf3")
        .bg("#30363d")
        .bold()
        .apply()?;

    // Inactive buffer tab.
    Highlight::new(HL_INACTIVE)
        .fg("#8b949e")
        .bg("#21262d")
        .apply()?;

    // Modified indicator — active.
    Highlight::new(HL_MODIFIED_ACTIVE)
        .fg("#d29922")
        .bg("#30363d")
        .bold()
        .apply()?;

    // Modified indicator — inactive.
    Highlight::new(HL_MODIFIED_INACTIVE)
        .fg("#d29922")
        .bg("#21262d")
        .apply()?;

    // Buffer number — active.
    Highlight::new(HL_NUMBER_ACTIVE)
        .fg("#79c0ff")
        .bg("#30363d")
        .apply()?;

    // Buffer number — inactive.
    Highlight::new(HL_NUMBER_INACTIVE)
        .fg("#6e7681")
        .bg("#21262d")
        .apply()?;

    // Icon — active.
    Highlight::new(HL_ICON_ACTIVE)
        .fg("#e6edf3")
        .bg("#30363d")
        .apply()?;

    // Icon — inactive.
    Highlight::new(HL_ICON_INACTIVE)
        .fg("#8b949e")
        .bg("#21262d")
        .apply()?;

    // Separator.
    Highlight::new(HL_SEPARATOR)
        .fg("#30363d")
        .bg("#1e2228")
        .apply()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight_group_names_are_unique() {
        let names = [
            HL_FILL,
            HL_ACTIVE,
            HL_INACTIVE,
            HL_MODIFIED_ACTIVE,
            HL_MODIFIED_INACTIVE,
            HL_NUMBER_ACTIVE,
            HL_NUMBER_INACTIVE,
            HL_ICON_ACTIVE,
            HL_ICON_INACTIVE,
            HL_SEPARATOR,
        ];
        let mut seen = std::collections::HashSet::new();
        for name in &names {
            assert!(seen.insert(name), "duplicate highlight group: {name}");
        }
    }

    #[test]
    fn highlight_group_names_start_with_narabi() {
        let names = [
            HL_FILL,
            HL_ACTIVE,
            HL_INACTIVE,
            HL_MODIFIED_ACTIVE,
            HL_MODIFIED_INACTIVE,
            HL_NUMBER_ACTIVE,
            HL_NUMBER_INACTIVE,
            HL_ICON_ACTIVE,
            HL_ICON_INACTIVE,
            HL_SEPARATOR,
        ];
        for name in &names {
            assert!(
                name.starts_with("Narabi"),
                "highlight group must start with Narabi: {name}"
            );
        }
    }
}
