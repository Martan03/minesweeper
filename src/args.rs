use pareg::{ArgErrCtx, ArgError, FromArg, Pareg};
use serde::{Deserialize, Serialize};

use crate::error::Result;

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

#[derive(Debug, Clone, Copy, Default)]
pub enum Action {
    #[default]
    Play,
    Help,
    Config,
}

#[derive(Debug, Default)]
pub struct Args {
    pub diff: Option<Difficulty>,
    pub action: Action,
}

impl Args {
    /// Parses given args and checks contraints
    /// ### Returns:
    /// - Constructed [`Args`]
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut parsed = Args::default();
        while let Some(arg) = args.next() {
            match arg {
                "-d" | "--diff" | "--difficulty" => {
                    parsed.diff = Some(args.next_arg()?)
                }
                "-c" | "--custom" => {
                    parsed.diff = Some(Difficulty::Custom {
                        width: args.next_arg()?,
                        height: args.next_arg()?,
                        mines: args.next_arg()?,
                    })
                }
                "config" => parsed.action = Action::Config,
                "-h" | "--help" | "help" => parsed.action = Action::Help,
                _ => return Err(args.err_unknown_argument().into()),
            }
        }

        Ok(parsed)
    }
}

impl<'a> FromArg<'a> for Difficulty {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        match arg {
            "easy" => Ok(Self::Easy),
            "medium" => Ok(Self::Medium),
            "hard" => Ok(Self::Hard),
            v => Err(ArgError::FailedToParse(Box::new(
                ArgErrCtx::from_msg(
                    format!("Invalid difficulty `{v}`").into(),
                    v.to_string(),
                )
                .hint("Valid options are `easy`, `medium` or `hard`"),
            ))),
        }
    }
}
