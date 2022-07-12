use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct RequestCli {
    #[clap(long = "bookmark-as", value_parser)]
    pub save_bookmark_as: Option<String>,

    #[clap(long = "bookmark", action)]
    pub save_bookmark: bool,

    /// Provide an `.env` or a yaml containing template variables
    #[clap(long = "env-file", value_parser, default_value = ".env")]
    pub env_file: PathBuf,

    /// Define a adhoc template variable like `--define foo=value --define bar=42`, see also `--env-file` for more convenience
    #[clap(long, short, number_of_values = 1, value_parser)]
    pub define: Vec<String>,

    #[clap(short = 'X', long = "request", value_parser, default_value = "GET")]
    pub http_method: String,

    /// set one ore more http headers in the form of `"Header-Name: Value"`
    ///
    /// ## Examples
    /// ```
    /// curlz -H "X-First-Name: Joe" https://example.com
    /// curlz -H "User-Agent: yes-please/2000" https://example.com
    /// curlz -H "Host:" https://example.com
    /// ```
    #[clap(long = "header", short = 'H', number_of_values = 1, value_parser)]
    pub headers: Vec<String>,

    #[clap(long = "json", action)]
    pub json: bool,

    #[clap(value_parser)]
    pub bookmark: Option<String>,

    #[clap(value_parser, last = true, multiple = true)]
    pub raw: Vec<String>,
}
