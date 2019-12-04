extern crate humantime;
#[macro_use]
extern crate clap;
use std::fs::{create_dir_all, copy};
use std::path::{PathBuf, Path};


fn save_files<E:AsRef<Path>, I:Iterator<Item=E>, T:AsRef<Path>>(list:I, to:T){
    let mut list:Box<dyn Iterator<Item=E>> = Box::new(list);
    let max = to.as_ref().read_dir().ok().map(|x|
        x.flat_map(Result::ok)
        .map(|x| x.path())
        .filter(|x| x.is_dir())
        .flat_map(
            |x| x.file_name()
                .map(OsStr::to_str)
                .flatten()
                .map(String::from)
        )
        .map(|x| humantime::parse_rfc3339(&x.as_str().replace("#",":")))
        .flat_map(Result::ok)
        .max()
    ).flatten();

    if let Some(value) = max{
        list = Box::new(list.filter(move |x| value < x.as_ref().metadata().unwrap().modified().unwrap()))
    }

    list.for_each(move |file|{
        let file = file.as_ref();
        let date = file.metadata().unwrap().modified().unwrap();
        let date_string = format!("{}", humantime::Timestamp::from(date)).replace(":","#");
        let to_path_folder = to.as_ref().join(date_string);
        if !to_path_folder.exists() {
            println!("Creating folder {:?}", to_path_folder);
            create_dir_all(&to_path_folder).expect(
                format!("Failed to created folder {:?}", to_path_folder).as_str()
            );
        }
        let to_file = to_path_folder.join(file.file_name().unwrap());
        println!("Copying file {:?} to {:?}", file, to_file);
        copy(file, &to_file).expect(
            format!("Failed to copy file {:?} to {:?}", file, to_file).as_str()
        );
    })
}

fn ironmanfilter<I:Iterator<Item = PathBuf>>(list:I) -> impl Iterator<Item = PathBuf>{
    let suffix = "_Backup";
    let ending = format!("{}.eu4",suffix);
    list.flat_map(move |x|
        if x.to_str().map(|x|x.ends_with(&ending)).unwrap_or(false) {
            x.to_str()
                .map(|y|y.replace(suffix, ""))
                .map(|y| vec![x, y.into()])
                .unwrap_or_default()
        } else {
            Default::default()
        }.into_iter()
    )
}

use std::ffi::OsStr;

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
    let mut list:Box<dyn Iterator<Item=PathBuf>> = Box::new(
        from.read_dir().expect(
            format!("Failed to read folder {:?}", from).as_str()
        ).flat_map(Result::ok).map(|x| x.path().to_path_buf())
    );
    if matches.is_present("CONFIG") {
        list = Box::new(ironmanfilter(list))
    }
    save_files(list, to);
}
