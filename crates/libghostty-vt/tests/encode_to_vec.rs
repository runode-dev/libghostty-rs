//! `encode_to_vec` keeps appending after the buffer's spare capacity runs out.
// These call into libghostty, which Miri cannot execute.
#![cfg(not(miri))]

use libghostty_vt::mouse::{self, Action, Button, EncoderSize, Format, Position, TrackingMode};

#[test]
fn mouse_encode_to_vec_grows_past_spare_capacity() {
    let mut encoder = mouse::Encoder::new().unwrap();
    encoder
        .set_tracking_mode(TrackingMode::Normal)
        .set_format(Format::Sgr)
        .set_size(EncoderSize {
            screen_width: 800,
            screen_height: 600,
            cell_width: 10,
            cell_height: 20,
            padding_top: 0,
            padding_bottom: 0,
            padding_left: 0,
            padding_right: 0,
        });
    let mut event = mouse::Event::new().unwrap();
    event
        .set_action(Action::Press)
        .set_button(Some(Button::Four))
        .set_position(Position { x: 15.0, y: 25.0 });

    // Many wheel steps encoded into one buffer, as a fast scroll does. The
    // starting capacity leaves a gap smaller than one sequence after a few.
    let mut out = Vec::with_capacity(64);
    for _ in 0..100 {
        encoder.encode_to_vec(&event, &mut out).unwrap();
    }
    assert_eq!(out, b"\x1b[<64;2;2M".repeat(100));
}
