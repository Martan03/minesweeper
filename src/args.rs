use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
    Custom(usize, usize, usize),
}

pub struct Args {
    pub diff: Difficulty,
    pub help: bool,
}

impl Args {
    /// Parses given args and checks contraints
    pub fn parse(args: std::env::Args) -> Result<Self, String> {
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
                _ => return Err("Unknown argument given".to_string()),
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
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
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

        self.diff = Difficulty::Custom(width, height, mines);
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
            "minesweeper" => "Starts the game with medium difficulty\n"
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

impl Default for Args {
    fn default() -> Self {
        Self {
            diff: Default::default(),
            help: false,
        }
    }
}
