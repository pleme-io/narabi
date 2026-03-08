//! Narabi (並び) — buffer and tab line for Neovim with LSP diagnostics indicators
//!
//! Part of the blnvim-ng distribution — a Rust-native Neovim plugin suite.
//! Built with [`nvim-oxi`](https://github.com/noib3/nvim-oxi) for zero-cost
//! Neovim API bindings.

use nvim_oxi as oxi;

#[oxi::plugin]
fn narabi() -> oxi::Result<()> {
    Ok(())
}
