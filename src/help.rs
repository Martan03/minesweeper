use termint::{
    enums::Color,
    help,
    widgets::{Grad, ToSpan},
};

/// Displays help
pub fn print_help() {
    println!(
        "Welcome to help for {} by {}\n",
        "minesweeper".fg(Color::Green),
        Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
    );
    help!(
        "Usage":
        "minesweeper" =>
            "Starts game with the default dificulity or opens TUI difficulty \
            picker.\n"
        "minesweeper config" =>
            "Opens the configuration file in the default editor.\n"
        "minesweeper help" => "Prints this help.\n"
        "minesweeper" ["flags"] => "Start the game with given options.\n"
        "Flags":
        "-d --diff --difficulty" ["easy|medium|hard"] =>
            "Sets the game difficulty.\n"
        "-c --custom" ["width"] ["height"] ["mines"] =>
            "Creates custom game with given size and amount of mines.\n"
        "-h --help" => "Displays this help."
    );
}
