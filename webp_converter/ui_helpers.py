from rich.console import Console
from rich.panel import Panel
from rich.table import Table
from rich import box
from rich.prompt import Confirm

console = Console()

def show_success(input_path, output_path, original_size, new_size, quality):
    """Show a beautiful success panel after conversion."""
    size_diff = original_size - new_size
    size_percent = (size_diff / original_size) * 100 if original_size > 0 else 0

    def format_size(size_bytes):
        if size_bytes < 1024:
            return f"{size_bytes} B"
        elif size_bytes < 1024 * 1024:
            return f"{size_bytes/1024:.1f} KB"
        else:
            return f"{size_bytes/(1024*1024):.2f} MB"

    table = Table(box=box.SIMPLE, show_header=False, padding=(0, 1))
    table.add_column("Property", style="cyan")
    table.add_column("Value", style="green")
    table.add_row("Original", input_path)
    table.add_row("WebP", output_path)
    table.add_row("Original Size", format_size(original_size))
    table.add_row("WebP Size", format_size(new_size))
    table.add_row("Saved", f"{format_size(size_diff)} ({size_percent:.1f}%)")
    table.add_row("Quality", f"{quality}%")
    console.print(
        Panel(
            table,
            title="[bold green]✓ Conversion Successful![/bold green]",
            border_style="green",
            expand=False
        )
    )

def show_error(message, title="Error"):
    console.print(
        Panel(
            f"[red]{message}[/red]",
            title=f"[bold red]❌ {title}[/bold red]",
            border_style="red",
            expand=False
        )
    )

def show_warning(message, title="Warning"):
    console.print(
        Panel(
            f"[yellow]{message}[/yellow]",
            title=f"[bold yellow]⚠️ {title}[/bold yellow]",
            border_style="yellow",
            expand=False
        )
    )

def show_info(message, title="Info"):
    console.print(
        Panel(
            message,
            title=f"[bold blue]ℹ️ {title}[/bold blue]",
            border_style="blue",
            expand=False
        )
    )

def ask_overwrite(filename):
    show_warning(f"Output file '{filename}' already exists.", title="File Exists")
    return Confirm.ask("Overwrite this file?", default=False)
