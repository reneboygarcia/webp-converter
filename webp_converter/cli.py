import os
os.environ["OMP_DISPLAY_ENV"] = "FALSE"
import sys
import os
import shutil
from pathlib import Path
from PIL import Image
from rich.console import Console
from rich.panel import Panel
from tqdm import tqdm
import questionary
from questionary import Style
from .ui_helpers import show_success, show_error, show_warning, show_info, ask_overwrite
from .image_utils import save_image_with_transparency
from .bg_removal import remove_background

# ANSI color codes for retro terminal style
CYAN = "\033[96m"
MAGENTA = "\033[95m"
YELLOW = "\033[93m"
GREEN = "\033[92m"
RED = "\033[91m"
RESET = "\033[0m"
BOLD = "\033[1m"

RETRO_ASCII = """

██╗    ██╗███████╗██████╗ ██████╗        ██████╗ ██████╗ ███╗   ██╗██╗   ██╗███████╗██████╗ ████████╗
██║    ██║██╔════╝██╔══██╗██╔══██╗      ██╔════╝██╔═══██╗████╗  ██║██║   ██║██╔════╝██╔══██╗╚══██╔══╝
██║ █╗ ██║█████╗  ██████╔╝██████╔╝█████╗██║     ██║   ██║██╔██╗ ██║██║   ██║█████╗  ██████╔╝   ██║   
██║███╗██║██╔══╝  ██╔══██╗██╔═══╝ ╚════╝██║     ██║   ██║██║╚██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗   ██║   
╚███╔███╔╝███████╗██████╔╝██║           ╚██████╗╚██████╔╝██║ ╚████║ ╚████╔╝ ███████╗██║  ██║   ██║   
 ╚══╝╚══╝ ╚══════╝╚═════╝ ╚═╝            ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝   ╚═╝   
                                                                                                     
"""

RETRO_BORDER = f"{CYAN}{BOLD}+{'-'*40}+{RESET}"

def retro_print(msg, color=CYAN, border=True):
    if border:
        print(RETRO_BORDER)
    for line in msg.split("\n"):
        if line.strip():
            print(f"{color}| {line.ljust(36)} |{RESET}")
    if border:
        print(RETRO_BORDER)


def get_downloads_dir() -> str:
    """Return the user's Downloads directory in a cross-platform way."""
    if os.name == "nt":
        import ctypes
        from pathlib import Path
        try:
            from ctypes import windll, wintypes
            CSIDL_PERSONAL = 0x0005
            SHGFP_TYPE_CURRENT = 0
            buf = ctypes.create_unicode_buffer(wintypes.MAX_PATH)
            windll.shell32.SHGetFolderPathW(
                None, CSIDL_PERSONAL, None, SHGFP_TYPE_CURRENT, buf
            )
            doc_path = Path(buf.value)
            downloads = doc_path.parent / "Downloads"
            return str(downloads)
        except Exception:
            return str(Path.home() / "Downloads")
    else:
        return os.path.join(os.path.expanduser("~"), "Downloads")


from .ui_helpers import show_success, show_error, show_warning, show_info, ask_overwrite
from .image_utils import save_image_with_transparency


def convert_to_webp_core(
    input_path: str,
    output_path: str,
    quality: int = 80,
    remove_bg: bool = False,
    lossless: bool = False,
) -> bool:
    """
    Core image-to-WebP conversion logic. No user interaction or file existence checks.
    Returns True on success, False on error.
    """
    try:
        from .bg_removal import remove_background
        with Image.open(input_path) as img:
            img = img.convert("RGBA")
            def has_transparency(image):
                if image.mode in ("RGBA", "LA"):
                    alpha = image.getchannel("A")
                    return alpha.getextrema()[0] < 255
                return False
            if remove_bg and not has_transparency(img):
                img = remove_background(img)
            elif remove_bg and has_transparency(img):
                show_info("Image already has transparency. Skipping background removal.", title="Background Removal")
            save_image_with_transparency(img, output_path, format="WEBP", lossless=lossless, quality=quality)
            original_size = os.path.getsize(input_path)
            new_size = os.path.getsize(output_path)
            show_success(
                os.path.basename(input_path),
                os.path.basename(output_path),
                original_size,
                new_size,
                quality,
            )
            return True
    except Exception as e:
        show_error(str(e), title="Conversion Error")
        return False

