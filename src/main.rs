fn main() {
    let nombre = "Alberto"; // immutable
    print!("Hola, {}!\n", nombre);

    let mut edad = 30; // mutable
    print!("Tienes {} años.\n", edad);

    edad += 1; // incrementa la edad
    print!("Ahora tienes {} años.\n", edad);

    // Tipos de datos
    let _numero: i32 = 42; // entero
    let _decimal: f64 = 3.141592653589793238462643383279; // decimal
    let _booleano: bool = true; // booleano
    let _caracter: char = 'A'; // caracter
    let _cadena: &str = "Hola, mundo!"; // cadena de texto
    let _arreglo: [i32; 5] = [1, 2, 3, 4, 5]; // arreglo de enteros
    let _tupla: (i32, f64, char) = (42, 3.14, 'A'); // tupla de diferentes tipos
    let _vector: Vec<i32> = vec![1, 2, 3, 4, 5]; // vector de enteros
    let _diccionario: std::collections::HashMap<&str, i32> = std::collections::HashMap::new(); // diccionario (hashmap)
    
    // Estructuras de control
    if edad < 18 {
        print!("Eres menor de edad.\n");
    } else if edad >= 18 && edad < 65 {
        print!("Eres adulto.\n");
    } else {
        print!("Eres mayor de edad.\n");
    }

    // Bucle for
    for i in 0..5 {
        print!("Contando: {}\n", i);
    }

    // Bucle while
    let mut contador = 0;
    while contador < 5 {
        print!("Contando: {}\n", contador);
        contador += 1;
    }
    // Bucle loop
    let mut contador_infinito = 0;
    loop {
        if contador_infinito >= 5 {
            break;
        }
        print!("Contando: {}\n", contador_infinito);
        contador_infinito += 1;
    }

    // Funciones
    fn suma(a: i32, b: i32) -> i32 {
        a + b
    }
    let resultado = suma(5, 10);
    print!("La suma de 5 y 10 es: {}\n", resultado);

    // Clases y estructuras
    struct Persona {
        nombre: String,
        edad: i32,
    }
    impl Persona {
        fn nueva(nombre: String, edad: i32) -> Persona {
            Persona { nombre, edad }
        }

        fn saludar(&self) {
            print!("Hola, mi nombre es {} y tengo {} años.\n", self.nombre, self.edad);
        }
    }
    let persona = Persona::nueva("Alberto".to_string(), 30);
    persona.saludar();

    // Manejo de errores
    let resultado_division = dividir(10, 0);
    match resultado_division {
        Ok(valor) => print!("El resultado de la división es: {}\n", valor),
        Err(e) => print!("Error: {}\n", e),
    }
    fn dividir(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("No se puede dividir entre cero".to_string())
        } else {
            Ok(a / b)
        }
    }
}
