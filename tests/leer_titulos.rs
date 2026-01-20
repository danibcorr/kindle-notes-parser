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

fn elegir_titulo(todos_los_titulos: Vec<String>) -> String {
    let mut indice_titulos = HashMap::new();
    let mut contador: i64 = 0;

    for titulo in todos_los_titulos.into_iter() {
        indice_titulos.insert(contador, titulo);
        contador += 1;
    }

    println!("Títulos disponibles: {:?}", indice_titulos);

    println!("Introduce el indice del titulo a seleccionar: ");

    loop {
        let mut input_texto = String::new();

        io::stdin()
            .read_line(&mut input_texto)
            .expect("Error al leer el input");

        let input_user: i64 = match input_texto.trim().parse() {
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

        return indice_titulos[&input_user].to_string();
    }
}

fn mapear_contenido_titulo(path_notas: &str, titulo_seleccionado: String) -> Vec<String> {
    let contenido: String = fs::read_to_string(path_notas).expect("Error al leer el fichero");
    let contenido_iterable = contenido.lines();

    let vector_de_contenidos: Vec<&str> = contenido_iterable.collect();

    let mut titulo_contenido = HashMap::new();

    for index in (0..vector_de_contenidos.len()).step_by(5) {
        let titulo_libro = vector_de_contenidos[index].trim().replace("\u{feff}", "");
        if let Some(contenido_del_libro) = vector_de_contenidos.get(index + 3) {
            titulo_contenido
                .entry(titulo_libro)
                .or_insert(Vec::new())
                .push(contenido_del_libro.to_string());
        }
    }

    let contenido_final = titulo_contenido
        .remove(&titulo_seleccionado)
        .unwrap_or_else(|| Vec::new());

    println!("Mostrando contenido para: {}", titulo_seleccionado);
    println!("{:?}", contenido_final);

    contenido_final
}

fn guardar_contenido(contenido_titulo_seleccionado: Vec<String>) {
    let mut file = File::create("prueba.txt").expect("Error al crear el fichero");

    let contenido_unido = contenido_titulo_seleccionado.join("\n");

    file.write_all(contenido_unido.as_bytes())
        .expect("Error al escribir el fichero.");
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
                    let todos_los_titulos = obtener_titulos_libros(&valor_comando);
                    println!("Títulos de libros encontrados: {:?}", todos_los_titulos);

                    let titulo_seleccionado = elegir_titulo(todos_los_titulos);
                    println!("Titulo seleccionado: {titulo_seleccionado}");

                    let contenido_titulo_seleccionado =
                        mapear_contenido_titulo(&valor_comando, titulo_seleccionado);
                    guardar_contenido(contenido_titulo_seleccionado);
                }
                _ => println!("Comando no disponible, utiliza --help o -h."),
            }
        }
        _ => println!("Comando no disponible, utiliza --help o -h."),
    }
}
