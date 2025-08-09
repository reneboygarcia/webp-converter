#!/usr/bin/env python3
"""
webp_converter.py
A simple CLI tool to convert images to WebP format.
"""
import sys
import os
import argparse
from PIL import Image
from .image_utils import save_image_with_transparency

def convert_to_webp(input_path: str, output_path: str = None) -> None:
    """
    Convert an image to WebP format.

    Args:
        input_path (str): Path to the input image file.
        output_path (str, optional): Path to save the WebP file. If not provided, saves as input.webp.
    Raises:
        FileNotFoundError: If the input file does not exist.
        OSError: If the image cannot be opened or saved.
    """
    if not os.path.isfile(input_path):
        raise FileNotFoundError(f"Input file '{input_path}' does not exist.")

    try:
        with Image.open(input_path) as img:
            img = img.convert('RGBA')
            if not output_path:
                base, _ = os.path.splitext(input_path)
                output_path = base + '.webp'
            save_image_with_transparency(img, output_path, format="WEBP")
            print(f"Converted '{input_path}' to '{output_path}'.")
    except Exception as e:
        print(f"Error converting image: {e}")
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(
        description="Convert an image to WebP format.",
        epilog="Example: python webp_converter.py input.jpg output.webp"
    )
    parser.add_argument('input', help='Path to the input image file')
    parser.add_argument('output', nargs='?', help='Optional output WebP file path')
    args = parser.parse_args()

    convert_to_webp(args.input, args.output)

if __name__ == "__main__":
    main()

