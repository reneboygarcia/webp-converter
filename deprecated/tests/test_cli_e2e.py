import os
import sys
import unittest
import tempfile
import shutil
from unittest.mock import patch, MagicMock
from PIL import Image

from webp_converter.cli import main

def make_mock_ask(value):
    mock_obj = MagicMock()
    mock_obj.ask.return_value = value
    return mock_obj

class TestCLIE2E(unittest.TestCase):
    def setUp(self):
        self.original_argv = sys.argv
        sys.argv = ["webp-convert"]
        self.temp_dir = tempfile.mkdtemp()
        self.input_dir = os.path.join(self.temp_dir, "input")
        self.output_dir = os.path.join(self.temp_dir, "output")
        os.makedirs(self.input_dir, exist_ok=True)
        os.makedirs(self.output_dir, exist_ok=True)

        # Create a sample input image
        self.sample_png = os.path.join(self.input_dir, "sample.png")
        img = Image.new("RGBA", (20, 20), (255, 0, 0, 255))
        img.save(self.sample_png)

    def tearDown(self):
        sys.argv = self.original_argv
        shutil.rmtree(self.temp_dir)

    @patch("webp_converter.cli.questionary.select")
    def test_menu_exit(self, mock_select):
        mock_select.return_value = make_mock_ask("Exit")
        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

    @patch("webp_converter.cli.questionary.select")
    def test_menu_show_info(self, mock_select):
        mock_select.side_effect = [
            make_mock_ask("Show information"),
            make_mock_ask("Exit")
        ]
        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

    @patch("webp_converter.cli.questionary.confirm")
    @patch("webp_converter.cli.questionary.text")
    @patch("webp_converter.cli.questionary.path")
    @patch("webp_converter.cli.questionary.select")
    def test_convert_to_webp_lossy_success(self, mock_select, mock_path, mock_text, mock_confirm):
        mock_select.side_effect = [
            make_mock_ask("Convert images"),
            make_mock_ask("Convert to WebP"),
            make_mock_ask("Lossy (smaller files, recommended)"),
            make_mock_ask("Exit")
        ]
        mock_path.side_effect = [
            make_mock_ask(self.sample_png),
            make_mock_ask(self.output_dir)
        ]
        mock_text.return_value = make_mock_ask("90")
        mock_confirm.return_value = make_mock_ask(True)

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

        expected_output = os.path.join(self.output_dir, "sample.webp")
        self.assertTrue(os.path.exists(expected_output))
        with Image.open(expected_output) as img:
            self.assertEqual(img.format, "WEBP")

    @patch("webp_converter.cli.questionary.confirm")
    @patch("webp_converter.cli.questionary.text")
    @patch("webp_converter.cli.questionary.path")
    @patch("webp_converter.cli.questionary.select")
    def test_convert_to_webp_lossless_success(self, mock_select, mock_path, mock_text, mock_confirm):
        mock_select.side_effect = [
            make_mock_ask("Convert images"),
            make_mock_ask("Convert to WebP"),
            make_mock_ask("Lossless (may increase file size)"),
            make_mock_ask("Exit")
        ]
        mock_path.side_effect = [
            make_mock_ask(self.sample_png),
            make_mock_ask(self.output_dir)
        ]
        mock_text.return_value = make_mock_ask("80")
        mock_confirm.return_value = make_mock_ask(True)

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

        expected_output = os.path.join(self.output_dir, "sample.webp")
        self.assertTrue(os.path.exists(expected_output))
        with Image.open(expected_output) as img:
            self.assertEqual(img.format, "WEBP")

    @patch("webp_converter.cli.questionary.confirm")
    @patch("webp_converter.cli.questionary.path")
    @patch("webp_converter.cli.questionary.select")
    def test_resize_only_retains_format(self, mock_select, mock_path, mock_confirm):
        mock_select.side_effect = [
            make_mock_ask("Convert images"),
            make_mock_ask("Resize Only (retain original format)"),
            make_mock_ask("Exit")
        ]
        mock_path.side_effect = [
            make_mock_ask(self.sample_png),
            make_mock_ask(self.output_dir)
        ]
        mock_confirm.return_value = make_mock_ask(True)

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

        expected_output = os.path.join(self.output_dir, "sample.png")
        self.assertTrue(os.path.exists(expected_output))
        with Image.open(expected_output) as img:
            self.assertEqual(img.format, "PNG")

    @patch("webp_converter.cli.ask_overwrite")
    @patch("webp_converter.cli.questionary.confirm")
    @patch("webp_converter.cli.questionary.text")
    @patch("webp_converter.cli.questionary.path")
    @patch("webp_converter.cli.questionary.select")
    def test_overwrite_prompt_decline(self, mock_select, mock_path, mock_text, mock_confirm, mock_ask_overwrite):
        # Create output file in advance
        expected_output = os.path.join(self.output_dir, "sample.webp")
        with open(expected_output, "w") as f:
            f.write("existing dummy content")

        mock_select.side_effect = [
            make_mock_ask("Convert images"),
            make_mock_ask("Convert to WebP"),
            make_mock_ask("Lossy (smaller files, recommended)"),
            make_mock_ask("Exit")
        ]
        mock_path.side_effect = [
            make_mock_ask(self.sample_png),
            make_mock_ask(self.output_dir)
        ]
        mock_text.return_value = make_mock_ask("85")
        mock_confirm.return_value = make_mock_ask(False) # Do not force overwrite
        mock_ask_overwrite.return_value = False # Decline overwrite

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

        # Output file should still contain the dummy content (not overwritten)
        with open(expected_output, "r") as f:
            content = f.read()
        self.assertEqual(content, "existing dummy content")

    @patch("webp_converter.cli.ask_overwrite")
    @patch("webp_converter.cli.questionary.confirm")
    @patch("webp_converter.cli.questionary.text")
    @patch("webp_converter.cli.questionary.path")
    @patch("webp_converter.cli.questionary.select")
    def test_overwrite_prompt_accept(self, mock_select, mock_path, mock_text, mock_confirm, mock_ask_overwrite):
        # Create output file in advance
        expected_output = os.path.join(self.output_dir, "sample.webp")
        with open(expected_output, "w") as f:
            f.write("existing dummy content")

        mock_select.side_effect = [
            make_mock_ask("Convert images"),
            make_mock_ask("Convert to WebP"),
            make_mock_ask("Lossy (smaller files, recommended)"),
            make_mock_ask("Exit")
        ]
        mock_path.side_effect = [
            make_mock_ask(self.sample_png),
            make_mock_ask(self.output_dir)
        ]
        mock_text.return_value = make_mock_ask("85")
        mock_confirm.return_value = make_mock_ask(False) # Do not force overwrite
        mock_ask_overwrite.return_value = True # Accept overwrite

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

        # Output file should have been overwritten and be a valid WebP image now
        self.assertTrue(os.path.exists(expected_output))
        with Image.open(expected_output) as img:
            self.assertEqual(img.format, "WEBP")

    def test_cli_argument_help(self):
        sys.argv = ["webp-convert", "--help"]
        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

        sys.argv = ["webp-convert", "-h"]
        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

    @patch("webp_converter.cli.questionary.path")
    @patch("webp_converter.cli.questionary.select")
    def test_workflow_cancel_input_path(self, mock_select, mock_path):
        mock_select.side_effect = [
            make_mock_ask("Convert images"),
            make_mock_ask("Exit")
        ]
        mock_path.return_value = make_mock_ask(None)

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

    @patch("webp_converter.cli.questionary.path")
    @patch("webp_converter.cli.questionary.select")
    def test_workflow_cancel_output_dir(self, mock_select, mock_path):
        mock_select.side_effect = [
            make_mock_ask("Convert images"),
            make_mock_ask("Exit")
        ]
        mock_path.side_effect = [
            make_mock_ask(self.sample_png),
            make_mock_ask(None)
        ]

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

    @patch("webp_converter.cli.questionary.select")
    def test_keyboard_interrupt_graceful_exit(self, mock_select):
        mock_select.side_effect = KeyboardInterrupt()

        with self.assertRaises(SystemExit) as cm:
            main()
        self.assertEqual(cm.exception.code, 0)

if __name__ == "__main__":
    unittest.main()
