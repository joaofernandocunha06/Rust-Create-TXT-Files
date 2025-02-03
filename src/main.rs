#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

fn main() -> std::io::Result<()> {
    println!("Qual é o nome desejado?");
    let username = whoami::username();
    let mut input_name = String::new();
    stdin().read_line(&mut input_name).expect("ERRO!");
    let path = format!("C:/Users/{}/Desktop/DIRECT/{}.txt", username, input_name.trim());

    let mut file = File::create(path)?;

    let mut input_mes = String::new();
    println!("Qual é a mensagem desejada?");
    stdin().read_line(&mut input_mes).expect("ERRO!");

    file.write_all(input_mes.as_bytes())?;
    Ok(())
}