//! `vt_write_until` stops at string boundaries and hands back the right
//! offsets and string details, run against the real library.
// These call into libghostty, which Miri cannot execute.
#![cfg(not(miri))]

use libghostty_vt::{
    Error, Terminal,
    terminal::{Boundary, StringEnd, StringIntroducer, StringKind},
};

const OSC: Boundary = Boundary::OSC_START.union(Boundary::OSC_END);

#[test]
fn finds_osc_content_ended_by_bel() {
    let mut terminal = Terminal::new(80, 24).unwrap();
    let data = b"ab\x1b]2;title\x07cd";
    let start = terminal.vt_write_until(data, OSC).unwrap().unwrap();
    assert_eq!(start.reached(), Boundary::OSC_START);
    assert_eq!(start.introducer().unwrap(), StringIntroducer::Esc);
    assert_eq!(start.consumed(), 4);
    let rest = &data[start.consumed()..];
    let end = terminal.vt_write_until(rest, OSC).unwrap().unwrap();
    assert_eq!(end.reached(), Boundary::OSC_END);
    assert_eq!(end.kind().unwrap(), StringKind::Osc);
    assert_eq!(end.end().unwrap(), StringEnd::Bel);
    assert_eq!(&rest[..end.consumed() - 1], b"2;title");
    assert!(terminal.vt_write_until(&rest[end.consumed()..], OSC).unwrap().is_none());
    assert_eq!(terminal.title().unwrap(), b"title");
}

#[test]
fn st_stops_after_the_esc_and_the_backslash_comes_next() {
    let mut terminal = Terminal::new(80, 24).unwrap();
    let start = terminal.vt_write_until(b"\x1b]2;t", OSC).unwrap().unwrap();
    assert_eq!(start.consumed(), 2);
    // The string is split across writes; the rest of the first one has no boundary.
    assert!(terminal.vt_write_until(b"2;t", OSC).unwrap().is_none());
    let end = terminal.vt_write_until(b"\x1b\\x", OSC).unwrap().unwrap();
    assert_eq!((end.consumed(), end.end().unwrap()), (1, StringEnd::StEsc));
    terminal.vt_write(b"\\x");
    assert_eq!(terminal.title().unwrap(), b"t");
}

#[test]
fn cancelled_osc_reports_cancelled() {
    let mut terminal = Terminal::new(80, 24).unwrap();
    let data = b"\x1b]2;t\x18";
    let start = terminal.vt_write_until(data, OSC).unwrap().unwrap();
    let end = terminal.vt_write_until(&data[start.consumed()..], OSC).unwrap().unwrap();
    assert_eq!(end.end().unwrap(), StringEnd::Cancelled);
}

#[test]
fn empty_boundaries_are_rejected() {
    let mut terminal = Terminal::new(80, 24).unwrap();
    assert!(matches!(terminal.vt_write_until(b"x", Boundary::empty()), Err(Error::InvalidValue)));
}
