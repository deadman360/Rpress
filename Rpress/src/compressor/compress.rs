use std::{fs::File, io::{Read, Write}, path::Path};

use zip::{result::ZipResult, write::FileOptions, ZipWriter};


pub fn compress_file(input_file: &str) -> ZipResult<()>{
    let output_file: &str = &format!("{}.zip",
     Path::new(input_file)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(input_file));

    let zip_file = File::create(output_file).expect("Erro ao criar o arquivo que receberá a compressão");

    let mut zip_writer = ZipWriter::new(zip_file);

    let options: zip::write::FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    if cfg!(unix){
        options.unix_permissions(0o755);
    };
    let mut buffer = Vec::new();
    let mut arquivo = File::open(input_file)
        .expect("Nao foi possivel ler o arquivo para o zip");
    arquivo.read_to_end(&mut buffer);

        zip_writer.start_file(input_file, options).expect("Erro ao inicializar o ZIP");
    zip_writer.write_all(&buffer).expect("Erro ao escrever o arquivo no ZIP");
    zip_writer.finish()?;
    Ok(())
    
}