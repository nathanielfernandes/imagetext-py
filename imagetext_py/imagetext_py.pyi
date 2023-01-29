from imagetext_py.lib import Color, EmojiOptions

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

    def to_buffer(self) -> list[int]:
        """Get the canvas as a buffer.

        Returns:
            list[int]: The canvas as a buffer.
        """

    @staticmethod
    def from_image(image: "Image.Image") -> Canvas:
        """Create a canvas from an image.

        Args:
            image (Image.Image): The image.

        Returns:
            Canvas: The canvas.
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

    def set_anti_alias(self, anti_alias: bool) -> None: 
        """Set the anti_alias of the paint.
        
        default: True

        Args:
            anti_alias (bool): Whether to antialias.
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
    def __new__(self, path: str, fallbacks: list[str]=None, emoji_options: EmojiOptions = None) -> Font:
        """Create a new font.

        Args:
            path (str): The path to the font.
            fallbacks (list[str], optional): The fallback fonts. Defaults to None.
            emoji_options (EmojiOptions, optional): The emoji options. Defaults to the default emoji options.

        Returns:
            Font: The font.
        """

    def set_emoji_options(self, emoji_options: EmojiOptions) -> None:
        """Set the emoji options of the font.

        Args:
            emoji_options (EmojiOptions): The emoji options.
        """
        

class FontDB:
    @staticmethod
    def LoadFromPath(name: str, path: str) -> None:
        """Load a font from a path. The font will be available by name.

        Args:
            name (str): The inputted name of the font.
            path (str): The path to the font.
        """

    @staticmethod
    def LoadFromDir(path: str) -> None:
        """Recursively Load all fonts from a directory.

        Args:
            path (str): The path to the directory.
        """

    @staticmethod
    def LoadSystemFonts() -> None:
        """Load all system found fonts."""

    @staticmethod
    def Query(names: str) -> Font:
        """Query a font by names. ex. 'Segoe-UI Segoe-UI-Emoji Segoe-UI-Symbol'
        
        A font with fallbacks and using default emoji options will be returned.

        Args:
            names (str): The name of the font.

        Returns:
            Font: The font.
        """

    @staticmethod
    def QueryWithEmoji(names: str, emoji_options: EmojiOptions) -> Font:
        """Query a font by names. ex. 'Segoe-UI Segoe-UI-Emoji Segoe-UI-Symbol'
        
        A font with fallbacks will be returned.

        Args:
            names (str): The name of the font.
            emoji_options (EmojiOptions): The emoji options.

        Returns:
            Font: The font.
        """


    @staticmethod
    def Remove(name: str) -> None:
        """Remove a font from the database.

        Args:
            name (str): The name of the font.
        """

    @staticmethod
    def SetDefaultEmojiOptions(emoji_options: EmojiOptions) -> None:
        """Set the default emoji options.

        Args:
            emoji_options (EmojiOptions): The emoji options.
        """
    



class EmojiSource(Enum):
    Twitter = 0
    Apple = 1
    Google = 2
    Microsoft = 3
    Samsung = 4
    Whatsapp = 5
    JoyPixels = 6
    OpenMoji = 7
    Emojidex = 8
    Messenger = 9
    Mozilla = 10
    Lg = 11
    Htc = 12
    Twemoji = 13

class TextAlign(Enum):
    Left = 0
    Center = 1
    Right = 2

class WrapStyle(Enum):
    Word = 0
    Character = 1

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
    draw_emojis: bool=True,
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
        draw_emojis (bool, optional): Whether to draw emojis. Defaults to True.
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
    draw_emojis: bool=True,
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
        draw_emojis (bool, optional): Whether to draw emojis. Defaults to True.
    """



def draw_text_multiline(
    canvas: Canvas,
    lines: list[str],
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
    draw_emojis: bool=True,
) -> None:
    """Draw text on a canvas.

    Args:
        canvas (Canvas): The canvas.
        lines (list[str]): The lines of text to draw.
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
        draw_emojis (bool, optional): Whether to draw emojis. Defaults to True.
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
    draw_emojis: bool=True,
    wrap_style: WrapStyle=WrapStyle.Word,
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
        draw_emojis (bool, optional): Whether to draw emojis. Defaults to True.
        wrap_style (WrapStyle, optional): The wrap style. Defaults to WrapStyle.Word.
    """

def text_size(
    text: str,
    size: float,
    font: Font,
    draw_emojis: bool=True,
) -> tuple[int, int]:
    """Get the size of a text in pixels.

    Args:
        text (str): The text.
        size (float): The size of the text.
        font (Font): The font of the text.
        draw_emojis (bool, optional): Whether to draw emojis. Defaults to True.

    Returns:
        tuple[float, float]: The size of the text.
    """

def text_size_multiline(
    lines: list[str],
    size: float,
    font: Font,
    line_spacing: float=1.0,
    draw_emojis: bool=True,
) -> tuple[int, int]:
    """Get the size of a text in pixels.

    Args:
        lines (list[str]): The lines of text.
        size (float): The size of the text.
        font (Font): The font of the text.
        line_spacing (float, optional): The line spacing. Defaults to 1.0.
        draw_emojis (bool, optional): Whether to draw emojis. Defaults to True.

    Returns:
        tuple[float, float]: The size of the text.
    """


def text_wrap(text: str, width: float, size: float, font: Font, draw_emojis: bool=True, wrap_style: WrapStyle=WrapStyle.Word) -> list[str]:
    """Wrap a text on a given pixel width.

    Args:
        text (str): The text to wrap.
        width (float): The width to wrap on.
        size (float): The size of the text.
        font (Font): The font of the text.
        draw_emojis (bool, optional): Whether to draw emojis. Defaults to True.
        wrap_style (WrapStyle, optional): The wrap style. Defaults to WrapStyle.Word.

    Returns:
        list[str]: The wrapped text.
    """
    