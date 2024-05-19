use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use sha2::{Sha512,Digest};
use md5;
use std::process::exit;

fn main(){
    let args : Vec<String> = env::args().collect();
    if  args.len() !=3{
        println!("Invalid amount of arguments!");
        println!("Example: cargo run <hash> <path for password list file>");
        exit(1);
    }    
    fn sha_256(args : Vec<String>){
        let wanted_hash: &String = &args[1];
        let pass_file :&str = &args[2];
        let mut attempts = 1;
        println!("Attempting to crack: {}\n",wanted_hash);
    
        let password_list = File::open(pass_file).unwrap();
        let reader = BufReader::new(password_list);

        for line in reader.lines(){
            let line = line.unwrap();
            let password:Vec<u8> = line.trim().to_owned().into_bytes();
    
            let password_hash:String = format!("{:x}",Sha512::digest(&password));
    
            println!("[{}] {} == {}",attempts, std::str::from_utf8(&password).unwrap(), password_hash);
    
            if &password_hash == wanted_hash{
                println!("Password Found! {:?}",std::str::from_utf8(&password).unwrap());
                exit(0);
    
            }
            attempts += 1;
        }
        println!("Password not fond!");
    }
    fn md_5(args: Vec<String>){
        let wanted_hash: &String = &args[1];
        let pass_file :&str = &args[2];
        let mut attempts = 1;
        println!("Attempting to crack: {}\n",wanted_hash);
    
        let password_list = File::open(pass_file).unwrap();
        let reader = BufReader::new(password_list);

        for line in reader.lines(){
            let line = line.unwrap();
            let password:Vec<u8> = line.trim().to_owned().into_bytes();
    
            let password_hash:String = format!("{:x}",md5::compute(&password));
    
            println!("[{}] {} == {}",attempts, std::str::from_utf8(&password).unwrap(), password_hash);
    
            if &password_hash == wanted_hash{
                println!("Password Found! {:?}",std::str::from_utf8(&password).unwrap());
                exit(0);
    
            }
            attempts += 1;
        }
        println!("Password not fond!");
    }
    println!("Choose Algorithms : \n 1 -> SHA256 \n 2 -> MD5 \n Enter 1 for SHA256 \n Enter 2 for md5 ");
    let mut choose =  String::new();
    let _ = io::stdin().read_line(&mut choose);

    let character = choose.trim().chars().next();


    println!("{}",choose);

    if character == Some('1') {
        sha_256(args);
    }
    else if character == Some('2') {
        md_5(args);
    }

    else{
        eprintln!("Enter Valid Option") ; 
    } 

}