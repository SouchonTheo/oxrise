use core_graphics::{
    event::CGEvent,
    event_source::{CGEventSource, CGEventSourceStateID},
};

pub fn get_mouse_position() -> core_graphics::geometry::CGPoint {
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .expect("Error creating event source");
    let event = CGEvent::new(source).expect("Error creating event");
    event.location()
}
