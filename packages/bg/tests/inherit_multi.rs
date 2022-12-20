use std::any::Any;

use bg::{build, builder};

pub trait MaybeEventListener {
    fn handle_event(&mut self, event: &dyn Any);
}

impl MaybeEventListener for () {
    fn handle_event(&mut self, _: &dyn Any) {}
}

impl<F> MaybeEventListener for F
where
    F: FnMut(&dyn Any),
{
    fn handle_event(&mut self, _: &dyn Any) {
        // do something
    }
}

builder! {
    #![inheritable]
    pub struct EventTargetProps {
        on_change[impl MaybeEventListener]: () = (),
        on_click [impl MaybeEventListener]: () = (),
    }
}

builder! {
    #![inheritable]
    pub struct AriaProps {
        aria_hidden[? bool],
        aria_label [borrow? str],
    }
}

builder! {
    #![inheritable]
    pub struct ElementProps {
        event_listeners[inherit EventTargetProps],
        aria[inherit AriaProps],
    }
}

#[test]
fn build() {
    let mut triggered_events = vec![];
    let mut props = build!(ElementProps {
        on_change: |_: &_| triggered_events.push("change"),
        aria_hidden: true,
    });
    assert!(props.aria.aria_hidden);
    (props.event_listeners.on_change)(&());
    assert_eq!(triggered_events, ["change"]);
}