def convert_to_webp(
    input_path: str,
    output_path: str = None,
    force: bool = False,
    quality: int = 80,
    remove_bg: bool = False,
    lossless: bool = False,
) -> bool:
    """
    Wrapper for image-to-WebP conversion. Handles file existence, output path, and user interaction.
    Returns True on success, False on error.
    """
    if not os.path.isfile(input_path):
        show_error(f"Input file '{input_path}' does not exist.")
        return False
    if not output_path:
        base = os.path.splitext(os.path.basename(input_path))[0]
        output_path = os.path.join(get_downloads_dir(), base + ".webp")
    if os.path.exists(output_path) and not force:
        if not ask_overwrite(os.path.basename(output_path)):
            show_info("Conversion skipped by user.", title="Skipped")
            return False
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    return convert_to_webp_core(
        input_path,
        output_path,
        quality=quality,
        remove_bg=remove_bg,
        lossless=lossless,
    )


def retro_input(prompt_msg, default=None):
    border_color = CYAN
    arrow_color = MAGENTA
    prompt_color = YELLOW
    block = f"{border_color}{BOLD}█{RESET}"
    border = block * 25
    print(f"\n{border}")
    print(f"{block}{' ' * 3}{prompt_color}{prompt_msg}{RESET}")
    if default:
        print(
            f"{block}{' ' * 3}{arrow_color}▶{RESET} Press Enter for default: {GREEN}{default}{RESET}"
        )
    print(f"{border}")
    inp = input(f"{arrow_color}{BOLD}➤ {RESET}")
    return inp.strip() if inp.strip() else (default if default is not None else None)


def prompt_for_inputs():
    msg = "Enter input image file(s) or a folder path (comma-separated for multiple files):"
    files = retro_input(msg)
    return [f.strip() for f in files.split(",") if f.strip()]


def prompt_for_output(default_path):
    msg = f"Enter output file path [default: {default_path}]"
    out = retro_input(msg, default=default_path)
    return out


def prompt_for_directory(default_dir):
    msg = f"Enter output directory for all files [default: {default_dir}]"
    out = retro_input(msg, default=default_dir)
    return out


def _create_progress_bar(total_images):
    return tqdm(
        total=total_images,
        unit="img",
        desc="Converting",
        bar_format="{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt} [{elapsed}<{remaining}]",
    )


CUSTOM_STYLE = Style(
    [
        ("qmark", "fg:#00d7af bold"),
        ("question", "bold"),
        ("answer", "fg:#ffaf00 bold"),
        ("pointer", "fg:#00d7af bold"),
        ("highlighted", "fg:#00d7af bold"),
        ("selected", "fg:#5f87ff bold"),
        ("instruction", "fg:#888888 italic"),
    ]
)

