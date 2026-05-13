# Widgets:

### Built-in widgets: 
[`Rect`](widgets/rect.md)
[`Text`](widgets/text.md)
[`Button`](widgets/button.md)
[`Container`](widgets/container.md)

### Manage widgets:
`ui.add(widget)`: adds a widget, returns a `WidgetHandle`   
`ui.remove(WidgetHandle)`: removes a widget at end of frame   
`ui.get(WidgetHandle)`: immutable reference to widget   
`ui.get_mut(WidgetHandle)`: mutable reference to widget   
`ui.set_children(parent, [child1, child2])`: sets parent-child relationship, reparents scene nodes   




