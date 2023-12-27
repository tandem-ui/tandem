use anyhow::Result;
use clap::Args;
use paperclip_ast_serialize::pc::serialize as serialize_pc;
use paperclip_common::fs::LocalFileReader;
use paperclip_config::{ConfigContext, DEFAULT_CONFIG_NAME};
use paperclip_project::{LocalIO, Project};
use std::io::prelude::*;
use std::{env, fs::File};

#[derive(Debug, Args)]
pub struct FmtArgs {
    /// Prints the formatted output instead
    #[clap(short, long, value_parser, default_value_t = false)]
    print: bool,

    /// The config file to use for compiling
    #[clap(short, long, default_value_t = String::from(DEFAULT_CONFIG_NAME))]
    config: String,
}

pub async fn fmt(args: FmtArgs) -> Result<()> {
    let current_dir = String::from(env::current_dir()?.to_str().unwrap());
    let config_context =
        ConfigContext::load(&current_dir, Some(args.config), &LocalFileReader::default())?;
    let io = LocalIO::new(config_context.clone());

    let mut project = Project::new(config_context, io);
    project.load_all_files().await?;

    let graph = project.graph.try_lock().unwrap();
    for (path, dep) in &graph.dependencies {
        println!("✍🏻  {}", path.replace(&format!("{}/", current_dir), ""));

        if let Some(document) = &dep.document {
            let content = serialize_pc(document);
            if args.print {
                println!("{}", content);
            } else {
                let mut file = File::create(path)?;
                file.write_all(content.as_str().as_bytes())?;
            }
        }
    }
    Ok(())
}
