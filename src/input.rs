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

    /// Example value
    #[structopt(long, short, default_value="value")]
    pub value: String,
}
