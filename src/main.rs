use serde_json;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;

struct Commands{
    init: String,
    newlist: String,
    newframe: String,
    delete: String,
    cd: String,
}
impl Commands {
    fn new() -> Commands {
        Commands { 
            init: String::from("init"), 
            newlist: String::from("new-list"), 
            newframe: String::from("new-frame"),
            delete: String::from("delete"), 
            cd: String::from("cd") 
        }
    } 
}
fn main() {
    let commands: Commands = Commands::new();
    let args: Vec<String> = env::args().collect();
    
    let pf = env::var("USERPROFILE").unwrap();
    let pf = Path::new(&pf);
    let mut default_path = pf.join(".Lists");
    default_path.push("base");

    // Destructure Struct for matching
    let Commands {init, newlist, newframe, delete, cd} = commands;
    
    match args.len() {
        2 => {
            match &args[1] {
                cmd if cmd == &init => {
                    println!("INIT");
                    // New Frame
                    cla_init();
                },
                _ => {panic!()}
            }
        }
        3 => {
            match &args[1] {
                cmd if cmd == &newlist => {
                    println!("NEW");
                    // New List
                    cla_new(&args[2]);
                },
                cmd if cmd == &newframe => {
                    println!("NEW");
                    // New List
                    cla_new(&args[2]);
                },
                cmd if cmd == &delete => {
                    println!("CHANGE");
                    // Delete list/frame/anything
                    cla_delete(&args[2]);
                },
                cmd if cmd == &cd => {
                    println!("CD");
                    // Change directory
                    cla_cd(&args[2]);
                }
                _ => {panic!()}
            }
        }
        _ => {
            panic!(); // Placeholder for now
        }
    }
}

fn cla_init() {
    let pf = env::var("USERPROFILE").unwrap();
    let pf = Path::new(&pf);
    let mut root = pf.join(".Lists");
    root.push("base");
    fs::create_dir_all(&root).unwrap();

}

fn cla_new(path: &PathBuf, name: &String) {
    let newpath = path.join(name);
    fs::create_dir(newpath).unwrap();
    //todo!()
}

fn cla_delete(list: &String) {
    todo!()
}

fn cla_cd(dir: &String) {
    todo!()
}