use std::collections::HashMap;
use std::collections::HashSet;

// Collection
fn main() {

    // Vector
    vectores();

    // HashMap
    hashmap();

    // HashSet
    hashset();
    
}

fn hashset () {

    let mut user_id = HashSet::new();
    user_id.insert(1);
    user_id.insert(2);
    user_id.remove(&1);

    // Metodos importantes
    
    // union
    // intersection
    // difference
    // symmetric_difference

}

fn hashmap () {

    let mut scores: HashMap<String, i32> = HashMap::new();

    // Agregar elementos
    scores.insert(String::from("America"), 20);

    // Obtener elementos
    let score = scores.get("America");

    match score {
        Some(score) => println!("{}", score),
        None => println!("No existe"),    
    }

    // Recorrer elementos
    for (key, value) in &scores {
        println!("Key: {}  Value:{}", key, value);
    }

    // Escribir elementos pero si no existe lo agrega
    scores.entry(String::from("Mexico")).or_insert(15);

}


fn vectores (){
    // Vector
    let mut vec1: Vec<i32> = Vec::new();
    let vec2 = vec![1, 2, 3];
    println!("{:?}", vec2);

    // Agregar un elemento
    vec1.push(1); 

    // Obtener un elemento ( Puede generar error si el indice no existe )
    let primer = vec1[0]; 

    // Obtener un elemento ( Controla el error si el indice no existe )
    let segundo = vec1.get(1);  

    if segundo.is_some() {
        println!("{:?}", segundo.unwrap());
    }

    match vec1.get(4) {
        Some(x) => println!("{:?}", x),
        None => println!("No se encontro el elemento"),
    }

    for i in &vec1 {
        println!("{:?}", i);
    }

    // Cambiar el valor por referencia
    for i in &mut vec1 {
        *i += 1;    
    }


    enum Mensaje {
        TEXT(String),
        ERROR(i32),
    }

    let mensajes = vec![ Mensaje::TEXT("Hola".to_string()), Mensaje::ERROR(1) ];

    for m in &mensajes {
        match m {
            Mensaje::TEXT(x) => println!("Texto {:?}", x),
            Mensaje::ERROR(x) => println!("Error {:?}", x),
        }
    }
}