class WebPConverterCLI:
    def __init__(self):
        self.console = Console()

    def show_welcome(self):
        self.console.print(f"[bold cyan]{RETRO_ASCII}[/bold cyan]")
        self.console.print(
            Panel.fit(
                "Convert images to WebP with style!\n\n"
                "Quick Start:\n"
                "1. Choose 'Convert Images'\n"
                "2. Select files/folders\n"
                "3. Set output options\n"
                "4. Enjoy your WebPs!\n\n"
                "Navigation:\n"
                "• Use ↑/↓ arrows to move\n"
                "• Press Enter to select\n"
                "• Press Esc to go back",
                title=" 🌎 WebP Converter",
                border_style="cyan",
            )
        )

    def main_menu(self):
        while True:
            choice = questionary.select(
                "What would you like to do?",
                choices=[
                    "Convert images",
                    "Show information",
                    "Exit",
                ],
                use_indicator=True,
                instruction="(Use ↑/↓ arrows and Enter to select, Esc to exit)",
                qmark="🔹",
                style=CUSTOM_STYLE,
            ).ask()

            if choice is None or choice == "Exit":
                self.console.print("👋 Goodbye!", style="yellow")
                sys.exit(0)
            elif choice == "Convert images":
                self.convert_images_workflow()
            elif choice == "Show information":
                self.show_info()

    def convert_images_workflow(self):
        input_path = self._get_input_path()
        inputs = self._parse_inputs(input_path)
        if not self._validate_inputs_exist(inputs):
            self.console.print("[red]One or more input paths do not exist.[/red]")
            return

        output_dir = self._get_output_dir(inputs)
        mode = self._get_operation_mode()
        quality, lossless, force, remove_bg = self._get_conversion_options(mode)

        files_to_convert = self._get_files_to_convert(inputs, output_dir, mode)
        if not files_to_convert:
            self.console.print("[yellow]No images found to process.[/yellow]")
            return

        self._process_files(files_to_convert, mode, quality, lossless, force, remove_bg)

    def _get_input_path(self):
        return questionary.path(
            "Enter input image file(s) or a folder path (comma-separated for multiple files):",
            qmark="🖼️ ",
            style=CUSTOM_STYLE,
        ).ask()

    def _get_output_dir(self, inputs):
        default_dir = get_downloads_dir()
        output_dir = questionary.path(
            f"Enter output directory for all files (default: {default_dir})",
            default=default_dir,
            qmark="📂 ",
            style=CUSTOM_STYLE,
        ).ask()
        if not output_dir:
            output_dir = default_dir
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
        return output_dir

    def _get_operation_mode(self):
        return questionary.select(
            "Choose operation mode:",
            choices=[
                "Convert to WebP",
                "Resize Only (retain original format)",
            ],
            qmark="🔧 ",
            style=CUSTOM_STYLE,
        ).ask()

    def _get_conversion_options(self, mode):
        quality = 80
        lossless = False
        if mode == "Convert to WebP":
            lossless_choice = questionary.select(
                "WebP compression mode (smaller files = Lossy, may increase size = Lossless):",
                choices=[
                    "Lossy (smaller files, recommended)",
                    "Lossless (may increase file size)",
                ],
                default="Lossy (smaller files, recommended)",
                qmark="🗜️ ",
                style=CUSTOM_STYLE,
            ).ask()
            lossless = (lossless_choice.startswith("Lossless"))
            quality = questionary.text(
                "WebP quality (0-100, default: 80):",
                default="80",
                validate=lambda val: val.isdigit() and 0 <= int(val) <= 100,
                qmark="🎚️ ",
                style=CUSTOM_STYLE,
            ).ask()
            quality = int(quality or 80)

        force = questionary.confirm(
            "Overwrite output file(s) without prompting?",
            default=False,
            qmark="⚠️ ",
            style=CUSTOM_STYLE,
        ).ask()

        remove_bg = False
        if mode == "Convert to WebP":
            remove_bg = questionary.confirm(
                "Remove background from images? (uses AI)",
                default=False,
                qmark="🪄 ",
                style=CUSTOM_STYLE,
            ).ask()

        return quality, lossless, force, remove_bg

    def _get_files_to_convert(self, inputs, output_dir, mode):
        files_to_convert = []
        for input_path in inputs:
            if os.path.isdir(input_path):
                for input_file, output_file, is_webp in self._get_image_files_from_dir(input_path, output_dir, input_path):
                    if is_webp:
                        files_to_convert.append((input_file, output_file, 'copy'))
                    else:
                        files_to_convert.append((input_file, output_file, 'convert'))
            else:
                file_lower = input_path.lower()
                if file_lower.endswith('.webp'):
                    rel_name = os.path.basename(input_path)
                    output_file = os.path.join(output_dir, rel_name)
                    files_to_convert.append((input_path, output_file, 'copy'))
                else:
                    rel_name = os.path.splitext(os.path.basename(input_path))[0] + '.webp'
                    output_file = os.path.join(output_dir, rel_name)
                    files_to_convert.append((input_path, output_file, 'convert'))
        return files_to_convert

    def _process_files(self, files_to_convert, mode, quality, lossless, force, remove_bg):
        errors = []
        from PIL import Image
        def resize_and_save(input_path, output_path):
            try:
                with Image.open(input_path) as img:
                    img = img.copy()
                    img.save(output_path)
                return True
            except Exception as e:
                return str(e)

        if mode == "Resize Only (retain original format)":
            if len(files_to_convert) > 1:
                with _create_progress_bar(len(files_to_convert)) as pbar:
                    for file_path, output_path, action in files_to_convert:
                        try:
                            if action == 'copy':
                                os.makedirs(os.path.dirname(output_path), exist_ok=True)
                                shutil.copy2(file_path, output_path)
                                self.console.print(
                                    Panel.fit(
                                        f"[yellow]Skipped (already WebP), copied to:[/yellow] {file_path} → {output_path}",
                                        border_style="yellow",
                                    )
                                )
                            else:
                                result = resize_and_save(file_path, output_path)
                                if result is not True:
                                    errors.append((file_path, result))
                        except Exception as e:
                            errors.append((file_path, str(e)))
                        pbar.update(1)
                total = len(files_to_convert)
                failed = len(errors)
                succeeded = total - failed
                if errors:
                    fail_list = "\n".join(f"{os.path.basename(f)}: {e}" for f, e in errors)
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully resized/copied:[/green] {succeeded}\n"
                        f"[red]Failed:[/red] {failed}\n\n"
                        f"[red]Failed files:[/red]\n{fail_list}"
                    )
                    self.console.print(
                        Panel.fit(
                            summary,
                            border_style="red",
                        )
                    )
                else:
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully resized/copied:[/green] {total}\n"
                        f"[red]Failed:[/red] 0"
                    )
                    self.console.print(
                        Panel.fit(
                            summary,
                            border_style="green",
                        )
                    )
            else:
                file_path, output_path, action = files_to_convert[0]
                try:
                    if action == 'copy':
                        os.makedirs(os.path.dirname(output_path), exist_ok=True)
                        shutil.copy2(file_path, output_path)
                        self.console.print(
                            Panel.fit(
                                f"[yellow]Skipped (already WebP), copied to:[/yellow] {file_path} → {output_path}",
                                border_style="yellow",
                            )
                        )
                    else:
                        result = resize_and_save(file_path, output_path)
                        if result is not True:
                            raise Exception(result)
                        self.console.print(
                            Panel.fit(
                                f"[green]Resized:[/green] {file_path} → {output_path}",
                                border_style="green",
                            )
                        )
                except Exception as e:
                    self.console.print(
                        Panel.fit(
                            f"[red]Failed to resize: {file_path}\nError: {e}[/red]",
                            border_style="red",
                        )
                    )
        else:
            if len(files_to_convert) > 1:
                with _create_progress_bar(len(files_to_convert)) as pbar:
                    for file_path, output_path, action in files_to_convert:
                        try:
                            if action == 'copy':
                                os.makedirs(os.path.dirname(output_path), exist_ok=True)
                                shutil.copy2(file_path, output_path)
                                self.console.print(
                                    Panel.fit(
                                        f"[yellow]Skipped (already WebP), copied to:[/yellow] {file_path} → {output_path}",
                                        border_style="yellow",
                                    )
                                )
                            else:
                                os.makedirs(os.path.dirname(output_path), exist_ok=True)
                                convert_to_webp(
                                    file_path,
                                    output_path,
                                    force=force,
                                    quality=quality,
                                    remove_bg=remove_bg,
                                    lossless=lossless,
                                )
                        except Exception as e:
                            errors.append((file_path, str(e)))
                        pbar.update(1)
                total = len(files_to_convert)
                failed = len(errors)
                succeeded = total - failed
                # Calculate counts for summary
                converted_count = sum(1 for _, _, a in files_to_convert if a == 'convert')
                copied_count = sum(1 for _, _, a in files_to_convert if a == 'copy')
                if errors:
                    fail_list = "\n".join(f"{os.path.basename(f)}: {e}" for f, e in errors)
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully converted:[/green] {converted_count - failed}\n"
                        f"[yellow]Copied (WebP):[/yellow] {copied_count}\n"
                        f"[red]Failed conversions:[/red] {failed}\n\n"
                        f"[red]Failed files:[/red]\n{fail_list}"
                    )
                    self.console.print(
                        Panel.fit(
                            summary,
                            border_style="red",
                        )
                    )
                else:
                    # Calculate counts for summary
                    converted_count = sum(1 for _, _, a in files_to_convert if a == 'convert')
                    copied_count = sum(1 for _, _, a in files_to_convert if a == 'copy')
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully converted:[/green] {converted_count}\n"
                        f"[yellow]Copied (WebP):[/yellow] {copied_count}\n"
                        f"[red]Failed:[/red] 0"
                    )
                    self.console.print(
                        Panel.fit(
                            summary,
                            border_style="green",
                        )
                    )
            else:
                file_path, output_path, action = files_to_convert[0]
                try:
                    if action == 'copy':
                        shutil.copy2(file_path, output_path)
                        self.console.print(
                            Panel.fit(
                                f"[green]Copied:[/green] {file_path} → {output_path}",
                                border_style="green",
                            )
                        )
                    else:
                        convert_to_webp(
                            file_path,
                            output_path,
                            force=force,
                            quality=quality,
                            remove_bg=remove_bg,
                            lossless=lossless,
                        )
                        self.console.print(
                            Panel.fit(
                                f"[green]Converted:[/green] {file_path} → {output_path}",
                                border_style="green",
                            )
                        )
                except Exception as e:
                    self.console.print(
                        Panel.fit(
                            f"[red]Failed to convert: {file_path}\nError: {e}[/red]",
                            border_style="red",
                        )
                    )

    def _parse_inputs(self, input_path):
        return [f.strip() for f in input_path.split(",") if f.strip()]

    def _validate_inputs_exist(self, inputs):
        return all(os.path.exists(p) for p in inputs)

    def _get_image_files_from_dir(self, directory, output_dir=None, input_root=None):
        """
        Recursively yield (input_file, output_file, is_webp) tuples, preserving folder structure.
        If file is .webp, is_webp=True; else, is_webp=False.
        """
        if input_root is None:
            input_root = directory
        for root, _, files in os.walk(directory):
            for file in files:
                file_lower = file.lower()
                if not file_lower.endswith(tuple(Image.registered_extensions().keys())):
                    continue
                input_file = os.path.join(root, file)
                if output_dir:
                    rel_path = os.path.relpath(input_file, input_root)
                    if file_lower.endswith('.webp'):
                        output_file = os.path.join(output_dir, rel_path)
                        yield (input_file, output_file, True)
                    else:
                        rel_path = os.path.splitext(rel_path)[0] + '.webp'
                        output_file = os.path.join(output_dir, rel_path)
                        yield (input_file, output_file, False)
                else:
                    yield (input_file, None, file_lower.endswith('.webp'))

    def _get_image_files(self, inputs, output_dir):
        """
        Yield (input_file, output_file, is_webp) for all files in inputs, preserving structure.
        """
        for input_path in inputs:
            if os.path.isdir(input_path):
                for input_file, output_file, is_webp in self._get_image_files_from_dir(input_path, output_dir, input_path):
                    yield (input_file, output_file, is_webp)
            else:
                file_lower = input_path.lower()
                if file_lower.endswith('.webp'):
                    rel_name = os.path.basename(input_path)
                    output_file = os.path.join(output_dir, rel_name)
                    yield (input_path, output_file, True)
                else:
                    rel_name = os.path.splitext(os.path.basename(input_path))[0] + '.webp'
                    output_file = os.path.join(output_dir, rel_name)
                    yield (input_path, output_file, False)

    def show_info(self):
        self.console.print(
            Panel.fit(
                "A command-line tool to convert images to WebP format.\n\n"
                "Features:\n"
                "• Batch/folder conversion\n"
                "• Retro/modern terminal UI\n"
                "• Progress bars\n"
                "• Friendly prompts\n\n"
                "For more info, visit: https://github.com/reneboygarcia/webp-converter",
                title="ℹ️ About WebP Converter",
                border_style="cyan",
            )
        )


def main():
    cli = WebPConverterCLI()
    cli.show_welcome()
    cli.main_menu()


if __name__ == "__main__":
    main()
