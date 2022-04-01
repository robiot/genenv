use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "genenv - Generate .env.example")]
pub struct Opt {
    /// Path to the directory
    #[structopt(default_value=".")]
    pub path: String,

    /// Recursivly go through subfolders to find .env files
    #[structopt(long, short)]
    pub recursive: bool,

    /// Folders to exclude for recursive
    #[structopt(long, short, min_values=1)]
    pub path_exclude: Vec<String>,

    /// Example value
    #[structopt(long, short, default_value="value")]
    pub value: String,
}
