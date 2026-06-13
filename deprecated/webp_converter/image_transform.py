"""
Robust logo/image transformation utilities for consistent sizing and padding.
"""

import os
import logging
from PIL import Image, UnidentifiedImageError
from .image_utils import save_image_with_transparency


def transform_logo(image_path, output_dir, target_size=300, padding=10):
    """
    Transforms a single logo/image for consistent sizing and padding.
    Args:
        image_path (str): Path to the input image.
        output_dir (str): Directory to save the transformed image.
        target_size (int): Desired width and height for all images (pixels).
        padding (int): Transparent padding to add around the image (pixels).
    Returns:
        output_path (str): Path to the saved transformed image, or None on error.
    """
    try:
        with Image.open(image_path) as img:
            img = img.convert("RGBA")
            original_width, original_height = img.size
            if original_width == 0 or original_height == 0:
                logging.warning(
                    f"Skipping {image_path}: Image has zero width or height."
                )
                return None
            scale = min(target_size / original_width, target_size / original_height)
            new_width = int(original_width * scale)
            new_height = int(original_height * scale)
            resized_img = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
            padded_width = target_size + (2 * padding)
            padded_height = target_size + (2 * padding)
            new_img = Image.new("RGBA", (padded_width, padded_height), (0, 0, 0, 0))
            paste_x = padding + (target_size - new_width) // 2
            paste_y = padding + (target_size - new_height) // 2
            new_img.paste(resized_img, (paste_x, paste_y), resized_img)
            base_name = os.path.basename(image_path)
            output_filename = os.path.splitext(base_name)[0] + ".png"
            output_path = os.path.join(output_dir, output_filename)
            save_image_with_transparency(new_img, output_path, format="PNG")
            logging.info(f"Transformed and saved: {output_path}")
            return output_path
    except FileNotFoundError:
        logging.error(f"Image file not found: {image_path}")
    except UnidentifiedImageError:
        logging.error(f"Unidentified or corrupt image file: {image_path}. Skipping.")
    except Exception as e:
        logging.error(f"Error processing {image_path}: {e}")
    return None


def process_all_logos(input_dir, output_dir, target_size=300, padding=10):
    """
    Processes all image files in a directory using transform_logo.
    Args:
        input_dir (str): Directory containing original images.
        output_dir (str): Directory to save transformed images.
        target_size (int): Desired size for all images (pixels).
        padding (int): Transparent padding (pixels).
    Returns:
        processed (list): List of output file paths for successfully transformed images.
    """
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
        logging.info(f"Created output directory: {output_dir}")
    image_extensions = (".png", ".jpeg", ".jpg", ".gif", ".bmp", ".tiff")
    processed = []
    for filename in os.listdir(input_dir):
        if filename.lower().endswith(image_extensions):
            image_path = os.path.join(input_dir, filename)
            output_path = transform_logo(image_path, output_dir, target_size, padding)
            if output_path:
                processed.append(output_path)
        else:
            logging.debug(f"Skipping non-image file: {filename}")
    return processed
