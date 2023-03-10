use std::collections::HashMap;
use clap::Parser;
use git2::Repository;

/// Vulcanic Labs tooling manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the template
   #[arg(short, long)]
   template: String,

   /// Name of the directory
   #[arg(short, long, default_value="<TEMPLATE NAME>")]
   output: String,
}

fn main() {
   let args = Args::parse();

   let repositories = HashMap::from([
       (String::from("hammer"), String::from("https://github.com/Vulcanic-Labs/hammer")),
       (String::from("template-repo-test"), String::from("https://github.com/Vulcanic-Labs/template-repo-test")),
   ]);

   let url = repositories.get(&args.template).unwrap();

   let mut output_dir = args.template;
   if args.output != "<TEMPLATE NAME>" {
       output_dir = args.output;
   }

   // clone to local
   let repo = match Repository::clone(url, output_dir) {
       Ok(repo) => repo,
       Err(e) => panic!("Failed to clone: {}", e)
   };

   // remove remote
   repo.remote_delete("origin").unwrap();
}
