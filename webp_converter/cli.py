import sys
import os
from pathlib import Path
from PIL import Image
from rich.console import Console
from rich.panel import Panel
from rich.progress import Progress, SpinnerColumn, TextColumn, BarColumn
from tqdm import tqdm
import questionary
from questionary import Style

# ANSI color codes for retro terminal style
CYAN = "\033[96m"
MAGENTA = "\033[95m"
YELLOW = "\033[93m"
GREEN = "\033[92m"
RED = "\033[91m"
RESET = "\033[0m"
BOLD = "\033[1m"


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
        import ctypes, sys
        from pathlib import Path

        try:
            # Windows 7+ Downloads folder
            from ctypes import windll, wintypes, byref

            CSIDL_PERSONAL = 0x0005  # My Documents
            SHGFP_TYPE_CURRENT = 0  # Get current, not default value
            buf = ctypes.create_unicode_buffer(wintypes.MAX_PATH)
            windll.shell32.SHGetFolderPathW(
                None, CSIDL_PERSONAL, None, SHGFP_TYPE_CURRENT, buf
            )
            doc_path = Path(buf.value)
            downloads = doc_path.parent / "Downloads"
            return str(downloads)
        except Exception:
            # fallback
            return str(Path.home() / "Downloads")
    else:
        return os.path.join(os.path.expanduser("~"), "Downloads")


from .ui_helpers import show_success, show_error, show_warning, show_info, ask_overwrite


def convert_to_webp(
    input_path: str, output_path: str = None, force: bool = False, quality: int = 80
) -> bool:
    """
    Convert an image to WebP format.
    Returns True on success, False on error.
    """
    if not os.path.isfile(input_path):
        show_error(f"Input file '{input_path}' does not exist.")
        return False
    try:
        with Image.open(input_path) as img:
            img = img.convert("RGBA") if img.mode in ("P", "LA") else img.convert("RGB")
            if not output_path:
                base = os.path.splitext(os.path.basename(input_path))[0]
                output_path = os.path.join(get_downloads_dir(), base + ".webp")
            if os.path.exists(output_path) and not force:
                if not ask_overwrite(os.path.basename(output_path)):
                    show_info("Conversion skipped by user.", title="Skipped")
                    return False
            img.save(output_path, "WEBP", quality=quality)
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


def retro_input(prompt_msg, default=None):
    # Beautiful retro border and prompt
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
    # Retro input arrow
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
    """Create a tqdm progress bar for image conversion."""
    return tqdm(
        total=total_images,
        unit="img",
        desc="Converting",
        bar_format="{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt} [{elapsed}<{remaining}]",
    )


CUSTOM_STYLE = Style(
    [
        ("qmark", "fg:#00d7af bold"),  # Question mark
        ("question", "bold"),  # Question text
        ("answer", "fg:#ffaf00 bold"),  # User's answer
        ("pointer", "fg:#00d7af bold"),  # Pointer arrow
        ("highlighted", "fg:#00d7af bold"),  # Highlighted choice
        ("selected", "fg:#5f87ff bold"),  # Selected choice
        ("instruction", "fg:#888888 italic"),  # Instruction text
    ]
)

RETRO_ASCII = """

██╗    ██╗███████╗██████╗ ██████╗        ██████╗ ██████╗ ███╗   ██╗██╗   ██╗███████╗██████╗ ████████╗
██║    ██║██╔════╝██╔══██╗██╔══██╗      ██╔════╝██╔═══██╗████╗  ██║██║   ██║██╔════╝██╔══██╗╚══██╔══╝
██║ █╗ ██║█████╗  ██████╔╝██████╔╝█████╗██║     ██║   ██║██╔██╗ ██║██║   ██║█████╗  ██████╔╝   ██║   
██║███╗██║██╔══╝  ██╔══██╗██╔═══╝ ╚════╝██║     ██║   ██║██║╚██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗   ██║   
╚███╔███╔╝███████╗██████╔╝██║           ╚██████╗╚██████╔╝██║ ╚████║ ╚████╔╝ ███████╗██║  ██║   ██║   
 ╚══╝╚══╝ ╚══════╝╚═════╝ ╚═╝            ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝   ╚═╝   
                                                                                                     
"""


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
        # Prompt for input files/folders
        self.console.print(f"[bold cyan]{RETRO_ASCII}[/bold cyan]")
        input_path = questionary.path(
            "Enter input image file(s) or a folder path (comma-separated for multiple files):",
            qmark="🖼️",
            style=CUSTOM_STYLE,
        ).ask()
        if not input_path:
            self.console.print("[yellow]No input provided. Returning to menu.[/yellow]")
            return
        inputs = self._parse_inputs(input_path)
        if not self._validate_inputs_exist(inputs):
            self.console.print("[red]One or more input paths do not exist.[/red]")
            return

        # Prompt for output directory
        default_dir = get_downloads_dir()
        output_dir = questionary.path(
            f"Enter output directory for all files (default: {default_dir})",
            default=default_dir,
            qmark="📂",
            style=CUSTOM_STYLE,
        ).ask()
        if not output_dir:
            output_dir = default_dir
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)

        # Prompt for WebP quality
        quality = questionary.text(
            "WebP quality (0-100, default: 80):",
            default="80",
            validate=lambda val: val.isdigit() and 0 <= int(val) <= 100,
            qmark="🎚️",
            style=CUSTOM_STYLE,
        ).ask()
        quality = int(quality or 80)

        # Prompt for overwrite
        force = questionary.confirm(
            "Overwrite output file(s) without prompting?",
            default=False,
            qmark="⚠️",
            style=CUSTOM_STYLE,
        ).ask()

        files_to_convert = list(self._get_image_files(inputs, output_dir))

        # Confirm before proceeding if no files found
        if not files_to_convert:
            self.console.print("[yellow]No images found to convert.[/yellow]")
            return

        # Convert with tqdm progress
        errors = []
        if len(files_to_convert) > 1:
            with _create_progress_bar(len(files_to_convert)) as pbar:
                for file_path, output_path in files_to_convert:
                    try:
                        convert_to_webp(
                            file_path, output_path, force=force, quality=quality
                        )
                    except Exception as e:
                        errors.append((file_path, str(e)))
                    pbar.update(1)
            if errors:
                self.console.print(
                    Panel.fit(
                        f"[red]Some files failed to convert:[/red]\n"
                        + "\n".join(f"{f}: {e}" for f, e in errors),
                        border_style="red",
                    )
                )
            else:
                self.console.print(
                    Panel.fit(
                        "[green]All images converted successfully![/green]",
                        border_style="green",
                    )
                )
        else:
            file_path, output_path = files_to_convert[0]
            try:
                convert_to_webp(file_path, output_path, force=force, quality=quality)
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
