"""
bg_removal.py
Reusable utility for removing image backgrounds using rembg.
"""
from PIL import Image
from rembg import remove
import io

def remove_background(input_image: Image.Image) -> Image.Image:
    """
    Remove the background from a PIL Image using rembg.
    Returns a new PIL Image with background removed (RGBA).
    """
    # rembg expects bytes, so convert PIL Image to bytes
    with io.BytesIO() as buf:
        input_image.save(buf, format="PNG")
        input_bytes = buf.getvalue()
    output_bytes = remove(input_bytes)
    return Image.open(io.BytesIO(output_bytes)).convert("RGBA")
