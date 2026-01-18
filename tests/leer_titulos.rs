// Esto es para leer los argumentos
use std::env;
// Esto es para soportar lectura de argumentos
use std::fs;
// Esto es para crear el HashSet
use std::collections::HashSet;

fn help() {
    println!(
        "\nAyuda de la linea de comandos para el parseador de notas de Kindle.\n\
        \nComandos disponibles:\n\n\
        \t--parser o -p <path>: Realizar el parseo del documento introducido.\n"
    );
}

fn obtener_titulos_libros(path_notas: &str) -> Vec<String> {
    // La referencia (&) no significa necesariamente modificación, sino préstamo (borrowing).
    // En Rust, una referencia &str es inmutable.

    let contenido: String = fs::read_to_string(path_notas).expect("Error al leer el documento.");
    let mut contenido_iterable = contenido.lines();
    
    let primer_titulo = contenido_iterable
        .next()
        .expect("Error, el fichero está vacio.")
        .to_string()
        .replace("\u{feff}", "");

    // Convertimos las líneas en un Vector, esto lo combina todo en un array dinámico
    // donde cada elemento del array es una linea del documento
    let vector_lineas: Vec<&str> = contenido_iterable.collect();
    //println!("{:?}", vector_lineas);
    //println!("{}", vector_lineas.len());

    let mut vector_titulos: Vec<String> = Vec::new();

    for indice_linea in 0..vector_lineas.len() {
        if vector_lineas[indice_linea] == "==========" {
            if indice_linea + 1 < vector_lineas.len() {
                let titulo_libro = vector_lineas[indice_linea + 1]
                    .trim()
                    .replace("\u{feff}", "");
                vector_titulos.push(titulo_libro);
            }
        }
    }

    // Tegno que meter el primer_titulo con el resto
    vector_titulos.push(primer_titulo);

    // Podemos eliminar duplicados creando un HashSet, que es como un hashmap pero en vez
    // de tener almacenados parejas de keys y values pues solo tenemos keys, que seran
    // los titulos de los libros
    let mut vector_titulos_unico: HashSet<String> = HashSet::new();
    vector_titulos.retain(|x| vector_titulos_unico.insert(x.to_string()));
    //println!("{:?}", vector_titulos);

    vector_titulos
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
                        "Títulos de libros encontrados: {:?}",
                        obtener_titulos_libros(&valor_comando)
                    );
                }
                _ => println!("Comando no disponible, utiliza --help o -h."),
            }
        }
        _ => println!("Comando no disponible, utiliza --help o -h."),
    }
}
