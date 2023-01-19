from __future__ import annotations

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