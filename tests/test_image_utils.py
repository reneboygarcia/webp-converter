import os
import unittest
import tempfile
import shutil
from PIL import Image
from webp_converter.image_utils import save_image_with_transparency

class TestImageUtils(unittest.TestCase):
    def setUp(self):
        # Create a temp directory for outputs
        self.test_dir = tempfile.mkdtemp()

    def tearDown(self):
        # Remove temp directory
        shutil.rmtree(self.test_dir)

    def test_save_image_with_transparency_png(self):
        # Create an RGBA image
        img = Image.new("RGBA", (50, 50), (255, 0, 0, 128))
        output_path = os.path.join(self.test_dir, "test_rgba.png")
        
        save_image_with_transparency(img, output_path, format="PNG")
        
        # Verify it exists and is RGBA
        self.assertTrue(os.path.exists(output_path))
        with Image.open(output_path) as loaded_img:
            self.assertEqual(loaded_img.mode, "RGBA")
            # Verify transparency value is preserved
            pixel = loaded_img.getpixel((0, 0))
            self.assertEqual(pixel[3], 128)

    def test_save_image_with_transparency_webp(self):
        # Create an RGBA image with actual transparency
        img = Image.new("RGBA", (50, 50), (0, 255, 0, 128))
        output_path = os.path.join(self.test_dir, "test_rgba.webp")
        
        save_image_with_transparency(img, output_path, format="WEBP")
        
        # Verify it exists and has transparency capability
        self.assertTrue(os.path.exists(output_path))
        with Image.open(output_path) as loaded_img:
            self.assertEqual(loaded_img.mode, "RGBA")
            pixel = loaded_img.getpixel((0, 0))
            self.assertEqual(pixel[3], 128)

if __name__ == "__main__":
    unittest.main()
