use clap::{Parser, Subcommand, Args};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "Httpie")]
#[command(author = "Yin Zhang. <miracleyin@live.com>")]
#[command(version = "1.0")]
#[command(about = "Http tool", long_about = None)]
#[command(propagate_version = true)]
struct Opts {
    #[command(subcommand)]
    subcom: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post)
}

// get
#[derive(Args, Debug)]
struct Get {
    url: String
}

// post
#[derive(Args, Debug)]
struct Post {
    url: String,
    body: Vec<String>
}




fn main() {
    let opts = Opts::parse();

    println!("{:?}", opts);


}