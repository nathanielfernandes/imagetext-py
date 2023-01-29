from __future__ import annotations

from imagetext_py.imagetext_py import *

try:
    from PIL import Image
except:
    pass

class Color(tuple):
    def __new__(cls, r: int, g: int, b: int, a: int = 255) -> Color:
        """Create a new color.

        Args:
            r (int): The red value of the color.
            g (int): The green value of the color.
            b (int): The blue value of the color.
            a (int, optional): The alpha value of the color. Defaults to 255.
        """
        return super().__new__(cls, (r, g, b, a))

    @staticmethod
    def from_hex(hex: str) -> Color:
        """Create a color from a hex string.

        Args:
            hex (str): The hex string.

        Returns:
            Color: The color.
        """
        return Color(*bytes.fromhex(hex))

    def __repr__(self) -> str:
        return f"Color({self.r}, {self.g}, {self.b}, {self.a})"

    @property
    def r(self) -> int:
        """The red value of the color."""
        return self[0]

    @property
    def g(self) -> int:
        """The green value of the color."""
        return self[1]

    @property
    def b(self) -> int:
        """The blue value of the color."""
        return self[2]

    @property
    def a(self) -> int:
        """The alpha value of the color."""
        return self[3]



class EmojiOptions:
    def __init__(self, scale: float = 1.0, shift: tuple[int, int] = (0, 0), parse_shortcodes: bool = True, parse_discord_emojis: bool = False, source: EmojiSource = EmojiSource.Twitter) -> None:
        """Create a new emoji options object.

        Args:
            scale (float, optional): The scale of the emojis. Defaults to 1.0.
            shift (tuple[int, int], optional): The shift of the emojis. Defaults to (0, 0).
            parse_shortcodes (bool, optional): Whether to parse shortcodes. Defaults to True.
            parse_discord_emojis (bool, optional): Whether to parse discord emojis. Defaults to False.
            source (EmojiSource, optional): The emoji source. Defaults to EmojiSource.Twitter.
        """
        self.scale = scale
        self.shift = shift
        self.parse_shortcodes = parse_shortcodes
        self.parse_discord_emojis = parse_discord_emojis
        self.source = source


class Writer:
    def __init__(self, image: "Image.Image") -> None:
        """Create a new draw object.

        Args:
            image (Image.Image): The image to draw on.
        """
        self.image = image
        self.__canvas = None

    def __enter__(self) -> Writer:
        assert self.image.mode == "RGBA", "The image must be in RGBA mode."
        self.__canvas = Canvas.from_image(self.image)
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        self.image.frombytes(self.__canvas.to_bytes()[1])
        self.__canvas = None

    @property
    def _canvas(self) -> Canvas:
        assert self.__canvas is not None, "You must use the Writer as a context manager."
        return self.__canvas

    def draw_text(
        self,
        text: str,
        x: float,
        y: float,
        size: float,
        font: Font,
        fill: Paint,
        stroke: Paint=None,
        stroke_color: Paint=None,
        draw_emojis: bool=False,
    ) -> None:
        """Draw text on the image.

        Args:
            text (str): The text to draw.
            x (float): The x position of the text.
            y (float): The y position of the text.
            size (float): The size of the text.
            font (Font): The font to use.
            fill (Paint): The fill paint.
            stroke (Paint, optional): The stroke paint. Defaults to None.
            stroke_color (Paint, optional): The stroke color. Defaults to None.
            draw_emojis (bool, optional): Whether to draw emojis. Defaults to False.
        """
        draw_text(self._canvas, text, x, y, size, font, fill, stroke, stroke_color, draw_emojis)

    def draw_text_anchored(
        self,
        text: str,
        x: float,
        y: float,
        ax: float,
        ay: float,
        size: float,
        font: Font,
        fill: Paint,
        stroke: Paint=None,
        stroke_color: Paint=None,
        draw_emojis: bool=False,
    ) -> None:
        """Draw text on the image.

        Args:
            text (str): The text to draw.
            x (float): The x position of the text.
            y (float): The y position of the text.
            ax (float): The x anchor of the text.
            ay (float): The y anchor of the text.
            size (float): The size of the text.
            font (Font): The font to use.
            fill (Paint): The fill paint.
            stroke (Paint, optional): The stroke paint. Defaults to None.
            stroke_color (Paint, optional): The stroke color. Defaults to None.
            draw_emojis (bool, optional): Whether to draw emojis. Defaults to False.
        """
        draw_text_anchored(self._canvas, text, x, y, ax, ay, size, font, fill, stroke, stroke_color, draw_emojis)

    def draw_text_multiline(
        self,
        text: str,
        x: float,
        y: float,
        ax: float,
        ay: float,
        width: float,
        size: float,
        font: Font,
        fill: Paint,
        line_spacing: float=1.0,
        align: TextAlign=TextAlign.Left,
        stroke: Paint=None,
        stroke_color: Paint=None,
        draw_emojis: bool=False,
    ) -> None:
        """Draw text on the image.

        Args:
            text (str): The text to draw.
            x (float): The x position of the text.
            y (float): The y position of the text.
            ax (float): The x anchor of the text.
            ay (float): The y anchor of the text.
            width (float): The width of the text.
            size (float): The size of the text.
            font (Font): The font to use.
            fill (Paint): The fill paint.
            line_spacing (float, optional): The line spacing. Defaults to 1.0.
            align (TextAlign, optional): The text alignment. Defaults to TextAlign.Left.
            stroke (Paint, optional): The stroke paint. Defaults to None.
            stroke_color (Paint, optional): The stroke color. Defaults to None.
            draw_emojis (bool, optional): Whether to draw emojis. Defaults to False.
        """
        draw_text_multiline(self._canvas, text, x, y, ax, ay, width, size, font, fill, line_spacing, align, stroke, stroke_color, draw_emojis)


    def draw_text_wrapped(
        self,
        text: str,
        x: float,
        y: float,
        ax: float,
        ay: float,
        width: float,
        size: float,
        font: Font,
        fill: Paint,
        line_spacing: float=1.0,
        align: TextAlign=TextAlign.Left,
        stroke: Paint=None,
        stroke_color: Paint=None,
        draw_emojis: bool=False,
        wrap_style: WrapStyle=WrapStyle.Word,
    ) -> None:
        """Draw text on the image.

        Args:
            text (str): The text to draw.
            x (float): The x position of the text.
            y (float): The y position of the text.
            ax (float): The x anchor of the text.
            ay (float): The y anchor of the text.
            width (float): The width of the text.
            size (float): The size of the text.
            font (Font): The font to use.
            fill (Paint): The fill paint.
            line_spacing (float, optional): The line spacing. Defaults to 1.0.
            align (TextAlign, optional): The text alignment. Defaults to TextAlign.Left.
            stroke (Paint, optional): The stroke paint. Defaults to None.
            stroke_color (Paint, optional): The stroke color. Defaults to None.
            draw_emojis (bool, optional): Whether to draw emojis. Defaults to False.
            wrap_style (WrapStyle, optional): The wrap style. Defaults to WrapStyle.Word.
        """
        draw_text_wrapped(self._canvas, text, x, y, ax, ay, width, size, font, fill, line_spacing, align, stroke, stroke_color, draw_emojis, wrap_style)