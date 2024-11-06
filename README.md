# Render0 - a tiny software renderer in rust

1. tiny renderer inspired software rasterization renderer based on softbuffer and winit

2. it's all about pixels

* draw line (Bresenham's line drawing algorithm)
* triangle resterization & back face culling
* z buffer (hidden faces removal)
* perspective projection
* moving the camera
* shaders for the software renderer
* tangent space normal mapping
* shadow mapping
* ambient occlusion


```py
def main():
    # setup window
    el = EventLoop()
    window = WindowBuilder().build(el)
    context = softbuffer.Context(window)
    surface = softbuffer.Surface(context, window)

    # model
    models = load_model()

    texture = load_texture()
    (w, h) = (texture.size().width, texture.size().height)
```

![render0](https://upload.wikimedia.org/wikipedia/commons/d/d1/Rendering_eq.png)
