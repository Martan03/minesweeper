use serde::{Deserialize, Serialize};
use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

use crate::error::Error;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
    Custom {
        width: usize,
        height: usize,
        mines: usize,
    },
}

#[derive(Debug, Default)]
pub struct Args {
    pub diff: Option<Difficulty>,
    pub help: bool,
}

impl Args {
    /// Parses given args and checks contraints
    /// ### Returns:
    /// - Constructed [`Args`]
    pub fn parse(args: std::env::Args) -> Result<Self, Error> {
        let mut parsed = Args::default();
        let args_len = args.len();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-d" | "--diff" => parsed.parse_diff(&mut args_iter)?,
                "-c" | "--custom" => parsed.parse_custom(&mut args_iter)?,
                "-h" | "--help" => {
                    parsed.help = true;
                    Args::help(args_len)?;
                    return Ok(parsed);
                }
                _ => return Err("Unknown argument given".into()),
            }
        }

        Ok(parsed)
    }
}

// Private methods implementations
impl Args {
    /// Parses difficulty args
    fn parse_diff<T>(&mut self, args: &mut T) -> Result<(), String>
    where
        T: Iterator<Item = String>,
    {
        let Some(diff) = args.next() else {
            return Err("Expected argument after '-d' flag".to_string());
        };

        self.diff = match diff.as_str() {
            "easy" => Some(Difficulty::Easy),
            "medium" => Some(Difficulty::Medium),
            "hard" => Some(Difficulty::Hard),
            _ => return Err(format!("Invalid difficulty: '{}'", diff)),
        };
        Ok(())
    }

    /// Parses custom difficulty args
    fn parse_custom<T>(&mut self, args: &mut T) -> Result<(), String>
    where
        T: Iterator<Item = String>,
    {
        let width = Args::get_num(args)?;
        let height = Args::get_num(args)?;
        let mines = Args::get_num(args)?;

        self.diff = Some(Difficulty::Custom {
            width,
            height,
            mines,
        });
        Ok(())
    }

    /// Gets number (usize) from args
    fn get_num<T>(args: &mut T) -> Result<usize, String>
    where
        T: Iterator<Item = String>,
    {
        let Some(val) = args.next() else {
            return Err("Expected more arguments".to_string());
        };

        val.parse::<usize>()
            .map_err(|_| format!("Number expected, got '{val}'"))
    }

    /// Displays help
    fn help(arg_len: usize) -> Result<(), String> {
        if arg_len > 2 {
            return Err(
                "Help cannot be combined with other arguments".to_string()
            );
        }

        println!(
            "Welcome to help for {} by {}\n",
            "minesweeper".fg(Fg::Green),
            Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "minesweeper" => "Opens TUI difficulty picker\n"
            "minesweeper" ["flags"] => "Start the game with given options\n"
            "Flags":
            "-d --diff" ["easy|medium|hard"] => "Sets the game difficulty\n"
            "-c --custom" ["width"] ["height"] ["mines"] =>
                "Creates custom game with given size and amount of mines\n"
            "-h --help" => "Displays this help"
        );
        Ok(())
    }
}
