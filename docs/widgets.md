Widgets:

ui.add(widget) — adds a widget, returns a handle
ui.remove(handle) — removes a widget at end of frame
ui.get(handle) — immutable reference to widget
ui.get_mut(handle) — mutable reference to widget
ui.set_children(parent, [child1, child2]) — sets parent-child relationship, reparents scene nodes

Built-in widgets: Rect, Text, Button, Container
