import os
import unittest
import tempfile
import shutil
from PIL import Image
from webp_converter.cli import (
    convert_to_webp_core, 
    convert_to_webp, 
    WebPConverterCLI
)

class TestCLI(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.mkdtemp()
        self.input_file = os.path.join(self.temp_dir, "input.png")
        img = Image.new("RGB", (10, 10), (255, 255, 255))
        img.save(self.input_file)
        self.output_file = os.path.join(self.temp_dir, "output.webp")

    def tearDown(self):
        shutil.rmtree(self.temp_dir)

    def test_convert_to_webp_core_success(self):
        metrics = convert_to_webp_core(
            self.input_file, 
            self.output_file, 
            quality=85, 
            lossless=False
        )
        self.assertIsInstance(metrics, dict)
        self.assertIn("original_size", metrics)
        self.assertIn("new_size", metrics)
        self.assertEqual(metrics["quality"], 85)
        self.assertTrue(os.path.exists(self.output_file))

    def test_convert_to_webp_core_failure(self):
        with self.assertRaises(Exception):
            # Pass a non-existent file
            convert_to_webp_core("non_existent.jpg", self.output_file)

    def test_convert_to_webp_wrapper(self):
        # Test converting through wrapper in silent mode
        success = convert_to_webp(
            self.input_file,
            self.output_file,
            force=True,
            quality=80,
            lossless=False,
            silent=True
        )
        self.assertTrue(success)
        self.assertTrue(os.path.exists(self.output_file))

    def test_cli_helper_parse_inputs(self):
        cli = WebPConverterCLI()
        inputs = cli._parse_inputs("file1.png,  file2.jpg,file3.png ")
        self.assertEqual(inputs, ["file1.png", "file2.jpg", "file3.png"])

    def test_cli_helper_validate_inputs_exist(self):
        cli = WebPConverterCLI()
        # Test missing input path
        self.assertFalse(cli._validate_inputs_exist([self.input_file, "ghost_file.jpg"]))
        # Test existing paths
        self.assertTrue(cli._validate_inputs_exist([self.input_file]))

    def test_cli_helper_get_image_files_from_dir(self):
        cli = WebPConverterCLI()
        
        # Create a subfolder structure
        sub_dir = os.path.join(self.temp_dir, "sub")
        os.makedirs(sub_dir)
        
        # Save a JPG and a WebP
        img1 = os.path.join(sub_dir, "pic1.jpg")
        Image.new("RGB", (5, 5)).save(img1)
        img2 = os.path.join(sub_dir, "pic2.webp")
        Image.new("RGB", (5, 5)).save(img2)
        
        # Save a non-image file
        txt = os.path.join(sub_dir, "note.txt")
        with open(txt, "w") as f:
            f.write("junk")
            
        output_dir = os.path.join(self.temp_dir, "out")
        
        # Walk directory
        results = list(cli._get_image_files_from_dir(sub_dir, output_dir=output_dir, input_root=sub_dir))
        
        # We expect 2 files (pic1.jpg -> will convert, pic2.webp -> will copy)
        self.assertEqual(len(results), 2)
        
        paths = {os.path.basename(r[0]): r for r in results}
        self.assertIn("pic1.jpg", paths)
        self.assertIn("pic2.webp", paths)
        
        # Verify action codes
        self.assertFalse(paths["pic1.jpg"][2]) # is_webp=False
        self.assertTrue(paths["pic2.webp"][2]) # is_webp=True

    def test_cli_helper_parse_inputs_empty(self):
        cli = WebPConverterCLI()
        self.assertEqual(cli._parse_inputs(""), [])
        self.assertEqual(cli._parse_inputs("  ,  "), [])
        self.assertEqual(cli._parse_inputs("file.png, , file2.jpg"), ["file.png", "file2.jpg"])

    def test_cli_helper_validate_inputs_exist_empty(self):
        cli = WebPConverterCLI()
        self.assertFalse(cli._validate_inputs_exist([]))
        self.assertFalse(cli._validate_inputs_exist([""]))

    def test_cli_get_downloads_dir(self):
        from webp_converter.cli import get_downloads_dir
        downloads = get_downloads_dir()
        self.assertTrue(isinstance(downloads, str))
        self.assertTrue(len(downloads) > 0)

    def test_convert_to_webp_invalid_image(self):
        txt_file = os.path.join(self.temp_dir, "invalid.txt")
        with open(txt_file, "w") as f:
            f.write("Not an image")
        out_webp = os.path.join(self.temp_dir, "invalid.webp")
        success = convert_to_webp(txt_file, out_webp, silent=True)
        self.assertFalse(success)

    def test_convert_to_webp_silent_mode(self):
        # When silent is true, convert_to_webp returns False and does not print or raise
        success = convert_to_webp("non_existent_file.png", "out.webp", silent=True)
        self.assertFalse(success)

if __name__ == "__main__":
    unittest.main()
