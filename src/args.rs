use clap::{Parser, Subcommand, Args, ValueEnum};

/// Перечисление для строгой валидации повторов
#[derive(ValueEnum, Clone, Debug)]
pub enum RepeatMode {
    Yes,
    No,
}

#[derive(Parser)]
#[command(
    name = "passwdh",
    about = "Password generation and checking utility",
    override_usage = "passwdh [COMMAND] [OPTION]... [PARAMETER]... <password>",
    // help_template = "{about-section}\nUsage: {usage}\n\n{all-args} {tab}\n\n{after-help}",
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = "[OTHER OPTIONS]\n  -e, --length\tSet length\n  -r, --repeat\tSet characters repeat\n  -p, --parameters\tSet [PARAMETERS]\n\n[EXAMPLES]\n    passwdh generate -p lbn -e 12\n    passwdh check -p lcsn myPass123!\n\n[PARAMETERS]\nFormat [lcbns]:\tWARNING!  If not set the default combination is <lbns>\n  (l)atin,\n  (c)yrillic,\n  (b)ig,\n  (n)umbers,\n  (s)pecial"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version, help = "Print version")]
    pub version: Option<bool>,
}

#[derive(Args, Clone)]
pub struct PasswordOptions {
    #[arg(
        global = true,
        short = 'e', 
        long = "length", 
        value_name = "n",
        help = "Set password length",
        help_heading = "OPTIONS",
    )]
    pub length: Option<usize>,

    #[arg(
        global = true,
        short = 'r', 
        long = "repeat", 
        value_name = "yes/no",
        help = "Enable symbols repetition (Default: no repeat)",
        help_heading = "OPTIONS",
    )]
    pub repeat: Option<RepeatMode>,

    #[arg(
        global = true,
        help_heading = "PARAMETERS [lcbns]",
        short = 'p',
        long = "parameters",
        value_name = "lcbns",
        help = "Format [lcbns]: (l)atin, (c)yrillic, (b)ig, (n)umbers, (s)pecial"
    )]
    pub parameters: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Generate a new password")]
    Generate(PasswordOptions),
    
    #[command(about = "Check an existing password against rules")]
    Check {
        #[arg(help = "The password to check")]
        password: String,
        
        #[command(flatten)]
        options: PasswordOptions,
    },
}