use std::ptr::eq;

use rgrep::regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Define la prueba unitaria
#[test]
fn test_regex_1() {
    // Define una expresión regular y un archivo de prueba
    let regex = "a[ec]d";
    let filepath = "unit_test.txt";

    // Crea una instancia de Regex con la expresión regular
    let mut regex_instance = match Regex::new(regex) {
        Ok(regex) => regex,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    // Lee el archivo de prueba y realiza la prueba
    let contents = match File::open(filepath) {
        Ok(contents) => contents,
        Err(err) => {
            panic!("Error al leer el archivo de prueba: {}", err);
        }
    };

    // Crear un lector de líneas para el archivo
    let reader = io::BufReader::new(contents);

    // Leer todas las líneas del archivo en un vector
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();


    match regex_instance.test(&lines[0]) {
        Ok(result) => {
            // Verifica si la prueba pasó o falló
            assert_eq!(result, "acd");
            println!("La prueba pasó: el texto coincide con la expresión regular");
        }
        Err(err) => {
            panic!("Error al realizar la prueba: {}", err);
        }
    }


}

#[test]
fn test_regex_2() {
    // Define una expresión regular y un archivo de prueba
    let regex = "a[ec]d";
    let filepath = "unit_test.txt";

    // Crea una instancia de Regex con la expresión regular
    let mut regex_instance = match Regex::new(regex) {
        Ok(regex) => regex,
        Err(err) => {
            panic!("Error al crear la instancia de Regex: {}", err);
        }
    };

    // Lee el archivo de prueba y realiza la prueba
    let contents = match File::open(filepath) {
        Ok(contents) => contents,
        Err(err) => {
            panic!("Error al leer el archivo de prueba: {}", err);
        }
    };

    // Crear un lector de líneas para el archivo
    let reader = io::BufReader::new(contents);

    // Leer todas las líneas del archivo en un vector
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();


    match regex_instance.test(&lines[1]) {
        Ok(result) => {
            // Verifica si la prueba pasó o falló
            assert_eq!(result, "");
            println!("La prueba pasó: el texto coincide con la expresión regular");
        }
        Err(err) => {
            panic!("Error al realizar la prueba: {}", err);
        }
    }


}
