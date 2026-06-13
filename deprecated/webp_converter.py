#!/usr/bin/env python3
"""
webp_converter.py
A simple CLI tool to convert images to WebP format.
"""
import sys
import os
import argparse

# Insert parent directory to path to resolve local package import when run as standalone script
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from webp_converter.cli import convert_to_webp

def main():
    parser = argparse.ArgumentParser(
        description="Convert an image to WebP format.",
        epilog="Example: python webp_converter.py input.jpg output.webp"
    )
    parser.add_argument('input', help='Path to the input image file')
    parser.add_argument('output', nargs='?', help='Optional output WebP file path')
    args = parser.parse_args()

    # Reuse convert_to_webp from the package (displays beautiful Rich panels!)
    success = convert_to_webp(args.input, args.output)
    if not success:
        sys.exit(1)

if __name__ == "__main__":
    main()
