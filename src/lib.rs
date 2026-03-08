//! Narabi (並び) — buffer and tab line for Neovim with LSP diagnostics indicators
//!
//! Part of the blnvim-ng distribution — a Rust-native Neovim plugin suite.
//! Built with [`nvim-oxi`](https://github.com/noib3/nvim-oxi) for zero-cost
//! Neovim API bindings.
//!
//! # Features
//!
//! - Renders open buffers in the tabline with file icons (via kamon)
//! - Clickable tabs via `%N@FuncName@` syntax for mouse support
//! - `<leader>1`–`<leader>9` keybindings for fast buffer switching
//! - Modified indicator `[+]` on unsaved buffers
//! - Custom highlight groups for active, inactive, and modified states

pub mod buffers;
pub mod render;
pub mod theme;

use nvim_oxi as oxi;
use nvim_oxi::api;
use nvim_oxi::api::opts::OptionOpts;
use tane::keymap::Keymap;

/// Maximum number of `<leader>N` keybindings to register.
const MAX_LEADER_KEYS: usize = 9;

/// Convert a `tane::Error` into an `oxi::Error` via the API error path.
fn tane_err(err: tane::Error) -> oxi::Error {
    oxi::Error::from(oxi::api::Error::Other(err.to_string()))
}

/// Register `<leader>1` through `<leader>9` keybindings that switch to the
/// Nth listed buffer.
fn register_keymaps() -> oxi::Result<()> {
    for n in 1..=MAX_LEADER_KEYS {
        let cmd = format!(":lua require('narabi').goto_buffer({n})<CR>");
        Keymap::normal(&format!("<leader>{n}"), &cmd)
            .desc(&format!("Switch to buffer {n}"))
            .register()
            .map_err(tane_err)?;
    }
    Ok(())
}

/// Update the Neovim tabline option with the current buffer state.
///
/// Called on `BufEnter`, `BufModifiedSet`, and similar autocommand events
/// to keep the tabline in sync.
fn refresh_tabline() -> oxi::Result<()> {
    let buffers = buffers::collect_buffers();
    let tabline = render::render_tabline(&buffers);
    api::set_option_value("tabline", tabline, &OptionOpts::default())?;
    Ok(())
}

#[oxi::plugin]
fn narabi() -> oxi::Result<()> {
    // Set up highlight groups.
    theme::setup_highlights().map_err(tane_err)?;

    // Register <leader>N keymaps.
    register_keymaps()?;

    // Set the tabline to use our custom rendering.
    // Initial render.
    refresh_tabline()?;

    // Set up autocommands to refresh the tabline on buffer changes.
    let group = api::create_augroup("NarabiTabline", &Default::default())?;

    let opts = oxi::api::opts::CreateAutocmdOpts::builder()
        .group(group)
        .patterns(["*"])
        .callback(|_| {
            // Ignore errors in the autocommand callback — a failed
            // tabline refresh should not break the editor.
            let _ = refresh_tabline();
            Ok::<bool, oxi::Error>(false)
        })
        .build();

    api::create_autocmd(
        [
            "BufEnter",
            "BufAdd",
            "BufDelete",
            "BufModifiedSet",
            "BufFilePost",
        ],
        &opts,
    )?;

    // Export the goto_buffer function for the <leader>N keymaps.
    // This function switches to the Nth listed buffer (1-indexed).
    oxi::api::set_var(
        "narabi_goto_buffer",
        oxi::Function::from_fn(|n: usize| {
            let listed = buffers::collect_buffers();
            if let Some(target) = listed.get(n.saturating_sub(1)) {
                let buf = api::Buffer::from(target.handle);
                let _ = api::set_current_buf(&buf);
            }
        }),
    )?;

    Ok(())
}
