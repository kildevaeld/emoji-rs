use curl::easy::Easy;
use phf_codegen;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

#[derive(Deserialize)]
struct Emoji {
    //no: i32,
    //category: String,
    emoji: String,
    description: String,
    //tags: Vec<String>,
    aliases: Vec<String>,
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("emojis.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let dst = {
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        //let url = "https://raw.githubusercontent.com/CodeFreezr/emojo/master/db/v5/emoji-v5.json";
        easy.url("https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json")
            .unwrap();

        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        dst
    };

    let emojis: Vec<Emoji> = serde_json::from_slice(&dst).unwrap();

    write!(
        &mut file,
        "/// Compile time generated lookup table for Emojis.\n///\n"
    )
    .unwrap();

    let mut m = phf_codegen::Map::new();

    let mut found = Vec::default();

    let emojis = emojis.into_iter().map(|mut m| {
        m.aliases = m.aliases.iter().map(|m| format!(":{}:",m)).collect();
        m
    }).collect::<Vec<_>>();



    for emoji in emojis.iter() {
        for alias in &emoji.aliases {
            if found.contains(&alias) {
                continue;
            }

          
            let out = format!("({:?}, {:?})", emoji.description, emoji.emoji);
            found.push(&alias);
            m.entry(alias.as_str(), out.as_str());
           
        }
    }

    write!(
        &mut file,
        "static EMOJIS: phf::Map<&'static str, (&'static str, &'static str)> = {};\n",
        m.build()
    )
    .unwrap();

}
