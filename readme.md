<div align="center">
  
  ![hero banner](https://cdn.discordapp.com/attachments/616149431124885520/1065472311114862612/test.png)
  
  ### A blazing fast text drawing library
  ###### imagetext-py is python bindings for [imagetext](https://github.com/nathanielfernandes/imagetext)

  ---
  
</div>

## About 
imagetext makes use of [rusttype](https://github.com/redox-os/rusttype) for font parsing and [tiny-skia](https://github.com/RazrFalcon/tiny-skia) for drawing. It has a simple API that allows you to draw text with ease.

## Features
- Multi-line text
- Text wrapping
- Text alignment
- Font fallbacks
- Text stroke 
- Gradient fills 

## Installation

```bash
pip install imagetext-py
```

## Example Usage

```python
from PIL import Image
from imagetext_py import *

# supports fonts with fallbacks
font = Font("coolvetica.ttf", fallbacks=["emojis.ttf", "japanese.otf"])

# create a canvas to draw on
cv = Canvas(512, 512, (255, 255, 255, 255))

# paints are used to fill and stroke text
black = Paint.Color((0, 0, 0, 255))
rainbow = Paint.Rainbow((0.0,0.0), (256.0,256.0))

# if a font doesn't have a glyph for a character, it will use the fallbacks
text = "hello my 😓 n🐢ame i☕s 会のすべ aての構成員 nathan and i drink soup boop coop, the quick brown fox jumps over the lazy dog"

draw_text_wrapped(canvas=cv,              # the canvas to draw on
                  text=text, 
                  x=256, y=256,           # the position of the text
                  ax=0.5, ay=0.5,         # the anchor of the text
                  size=67,                # the size of the text
                  width=500,              # the width of the text
                  font=font,              
                  fill=black,
                  align=TextAlign.Center,
                  stroke=2.0,             # the stroke width (optional)
                  stroke_color=rainbow)   # the stroke color (optional)

# you can convert the canvas to a PIL image
im: Image.Image = cv.to_image()
im.save("test.png")

# or you can just get the raw bytes
dimensions, bytes = cv.to_bytes()

# you can also save directly to a file
cv.save("test.png")
```
produces this image:

![test.png](https://cdn.discordapp.com/attachments/749779629643923548/1065477410281246791/image.png)

##### took `6ms` to draw this on my machine