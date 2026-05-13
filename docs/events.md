Listeners:

ui.listen(handle, |e: &EventType, ui| {}) — listen on a specific widget
ui.listen_once(handle, ...) — fires once then removes itself
ui.listen_while(handle, |e, ui| bool) — fires while closure returns true
ui.listen_global(|e: &EventType, ui| {}) — listens globally
ui.listen_global_once(...) / ui.listen_global_while(...)  — same as above
ui.listen_off(handle) — unsubscribe a listener
ui.send_to(handle, event) — send event to a specific widget
ui.send_global(event) — broadcast event globally

Nested listeners work, including Ui::listen_off
Mutable state in closures works without any Rc/Cell/RefCell/Mutex

Built-in events: Click, MouseDown, MouseUp, MouseMove, MouseScroll, HoverEnter, HoverLeave, KeyPress, KeyRelease, FocusGained, FocusLost, MouseEnter, MouseLeave, WindowResized
