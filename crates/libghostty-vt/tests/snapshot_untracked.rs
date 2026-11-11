//! Encoding a snapshot mid-sequence without continuation tracking reports
//! `InvalidValue`, as upstream documents.
// These call into libghostty, which Miri cannot execute.
#![cfg(not(miri))]

use libghostty_vt::{Terminal, error::Error};

#[test]
fn encoding_an_untracked_unfinished_sequence_is_invalid() {
    let mut terminal = Terminal::new(20, 4).unwrap();
    terminal.vt_write(b"\x1b[3");
    assert!(matches!(
        terminal.encode_snapshot_alloc(None),
        Err(Error::InvalidValue)
    ));
}
