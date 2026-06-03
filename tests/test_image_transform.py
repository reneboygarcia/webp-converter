import os
import unittest
import tempfile
import shutil
from PIL import Image
from webp_converter.image_transform import transform_logo, process_all_logos

class TestImageTransform(unittest.TestCase):
    def setUp(self):
        # Create temp directories for inputs and outputs
        self.input_dir = tempfile.mkdtemp()
        self.output_dir = tempfile.mkdtemp()
        
        # Create a mock input image
        self.mock_image_path = os.path.join(self.input_dir, "logo.png")
        img = Image.new("RGBA", (100, 200), (255, 0, 0, 255))
        img.save(self.mock_image_path)

    def tearDown(self):
        # Clean up temp directories
        shutil.rmtree(self.input_dir)
        shutil.rmtree(self.output_dir)

    def test_transform_logo(self):
        target_size = 150
        padding = 15
        
        output_path = transform_logo(
            self.mock_image_path,
            self.output_dir,
            target_size=target_size,
            padding=padding
        )
        
        # Verify output exists and is resized correctly with padding
        self.assertIsNotNone(output_path)
        self.assertTrue(os.path.exists(output_path))
        
        with Image.open(output_path) as out_img:
            # Padded size should be target_size + (2 * padding)
            expected_size = target_size + (2 * padding)
            self.assertEqual(out_img.size, (expected_size, expected_size))
            self.assertEqual(out_img.mode, "RGBA")

    def test_process_all_logos(self):
        # Save another mock image
        img2 = Image.new("RGB", (300, 300), (0, 0, 255))
        img2.save(os.path.join(self.input_dir, "icon.jpg"))
        
        processed_files = process_all_logos(self.input_dir, self.output_dir, target_size=100, padding=5)
        
        # We expect 2 processed outputs (both logo.png and icon.jpg converted to pngs)
        self.assertEqual(len(processed_files), 2)
        for f in processed_files:
            self.assertTrue(os.path.exists(f))
            self.assertTrue(f.endswith(".png"))

if __name__ == "__main__":
    unittest.main()
