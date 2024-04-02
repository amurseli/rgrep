use rgrep::regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Obtener los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificar si se proporcionaron suficientes argumentos
    if args.len() < 3 {
        println!("Usage: {} <regex> <filepath>", args[0]);
        return;
    }

    // Obtener el patrón de expresión regular y la ruta del archivo desde los argumentos
    let regex_str = &args[1];
    let file_path = &args[2];

    // Compilar el patrón de expresión regular
    let pattern = match Regex::new(regex_str) {
        Ok(pattern) => pattern,
        Err(err) => {
            println!("Error al crear la expresión regular: {}", err);
            return;
        }
    };

    // Intentar abrir el archivo
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error al abrir el archivo '{}': {}", file_path, err);
            return;
        }
    };

    // Crear un lector de líneas para el archivo
    let reader = io::BufReader::new(file);

    // Leer todas las líneas del archivo en un vector
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Procesar cada línea para encontrar coincidencias con el patrón
    for line in lines {
        println!("La línea analizada es {:?}", line);
        let mut pattern = match Regex::new(regex_str) {
            Ok(regex) => regex,
            Err(err) => {
                println!("Error creating regex pattern: {}", err);
                continue; // Saltar a la siguiente iteración si hay un error
            }
        };
        match pattern.test(&line) {
            Ok(result) => println!("{}", result),
            Err(err) => println!("Error applying the regular expression pattern: {}", err),
        }
    }
}
