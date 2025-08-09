"""
image_utils.py
Shared utilities for image conversion and saving with transparency (DRY principle).
"""
from PIL import Image

def save_image_with_transparency(img, output_path, format="PNG", lossless=False, **kwargs):
    """
    Convert image to RGBA and save as PNG or WebP (preserves transparency).
    Args:
        img (PIL.Image.Image): Image to save.
        output_path (str): Path to save the image.
        format (str): 'PNG' or 'WEBP'.
        lossless (bool): If True, use lossless WebP. Default False (lossy, smaller).
        kwargs: Additional arguments for PIL save.
    """
    img = img.convert("RGBA")
    if format.upper() == "WEBP":
        img.save(output_path, "WEBP", lossless=lossless, **kwargs)
    else:
        img.save(output_path, "PNG", **kwargs)

