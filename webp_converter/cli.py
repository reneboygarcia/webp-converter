import os
import sys
import shutil
from pathlib import Path
from PIL import Image
from rich.console import Console
from rich.panel import Panel
from rich.progress import Progress, BarColumn, TextColumn, TimeElapsedColumn, TimeRemainingColumn
import questionary
from questionary import Style
from concurrent.futures import ThreadPoolExecutor, as_completed
from .ui_helpers import show_success, show_error, show_warning, show_info, ask_overwrite
from .image_utils import save_image_with_transparency

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

RETRO_ASCII_SMALL = """
██      ██  ███████  ██████   ██████ 
██      ██  ██       ██   ██  ██   ██
██  ██  ██  █████    ██████   ██████ 
██  ██  ██  ██       ██   ██  ██     
 ███  ███   ███████  ██████   ██     

 ████   ████  ██  ██ ██  ██ ██████ █████  ██████ ██████ █████ 
██     ██  ██ ███ ██ ██  ██ ██     ██  ██   ██   ██     ██  ██
██     ██  ██ ██████  ████  ████   █████    ██   ████   █████ 
██     ██  ██ ██ ███  ████  ██     ██ ██    ██   ██     ██ ██ 
 ████   ████  ██  ██   ██   ██████ ██  ██   ██   ██████ ██  ██
"""



def get_downloads_dir() -> str:
    """Return the user's Downloads directory in a cross-platform way."""
    if os.name == "nt":
        import ctypes
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




def convert_to_webp_core(
    input_path: str,
    output_path: str,
    quality: int = 80,
    lossless: bool = False,
) -> dict:
    """
    Core image-to-WebP conversion logic. No user interaction or file existence checks.
    Returns a dict of file metrics on success, or raises an exception.
    """
    with Image.open(input_path) as img:
        if img.mode != "RGBA":
            img = img.convert("RGBA")
        save_image_with_transparency(img, output_path, format="WEBP", lossless=lossless, quality=quality)
        return {
            "original_size": os.path.getsize(input_path),
            "new_size": os.path.getsize(output_path),
            "quality": quality,
        }

def convert_to_webp(
    input_path: str,
    output_path: str = None,
    force: bool = False,
    quality: int = 80,
    lossless: bool = False,
    silent: bool = False,
) -> bool:
    """
    Wrapper for image-to-WebP conversion. Handles file existence, output path, and user interaction.
    Returns True on success, False on error.
    """
    if not os.path.isfile(input_path):
        if not silent:
            show_error(f"Input file '{input_path}' does not exist.")
        return False
    if not output_path:
        base = os.path.splitext(os.path.basename(input_path))[0]
        output_path = os.path.join(get_downloads_dir(), base + ".webp")
    if os.path.exists(output_path) and not force:
        if silent:
            return False
        if not ask_overwrite(os.path.basename(output_path)):
            show_info("Conversion skipped by user.", title="Skipped")
            return False
    dir_name = os.path.dirname(output_path)
    if dir_name:
        os.makedirs(dir_name, exist_ok=True)
    try:
        metrics = convert_to_webp_core(
            input_path,
            output_path,
            quality=quality,
            lossless=lossless,
        )
        if not silent:
            show_success(
                input_path,
                output_path,
                metrics["original_size"],
                metrics["new_size"],
                metrics["quality"],
            )
        return True
    except Exception as e:
        if not silent:
            show_error(str(e), title="Conversion Error")
        return False




class RichProgressBar:
    def __init__(self, total):
        self.progress = Progress(
            TextColumn("Converting: [progress.percentage]{task.percentage:>3.0f}%"),
            BarColumn(bar_width=40),
            TextColumn("[progress.completed]{task.completed}/{task.total}"),
            TextColumn("["),
            TimeElapsedColumn(),
            TextColumn("<"),
            TimeRemainingColumn(),
            TextColumn("]"),
        )
        self.task_id = self.progress.add_task("Converting", total=total)

    def __enter__(self):
        self.progress.start()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.progress.stop()

    def update(self, n=1):
        self.progress.update(self.task_id, advance=n)


