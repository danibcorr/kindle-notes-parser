// Esto es para soportar lectura de argumentos
use std::env;
use std::fs;

fn help() {
    println!(
        "\nAyuda de la linea de comandos para el parseador de notas de Kindle.\n\
        \nComandos disponibles:\n\n\
        \t--parser o -p <path>: Realizar el parseo del documento introducido.\n"
    );
}

fn obtener_titulo_principal(path_notas: &str) -> String {
    // La referencia (&) no significa necesariamente modificación, sino préstamo (borrowing).
    // En Rust, una referencia &str es inmutable.

    let contenido = fs::read_to_string(path_notas).expect("Error al leer el documento.");
    contenido
        .lines()
        .next()
        .expect("Error, el fichero está vacio.")
        .to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("No se ha introducido ningún argumento."),
        2 => match args[1].to_lowercase().as_str() {
            "--help" | "-h" => help(),
            "--parser" | "-p" => {
                println!("Te ha faltado introducir el path del fichero.")
            }
            _ => println!("Comando no disponible, utiliza --help o -h."),
        },
        3 => {
            let comando = &args[1];
            let valor_comando = &args[2];

            match comando.to_lowercase().as_str() {
                "--parser" | "-p" => {
                    println!(
                        "El primer titulo del documento es: {}",
                        obtener_titulo_principal(&valor_comando)
                    );
                }
                _ => println!("Comando no disponible, utiliza --help o -h."),
            }
        }
        _ => println!("Comando no disponible, utiliza --help o -h."),
    }
}
