extern crate humantime;
#[macro_use]
extern crate clap;
use std::fs::{read_dir, create_dir_all, copy, DirEntry};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::{PathBuf, Path};


fn save_files<E:AsRef<Path>, I:Iterator<Item=E>, T:AsRef<Path>>(list:I, to:T){
    list.for_each(move |file|{
        let file = file.as_ref();
        let date = file.metadata().unwrap().modified().unwrap();
        let date_string = format!("{}", humantime::Timestamp::from(date));
        let to_path_folder = to.as_ref().join(date_string);
        if !to_path_folder.exists() {
            create_dir_all(&to_path_folder).unwrap();
        }
        let to_file = to_path_folder.join(file.file_name().unwrap());
        copy(file, to_file).unwrap();
    })
}

fn ironmanfilter<I:Iterator<Item = PathBuf>>(list:I) -> impl Iterator<Item = PathBuf>{
    let suffix = "_Backup";
    let ending = format!("{}.eu4",suffix);
    list.flat_map(move |x|
        if x.ends_with(&ending) {
            x.to_str()
                .map(|y|y.replace(suffix, ""))
                .map(|y| vec![x, y.into()])
                .unwrap_or_default()
        } else {
            Default::default()
        }.into_iter()
    )
}

use std::env;
fn main() {
    let matches = clap_app!(saver=>
        (version: "0.1")
        (author: "Jonas Ingerslev Sørensen. <jonas.jonas.srensen11@gmail.com>")
        (about: "Saves files into a directory separated by modification date")
        (@arg CONFIG: -i --ironman "Sets the saver to only save ironman marked files.")
        (@arg INPUT: +required "Sets the input folder to save")
        (@arg OUTPUT: +required "Sets the output folder to save")
    ).get_matches();

    let from = Path::new(matches.value_of("INPUT").unwrap());
    let to = Path::new(matches.value_of("OUTPUT").unwrap());
    let mut list:Box<dyn Iterator<Item=PathBuf>> = Box::new(from.read_dir().unwrap().flat_map(|x| x.map(|x| x.path().to_path_buf())));

    if matches.is_present("CONFIG") {
        list = Box::new(ironmanfilter(list))
    }

    save_files(list, to);
}