def _create_progress_bar(total_images):
    return RichProgressBar(total_images)


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
        terminal_width = self.console.width
        if terminal_width >= 105:
            self.console.print(f"[bold cyan]{RETRO_ASCII}[/bold cyan]", soft_wrap=True)
        elif terminal_width >= 66:
            self.console.print(f"[bold cyan]{RETRO_ASCII_SMALL}[/bold cyan]", soft_wrap=True)
        else:
            self.console.print("[bold cyan]⚡ WEBP CONVERTER ⚡[/bold cyan]\n")
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
        try:
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
        except KeyboardInterrupt:
            self.console.print("\n👋 Goodbye!", style="yellow")
            sys.exit(0)

    def convert_images_workflow(self):
        input_path = self._get_input_path()
        if input_path is None:
            self.console.print("↩ Returned to main menu.", style="yellow")
            return
        inputs = self._parse_inputs(input_path)
        if not self._validate_inputs_exist(inputs):
            self.console.print("[red]One or more input paths do not exist.[/red]")
            return

        output_dir = self._get_output_dir(inputs)
        if output_dir is None:
            self.console.print("↩ Returned to main menu.", style="yellow")
            return
        mode = self._get_operation_mode()
        if mode is None:
            self.console.print("↩ Returned to main menu.", style="yellow")
            return
        options = self._get_conversion_options(mode)
        if options is None:
            self.console.print("↩ Returned to main menu.", style="yellow")
            return
        quality, lossless, force = options

        files_to_convert = self._get_files_to_convert(inputs, output_dir, mode)
        if not files_to_convert:
            self.console.print("[yellow]No images found to process.[/yellow]")
            return

        self._process_files(files_to_convert, mode, quality, lossless, force)

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
        if output_dir is None:
            return None
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
            if lossless_choice is None:
                return None
            lossless = (lossless_choice.startswith("Lossless"))
            quality_choice = questionary.text(
                "WebP quality (0-100, default: 80):",
                default="80",
                validate=lambda val: val.isdigit() and 0 <= int(val) <= 100,
                qmark="🎚️ ",
                style=CUSTOM_STYLE,
            ).ask()
            if quality_choice is None:
                return None
            quality = int(quality_choice or 80)

        force = questionary.confirm(
            "Overwrite output file(s) without prompting?",
            default=False,
            qmark="⚠️ ",
            style=CUSTOM_STYLE,
        ).ask()
        if force is None:
            return None

        return quality, lossless, force

    def _get_files_to_convert(self, inputs, output_dir, mode):
        files_to_convert = []
        is_resize_only = mode.startswith("Resize Only") if mode else False
        for input_path in inputs:
            if os.path.isdir(input_path):
                for input_file, output_file, is_webp in self._get_image_files_from_dir(input_path, output_dir, input_path):
                    if is_resize_only and not is_webp:
                        orig_ext = os.path.splitext(input_file)[1]
                        output_file = os.path.splitext(output_file)[0] + orig_ext
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
                    if is_resize_only:
                        rel_name = os.path.basename(input_path)
                    else:
                        rel_name = os.path.splitext(os.path.basename(input_path))[0] + '.webp'
                    output_file = os.path.join(output_dir, rel_name)
                    files_to_convert.append((input_path, output_file, 'convert'))
        return files_to_convert

    def _process_files(self, files_to_convert, mode, quality, lossless, force):
        errors = []

        # Pre-filter files to check for overwrites before starting execution
        filtered_files = []
        if not force:
            for file_path, output_path, action in files_to_convert:
                if os.path.exists(output_path):
                    if ask_overwrite(os.path.basename(output_path)):
                        filtered_files.append((file_path, output_path, action))
                else:
                    filtered_files.append((file_path, output_path, action))
            files_to_convert = filtered_files
        
        if not files_to_convert:
            self.console.print("[yellow]No files to process (all skipped or already up to date).[/yellow]")
            return

        def resize_and_save(input_path, output_path):
            try:
                with Image.open(input_path) as img:
                    img.save(output_path)
                return True
            except Exception as e:
                return str(e)

        def process_single_resize(item):
            file_path, output_path, action = item
            try:
                dir_name = os.path.dirname(output_path)
                if dir_name:
                    os.makedirs(dir_name, exist_ok=True)
                if action == 'copy':
                    shutil.copy2(file_path, output_path)
                    return ('copy', file_path, output_path, True)
                else:
                    result = resize_and_save(file_path, output_path)
                    if result is True:
                        return ('resize', file_path, output_path, True)
                    else:
                        return ('resize', file_path, output_path, result)
            except Exception as e:
                return ('error', file_path, output_path, str(e))

        def process_single_convert(item):
            file_path, output_path, action = item
            try:
                dir_name = os.path.dirname(output_path)
                if dir_name:
                    os.makedirs(dir_name, exist_ok=True)
                if action == 'copy':
                    shutil.copy2(file_path, output_path)
                    return ('copy', file_path, output_path, True)
                else:
                    # We pass force=True because overwrite has already been confirmed/pre-filtered,
                    # and silent=True to prevent terminal output pollution.
                    result = convert_to_webp(
                        file_path,
                        output_path,
                        force=True,
                        quality=quality,
                        lossless=lossless,
                        silent=True,
                    )
                    if result:
                        return ('convert', file_path, output_path, True)
                    else:
                        return ('convert', file_path, output_path, "Conversion failed")
            except Exception as e:
                return ('error', file_path, output_path, str(e))

        if mode == "Resize Only (retain original format)":
            if len(files_to_convert) > 1:
                copied_count = 0
                resized_count = 0
                with _create_progress_bar(len(files_to_convert)) as pbar:
                    max_workers = min(8, os.cpu_count() or 4)
                    with ThreadPoolExecutor(max_workers=max_workers) as executor:
                        futures = {executor.submit(process_single_resize, item): item for item in files_to_convert}
                        for future in as_completed(futures):
                            action_type, file_path, output_path, status = future.result()
                            if status is not True:
                                errors.append((file_path, status))
                            else:
                                if action_type == 'copy':
                                    copied_count += 1
                                else:
                                    resized_count += 1
                            pbar.update(1)
                
                total = len(files_to_convert)
                failed = len(errors)
                succeeded = total - failed
                if errors:
                    fail_list = "\n".join(f"{os.path.basename(f)}: {e}" for f, e in errors)
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully resized:[/green] {resized_count}\n"
                        f"[yellow]Copied (WebP):[/yellow] {copied_count}\n"
                        f"[red]Failed:[/red] {failed}\n\n"
                        f"[red]Failed files:[/red]\n{fail_list}"
                    )
                    self.console.print(Panel.fit(summary, border_style="red"))
                else:
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully resized/copied:[/green] {total}\n"
                        f"[red]Failed:[/red] 0"
                    )
                    self.console.print(Panel.fit(summary, border_style="green"))
            else:
                file_path, output_path, action = files_to_convert[0]
                try:
                    dir_name = os.path.dirname(output_path)
                    if dir_name:
                        os.makedirs(dir_name, exist_ok=True)
                    if action == 'copy':
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
                converted_count = 0
                copied_count = 0
                with _create_progress_bar(len(files_to_convert)) as pbar:
                    max_workers = min(8, os.cpu_count() or 4)
                    with ThreadPoolExecutor(max_workers=max_workers) as executor:
                        futures = {executor.submit(process_single_convert, item): item for item in files_to_convert}
                        for future in as_completed(futures):
                            action_type, file_path, output_path, status = future.result()
                            if status is not True:
                                errors.append((file_path, status))
                            else:
                                if action_type == 'copy':
                                    copied_count += 1
                                else:
                                    converted_count += 1
                            pbar.update(1)
                
                total = len(files_to_convert)
                failed = len(errors)
                if errors:
                    fail_list = "\n".join(f"{os.path.basename(f)}: {e}" for f, e in errors)
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully converted:[/green] {converted_count}\n"
                        f"[yellow]Copied (WebP):[/yellow] {copied_count}\n"
                        f"[red]Failed conversions:[/red] {failed}\n\n"
                        f"[red]Failed files:[/red]\n{fail_list}"
                    )
                    self.console.print(Panel.fit(summary, border_style="red"))
                else:
                    summary = (
                        f"[yellow]Processed:[/yellow] {total}\n"
                        f"[green]Successfully converted:[/green] {converted_count}\n"
                        f"[yellow]Copied (WebP):[/yellow] {copied_count}\n"
                        f"[red]Failed:[/red] 0"
                    )
                    self.console.print(Panel.fit(summary, border_style="green"))
            else:
                file_path, output_path, action = files_to_convert[0]
                try:
                    dir_name = os.path.dirname(output_path)
                    if dir_name:
                        os.makedirs(dir_name, exist_ok=True)
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
                            force=True,
                            quality=quality,
                            lossless=lossless,
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
        if not inputs:
            return False
        return all(os.path.exists(p) for p in inputs)

    def _get_image_files_from_dir(self, directory, output_dir=None, input_root=None):
        """
        Recursively yield (input_file, output_file, is_webp) tuples, preserving folder structure.
        If file is .webp, is_webp=True; else, is_webp=False.
        """
        if input_root is None:
            input_root = directory
        supported_exts = tuple(Image.registered_extensions().keys())
        for root, _, files in os.walk(directory):
            for file in files:
                file_lower = file.lower()
                if not file_lower.endswith(supported_exts):
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
    if len(sys.argv) > 1 and sys.argv[1] in ("--help", "-h"):
        print("webp-convert - A simple CLI tool to convert images to WebP format")
        print("\nUsage:")
        print("  webp-convert          Launch interactive TUI menu")
        print("  webp-convert --help   Show this help message")
        sys.exit(0)

    try:
        cli = WebPConverterCLI()
        cli.show_welcome()
        cli.main_menu()
    except KeyboardInterrupt:
        print("\n👋 Goodbye!")
        sys.exit(0)


if __name__ == "__main__":
    main()
