use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::{ read };
use std::path::Path;

use rayon::prelude::*;
use colour::*;
use crc::{crc32};


fn main() {
    // Carrega os argumentos enviados por linha de comando
    let args: Vec<String> = env::args().collect();
    let lista_arquivos = args[1..args.len()].to_vec();
    
    green_ln!("Lendo arquivos..\n");

    lista_arquivos.par_iter()
            .for_each(|caminho_arquivo| {
                let nome_arquivo = Path::new(&caminho_arquivo).file_name().unwrap().to_str().unwrap();
                let arquivo = read(&caminho_arquivo).expect("Não foi possível ler o arquivo.");
                let crc = crc32::checksum_ieee(&arquivo);

                white_ln!("\nArquivo: {}\nCRC32:{:X}", nome_arquivo, crc);
            });
    pause();
}

fn pause() {
    let mut stdin = io::stdin();

    blue_ln!("\n\nFim.");

    // Queremos que o cursor fique no final da linha, então imprimimos sem uma linha nova
    blue_ln!("Aperte enter para encerrar...");

    // Lê um único byte e descarta
    let _ = stdin.read(&mut [0u8]).unwrap();
}
