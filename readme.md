<div align="center">
  
  ![hero banner](https://cdn.discordapp.com/attachments/616149431124885520/1065472311114862612/test.png)
  
  ### A blazing fast text drawing library
  ###### imagetext-py is python bindings for [imagetext](https://github.com/nathanielfernandes/imagetext)


[![CI](https://github.com/nathanielfernandes/imagetext-py/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/nathanielfernandes/imagetext-py/actions/workflows/CI.yml)
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
- Emojis! (almost every platform supported)
- Global Font Database with css-like font querying

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


## Pillow and FontDB Usage 
```python
from PIL import Image
from imagetext_py import *

FontDB.SetDefaultEmojiOptions(EmojiOptions(allow_discord=True))
FontDB.LoadFromDir(".")

font = FontDB.Query("coolvetica japanese")

with Image.new("RGBA", (512, 512), "white") as im:
    with Writer(im) as w:
        w.draw_text_wrapped(
            text="hello from python 😓 lol, <:blobpain:739614945045643447> " \
                 "ほまみ <:chad:682819256173461522><:bigbrain:744344773229543495> " \
                 "emojis workin",
            x=256, y=256,
            ax=0.5, ay=0.5,
            width=512,
            size=90,
            font=font,
            fill=Paint.Color((0, 0, 0, 255)),
            align=TextAlign.Center,
            stroke=2.0,
            stroke_color=Paint.Rainbow((0.0,0.0), (256.0,256.0)),
            draw_emojis=True
        )
    im.save("test.png")
```