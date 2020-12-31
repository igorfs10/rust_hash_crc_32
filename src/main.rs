use std::env;
use std::fs::read;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use console::style;
use crc::{Crc, CRC_32_ISO_HDLC};
use rayon::prelude::*;

fn main() {
    // Carrega os argumentos enviados por linha de comando
    let args: Vec<String> = env::args().collect();
    let lista_arquivos = args[1..args.len()].to_vec();

    println!("{}", style("Lendo arquivos..\n").green());

    lista_arquivos.par_iter().for_each(|caminho_arquivo| {
        let nome_arquivo = Path::new(&caminho_arquivo)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let arquivo = read(&caminho_arquivo).expect("Não foi possível ler o arquivo.");
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let checksum = crc.checksum(&arquivo);

        println!(
            "{}",
            style(format!("\nArquivo: {}\nCRC32:{:X}", nome_arquivo, checksum)).white()
        );
    });
    pause();
}

fn pause() {
    let mut stdin = io::stdin();

    println!("{}", style("\n\nFim.").blue());

    // Queremos que o cursor fique no final da linha, então imprimimos sem uma linha nova
    println!("{}", style("Aperte enter para encerrar...").blue());

    // Lê um único byte e descarta
    let _ = stdin.read(&mut [0u8]).unwrap();
}
