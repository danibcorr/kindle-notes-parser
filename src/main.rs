// Esto es para leer los argumentos
use std::env;
// Esto es para soportar lectura de argumentos
use std::fs;
// Esto es para crear el HashSet
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Write;

fn help() {
    println!(
        "\nAyuda de la linea de comandos para el parseador de notas de Kindle.\n\
        \nComandos disponibles:\n\n\
        \t--parser o -p <path>: Realizar el parseo del documento introducido.\n"
    );
}

fn leer_fichero_notas(path_notas: &str) -> String {
    return match fs::read_to_string(path_notas) {
        Ok(contenido) => {
            println!("El contenido del documento se ha leido correctamente.");
            contenido
        }
        Err(e) => {
            panic!("Error al leer el fichero: {}", e);
        }
    };
}

fn limpiar_contenido(contenido: &str) -> String {
    return contenido.trim().to_string().replace("\u{feff}", "");
}

fn obtener_titulos_libros(contenido_notas: &str) -> HashSet<String> {
    // Podemos crear un iterable del documento
    let contenido_iterable = contenido_notas.lines();

    // Podemos ahora utilizar enumerate para que cada linea ahora sea una tupla de
    // su indice y contenido, para posteriormente aplicar un filtrado junto con un mapeo
    // donde si el resto de dividir el valor del indice entre 5 es 0, entonces estoy
    // en la posicion del titulo
    let mut titulos_disponibles: Vec<String> = contenido_iterable
        .enumerate()
        .filter_map(|(indice, contenido)| {
            if indice % 5 == 0 {
                Some(limpiar_contenido(contenido))
            } else {
                None
            }
        })
        .collect();

    // Podemos eliminar duplicados creando un HashSet, que es como un HashMap pero en vez
    // de tener almacenados parejas de keys y values pues solo tenemos keys, que seran
    // los titulos de los libros
    let mut titulos_disponibles_filtrados: HashSet<String> = HashSet::new();
    titulos_disponibles.retain(|titulos| titulos_disponibles_filtrados.insert(titulos.to_string()));
    println!(
        "Títulos de libros encontrados: {:?}",
        titulos_disponibles_filtrados
    );

    return titulos_disponibles_filtrados;
}

fn elegir_titulo(titulos_disponibles: HashSet<String>) -> String {
    // Para crear un mapeo entre cada título disponible y un indice que permita al
    // usuario elegir un titulo, podemos usar un HashMap
    let indice_titulos: HashMap<usize, String> =
        titulos_disponibles.into_iter().enumerate().collect();

    println!("\nIndice de títulos: \n{:?}", indice_titulos);

    println!("\nIntroduce el indice del titulo a seleccionar:");

    loop {
        let mut input_texto: String = String::new();

        io::stdin()
            .read_line(&mut input_texto)
            .expect("Error al leer el input");

        let input_user: usize = match input_texto.trim().parse() {
            Ok(num) => {
                if indice_titulos.contains_key(&num) {
                    num
                } else {
                    println!("El índice no existe, prueba otra vez.");
                    continue;
                }
            }
            Err(_) => {
                println!("Por favor, introduce un número válido.");
                continue;
            }
        };

        let titulo_seleccionado: String = indice_titulos
            .get(&input_user)
            .expect("Error al obtener el título.")
            .to_string();

        println!("Titulo seleccionado: {titulo_seleccionado}");

        return titulo_seleccionado;
    }
}

fn obtener_contenido(contenido_notas: &str, titulo_seleccionado: &str) -> Vec<String> {
    let vector_de_contenidos: Vec<&str> = contenido_notas.lines().collect();

    let mut resultados = Vec::new();

    for index in (0..vector_de_contenidos.len()).step_by(5) {
        let titulo_libro = limpiar_contenido(vector_de_contenidos[index]);
        if titulo_libro == titulo_seleccionado {
            if let Some(contenido_del_libro) = vector_de_contenidos.get(index + 3) {
                resultados.push(contenido_del_libro.to_string());
            }
        }
    }

    return resultados;
}

fn guardar_contenido(contenido_titulo_seleccionado: Vec<String>) {
    let mut file = File::create("prueba.txt").expect("Error al crear el fichero");

    let contenido_unido = contenido_titulo_seleccionado.join("\n");

    match file.write_all(contenido_unido.as_bytes()) {
        Ok(_) => println!("El fichero se ha guardado correctamente."),
        Err(_) => println!("Error al guardar el fichero."),
    };
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
            let path_notas = &args[2];

            match comando.to_lowercase().as_str() {
                "--parser" | "-p" => {
                    let contenido_notas: String = leer_fichero_notas(path_notas);

                    let titulos_disponibles: HashSet<String> =
                        obtener_titulos_libros(&contenido_notas);

                    let titulo_seleccionado = elegir_titulo(titulos_disponibles);

                    let contenido_titulo_seleccionado =
                        obtener_contenido(&contenido_notas, &titulo_seleccionado.as_str());

                    guardar_contenido(contenido_titulo_seleccionado);
                }
                _ => println!("Comando no disponible, utiliza --help o -h."),
            }
        }
        _ => println!("Comando no disponible, utiliza --help o -h."),
    }
}
