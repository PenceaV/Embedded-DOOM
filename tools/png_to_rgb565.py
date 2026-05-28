#!/usr/bin/env python3
"""
Convert a 16x16 PNG (with transparency) to an Rgb565 const in sprites.rs.
Usage:  python tools/png_to_rgb565.py assets/gun.png
        python tools/png_to_rgb565.py assets/enemy.png
"""

import sys
import re
from pathlib import Path
from PIL import Image

SPRITE_W = 16
SPRITE_H = 16
SPRITES_RS = Path(__file__).parent.parent / "src" / "game" / "engine" / "sprites.rs"


def to_rgb565_components(r: int, g: int, b: int) -> tuple[int, int, int]:
    """
    Return the raw 5-bit R, 6-bit G, 5-bit B channel values that
    embedded-graphics Rgb565::new() expects (NOT packed u16 values).
    """
    r5 = r >> 3          # 8-bit -> 5-bit
    g6 = g >> 2          # 8-bit -> 6-bit
    b5 = b >> 3          # 8-bit -> 5-bit
    return r5, g6, b5


def png_to_const(png_path: Path) -> str:
    img = Image.open(png_path).convert("RGBA")
    if img.size != (SPRITE_W, SPRITE_H):
        raise ValueError(f"Expected {SPRITE_W}x{SPRITE_H}, got {img.size}")

    # Derive a SCREAMING_SNAKE_CASE const name from the filename.
    # e.g. "gun.png" -> "GUN", "enemy_walk.png" -> "ENEMY_WALK"
    stem = png_path.stem.upper()
    stem = re.sub(r"[^A-Z0-9]+", "_", stem).strip("_")
    const_name = f"SPRITE_{stem}"

    pixels = []
    for y in range(SPRITE_H):
        row = []
        for x in range(SPRITE_W):
            r, g, b, a = img.getpixel((x, y))
            if a < 128:
                # Transparent pixel — sentinel value recognised by the renderer
                row.append("TRANSPARENT")
            else:
                r5, g6, b5 = to_rgb565_components(r, g, b)
                row.append(f"C({r5},{g6},{b5})")
        pixels.append(row)

    # Format as a 16-row block so the source mirrors the sprite visually.
    rows_str = ""
    for row in pixels:
        rows_str += "    " + ", ".join(row) + ",\n"

    return (
        f"/// Auto-generated from {png_path.name} — do not edit by hand.\n"
        f"pub const {const_name}: Sprite = [\n"
        f"{rows_str}"
        f"];\n"
    )


def update_sprites_rs(new_const: str, const_name: str):
    text = SPRITES_RS.read_text() if SPRITES_RS.exists() else ""

    # Replace existing const of the same name, or append.
    pattern = rf"/// Auto-generated from.*?pub const {const_name}.*?\];\n"
    if re.search(pattern, text, re.DOTALL):
        text = re.sub(pattern, new_const, text, flags=re.DOTALL)
        print(f"Updated {const_name} in {SPRITES_RS}")
    else:
        text += "\n" + new_const
        print(f"Appended {const_name} to {SPRITES_RS}")

    SPRITES_RS.write_text(text)


def main():
    if len(sys.argv) < 2:
        print("Usage: python tools/png_to_rgb565.py <path/to/sprite.png>")
        sys.exit(1)

    for arg in sys.argv[1:]:
        path = Path(arg)
        if not path.exists():
            print(f"File not found: {path}")
            sys.exit(1)
        const_str = png_to_const(path)
        # Extract const name from generated string
        name_match = re.search(r"pub const (\w+):", const_str)
        update_sprites_rs(const_str, name_match.group(1))


if __name__ == "__main__":
    main()