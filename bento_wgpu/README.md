# bento_wgpu

2D GPU renderer built on [wgpu](https://github.com/gfx-rs/wgpu), designed as the rendering backend for [bento](https://github.com/oganas/bento).

Scene nodes can be added to a `Scene` and rendered with a `Renderer`.

## Shape rendering

## Text rendering: 
full unicode, emoji, and bidirectional support. relying on [cosmic-text](https://github.com/pop-os/cosmic-text)
Customisable with per character ranges:
- Foreground color
- Background color
- Underline
- Strikethrough
- Weight
- Italic
- Font
Text wrapping with max_width
Two layer cache separating redrawing and reshaping, which depend on which properties changed

## Performance
GPU uploads only on change

