use clap::Parser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Kindle Parser")]
#[command(version = "0.1")]
#[command(about = "CLI para parsear ficheros txt de los subrayados y notas de Kindle.")]
struct KindleCLI {
    /// Parsea el fichero txt dado el path donde se encuentra el fichero
    #[arg(short, long, value_name = "PATH")]
    parser: Option<PathBuf>,
}

fn leer_fichero_notas(path_notas: &PathBuf) -> String {
    return match fs::read_to_string(path_notas) {
        Ok(contenido) => contenido,
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

    return titulos_disponibles_filtrados;
}

fn elegir_titulo(titulos_disponibles: HashSet<String>) -> String {
    // Para crear un mapeo entre cada título disponible y un indice que permita al
    // usuario elegir un titulo, podemos usar un HashMap
    let indice_titulos: HashMap<usize, String> =
        titulos_disponibles.into_iter().enumerate().collect();

    // Los HashMap no están ordenados
    let mut indice_titulos_ordenados: Vec<(&usize, &String)> = indice_titulos.iter().collect();
    indice_titulos_ordenados.sort_by_key(|(indice, _)| *indice);

    println!("Indice de títulos encontrados:\n");
    indice_titulos_ordenados.iter().for_each(|(id, titulo)| {
        println!("{} - {}", id, titulo);
    });

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

fn guardar_contenido(contenido_titulo_seleccionado: Vec<String>, titulo_seleccionado: &str) {
    fs::create_dir_all("outputs").expect("Error al crear la carpeta 'outputs'.");

    let path_fichero = format!("outputs/{}.txt", titulo_seleccionado);
    let mut file = File::create(path_fichero).expect("Error al crear el fichero");

    let contenido_unido = contenido_titulo_seleccionado.join("\n");

    match file.write_all(contenido_unido.as_bytes()) {
        Ok(_) => println!("El fichero se ha guardado correctamente."),
        Err(_) => println!("Error al guardar el fichero."),
    };
}

fn main() {
    let cli = KindleCLI::parse();

    if let Some(path_notas) = cli.parser.as_ref() {
        let contenido_notas: String = leer_fichero_notas(path_notas);
        let titulos_disponibles: HashSet<String> = obtener_titulos_libros(&contenido_notas);
        let titulo_seleccionado = elegir_titulo(titulos_disponibles);
        let contenido_titulo_seleccionado =
            obtener_contenido(&contenido_notas, &titulo_seleccionado.as_str());
        guardar_contenido(contenido_titulo_seleccionado, &titulo_seleccionado);
    }
}
