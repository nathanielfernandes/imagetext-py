from imagetext_py.lib import *

from enum import Enum

try:
    from PIL import Image
except:
    pass

class Canvas:
    def __new__(self, width: int, height: int, color: Color) -> Canvas:
        """Create a new canvas.

        Args:
            width (int): The width of the canvas.
            height (int): The height of the canvas.
            color (Color): The color of the canvas.

        Returns:
            Canvas: The canvas.
        """

    def save(self, path: str) -> None: 
        """Save the canvas to a file.

        Args:
            path (str): The path to save the file to.
        """

    def to_bytes(self) -> tuple[tuple[int, int], bytes]: 
        """Get the canvas as bytes.

        Returns:
            tuple[tuple[int, int], bytes]: The canvas dimensions and bytes.
        """

    def to_image(self) -> "Image.Image": 
        """Get the canvas as a PIL image.

        Returns:
            The canvas as an image.
        """


class Paint:
    def __new__(self, color: Color=(0, 0, 0, 255), anti_alias: bool = True) -> Paint:
        """Create a new paint.

        Args:
            color (Color, optional): The color of the paint. Defaults to (0, 0, 0, 255).
            anti_alias (bool, optional): Whether to antialias. Defaults to True.

        Returns:
            Paint: The paint.
        """
    
    def set_color(self, color: Color) -> None: 
        """Set the color of the paint.

        Args:
            color (Color): The color.
        """

    def set_antialias(self, antialias: bool) -> None: 
        """Set the antialias of the paint.
        
        default: True

        Args:
            antialias (bool): Whether to antialias.
        """

    @staticmethod
    def Color(color: Color) -> Paint:
        """Create a paint with a color.

        Args:
            color (Color): The color.

        Returns:
            Paint: The paint.
        """

    @staticmethod
    def Gradient(start: tuple[float, float], stop: tuple[float, float], colors: list[Color]) -> Paint:
        """Create a paint with a gradient.

        Args:
            start (tuple[float, float]): The start of the gradient.
            stop (tuple[float, float]): The stop of the gradient.
            colors (list[Color]): The colors of the gradient.

        Returns:
            Paint: The paint.
        """

    @staticmethod
    def Rainbow(start: tuple[float, float], stop: tuple[float, float]) -> Paint:
        """Create a paint with a rainbow gradient.

        Args:
            start (tuple[float, float]): The start of the gradient.
            stop (tuple[float, float]): The stop of the gradient.

        Returns:
            Paint: The paint.
        """


class Font:
    def __new__(self, path: str, fallbacks: list[str]=None) -> Font:
        """Create a new font.

        Args:
            path (str): The path to the font.
            fallbacks (list[str], optional): The fallback fonts. Defaults to None.

        Returns:
            Font: The font.
        """

class TextAlign(Enum):
    Left = 0
    Center = 1
    Right = 2


def draw_text(
    canvas: Canvas,
    text: str,
    x: float,
    y: float,
    size: float,
    font: Font,
    fill: Paint,
    stroke: Paint=None,
    stroke_color: Paint=None,
) -> None:
    """Draw text on a canvas.

    Args:
        canvas (Canvas): The canvas.
        text (str): The text to draw.
        x (float): The x position of the text.
        y (float): The y position of the text.
        size (float): The size of the text.
        font (Font): The font of the text.
        fill (Paint): The fill of the text.
        stroke (Paint, optional): The stroke of the text. Defaults to None.
        stroke_color (Paint, optional): The stroke color of the text. Defaults to None.
    """

def draw_text_anchored(
    canvas: Canvas,
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
) -> None:
    """Draw text on a canvas.

    Args:
        canvas (Canvas): The canvas.
        text (str): The text to draw.
        x (float): The x position of the text.
        y (float): The y position of the text.
        ax (float): The x anchor of the text.
        ay (float): The y anchor of the text.
        size (float): The size of the text.
        font (Font): The font of the text.
        fill (Paint): The fill of the text.
        stroke (Paint, optional): The stroke of the text. Defaults to None.
        stroke_color (Paint, optional): The stroke color of the text. Defaults to None.
    """



def draw_text_multiline(
    canvas: Canvas,
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
) -> None:
    """Draw text on a canvas.

    Args:
        canvas (Canvas): The canvas.
        text (str): The text to draw.
        x (float): The x position of the text.
        y (float): The y position of the text.
        ax (float): The x anchor of the text.
        ay (float): The y anchor of the text.
        width (float): The width of the text.
        size (float): The size of the text.
        font (Font): The font of the text.
        fill (Paint): The fill of the text.
        line_spacing (float, optional): The line spacing. Defaults to 1.0.
        align (TextAlign, optional): The text alignment. Defaults to TextAlign.Left.
        stroke (Paint, optional): The stroke of the text. Defaults to None.
        stroke_color (Paint, optional): The stroke color of the text. Defaults to None.
    """


def draw_text_wrapped(
    canvas: Canvas,
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
) -> None:
    """Draw text on a canvas.

    Args:
        canvas (Canvas): The canvas.
        text (str): The text to draw.
        x (float): The x position of the text.
        y (float): The y position of the text.
        ax (float): The x anchor of the text.
        ay (float): The y anchor of the text.
        width (float): The width of the text.
        size (float): The size of the text.
        font (Font): The font of the text.
        fill (Paint): The fill of the text.
        line_spacing (float, optional): The line spacing. Defaults to 1.0.
        align (TextAlign, optional): The text alignment. Defaults to TextAlign.Left.
        stroke (Paint, optional): The stroke of the text. Defaults to None.
        stroke_color (Paint, optional): The stroke color of the text. Defaults to None.
    """

def text_size(
    text: str,
    size: float,
    font: Font,
) -> tuple[int, int]:
    """Get the size of a text in pixels.

    Args:
        text (str): The text.
        size (float): The size of the text.
        font (Font): The font of the text.

    Returns:
        tuple[float, float]: The size of the text.
    """

def text_size_multiline(
    text: str,
    size: float,
    font: Font,
    line_spacing: float=1.0,
) -> tuple[int, int]:
    """Get the size of a text in pixels.

    Args:
        text (str): The text.
        size (float): The size of the text.
        font (Font): The font of the text.
        line_spacing (float, optional): The line spacing. Defaults to 1.0.

    Returns:
        tuple[float, float]: The size of the text.
    """

def split_on_space(text: str) -> list[str]:
    """Split a string on spaces.

    Args:
        text (str): The text to split.

    Returns:
        list[str]: The split text.
    """

def word_wrap(text: str, width: float, size: float, font: Font ) -> list[str]:
    """Wrap a text on a given pixel width.

    Args:
        text (str): The text to wrap.
        width (float): The width to wrap on.
        size (float): The size of the text.
        font (Font): The font of the text.

    Returns:
        list[str]: The wrapped text.
    """
    