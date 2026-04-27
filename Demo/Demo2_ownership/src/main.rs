fn main() {
    println!("=== DEMO 2 : Ownership et Borrowing ===\n");

    // ---------------------------------------------------------
    // 1. Move semantics
    // ---------------------------------------------------------
    println!("--- Move semantics ---");

    let s1 = String::from("hello");
    let s2 = s1; // s1 est MOVE vers s2
    // println!("{}", s1); // ERREUR COMPILATION : value used after move
    println!("s2 = {}", s2);

    // Clone explicite
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {} (clone)", s2, s3);

    // ---------------------------------------------------------
    // 2. Copy semantics (types sur la stack)
    // ---------------------------------------------------------
    println!("\n--- Copy semantics ---");

    let x = 42;
    let y = x; // Copy (i32 implemente Copy)
    println!("x = {}, y = {} (les deux sont valides)", x, y);

    let point = (3.0, 4.0); // Les tuples de Copy types sont Copy
    let autre_point = point;
    println!("point = {:?}, autre = {:?}", point, autre_point);

    // ---------------------------------------------------------
    // 3. Ownership et fonctions
    // ---------------------------------------------------------
    println!("\n--- Ownership et fonctions ---");

    let texte = String::from("Bonjour");
    prendre_ownership(texte);
    // println!("{}", texte); // ERREUR : texte a ete move dans la fonction

    let nombre = 42;
    copier_valeur(nombre);
    println!("nombre est toujours valide : {}", nombre); // OK (Copy)

    // Rendre l'ownership
    let s = String::from("hello");
    let s = ajouter_monde(s); // s est move puis rendu
    println!("Resultat : {}", s);

    // ---------------------------------------------------------
    // 4. References immutables (&T)
    // ---------------------------------------------------------
    println!("\n--- References immutables ---");

    let mut message = String::from("Bonjour Rust");
    let len = calculer_longueur(&message); // Emprunt immutable
    println!("'{}' a {} caracteres", message, len); // message toujours valide

    // Plusieurs emprunts immutables simultanes
    let r1 = &message;
    let r2 = &message;
    let r3 = &message;

    println!("r1={}, r2={}, r3={}", r1, r2, r3);
    // ---------------------------------------------------------
    // 5. References mutables (&mut T)
    // ---------------------------------------------------------
    println!("\n--- References mutables ---");

    let mut texte = String::from("Hello");
    modifier_texte(&mut texte);
    println!("Apres modification : {}", texte);

    // Un seul emprunt mutable a la fois
    let r1 = &mut texte;
    // let r2 = &mut texte; // ERREUR : cannot borrow as mutable more than once
    r1.push_str("!!!");
    println!("Via r1 : {}", r1);

    // Apres la derniere utilisation de r1, on peut re-emprunter
    let r2 = &mut texte;
    r2.push_str("???");
    println!("Via r2 : {}", r2);

    // ---------------------------------------------------------
    // 6. Regles d'emprunt
    // ---------------------------------------------------------
    println!("\n--- Regles d'emprunt ---");

    let mut data = vec![1, 2, 3, 4, 5];

    // OK : emprunts immutables qui se terminent avant le mutable
    let premier = &data[0];
    let dernier = &data[data.len() - 1];
    println!("Premier = {}, Dernier = {}", premier, dernier);
    // premier et dernier ne sont plus utilises apres cette ligne

    // Maintenant on peut emprunter mutablement
    data.push(6);
    println!("data = {:?}", data);

    // ---------------------------------------------------------
    // 7. Slices
    // ---------------------------------------------------------
    println!("\n--- Slices ---");

    let phrase = String::from("Bonjour le monde Rust");
    let premier_mot = trouver_premier_mot(&phrase);
    println!("Premier mot : '{}'", premier_mot);

    let nombres = [10, 20, 30, 40, 50];
    let milieu = &nombres[1..4];
    println!("Milieu du tableau : {:?}", milieu);

    let somme = sommer_slice(milieu);
    println!("Somme du milieu : {}", somme);

    // ---------------------------------------------------------
    // 8. Lifetimes basiques
    // ---------------------------------------------------------
    println!("\n--- Lifetimes ---");

    let chaine1 = String::from("longue chaine");
    let resultat;
    {
        let chaine2 = String::from("xyz");
        resultat = le_plus_long(&chaine1, &chaine2);
        println!("Le plus long : '{}'", resultat);
    }
    // println!("{}", resultat); // Ne compile pas si chaine2 est droppee

    // Struct avec lifetime
    let roman = String::from("Un roman tres long et passionnant...");
    let extrait = Extrait::new(&roman);
    println!("Extrait : {:?}", extrait);

    // ---------------------------------------------------------
    // 9. Pattern Matching et ownership
    // ---------------------------------------------------------
    println!("\n--- Pattern matching et ownership ---");

    let messages = vec![
        Message::Texte(String::from("Bonjour")),
        Message::Nombre(42),
        Message::Rien,
    ];

    for msg in &messages {
        traiter_message(msg);
    }
}

// === Fonctions pour Ownership ===

fn prendre_ownership(s: String) {
    println!("J'ai pris ownership de : '{}'", s);
} // s est droppee ici

fn copier_valeur(n: i32) {
    println!("Copie de la valeur : {}", n);
}

fn ajouter_monde(mut s: String) -> String {
    s.push_str(", monde!");
    s // Rend l'ownership
}

// === Fonctions pour Borrowing ===

fn calculer_longueur(s: &String) -> usize {
    s.len()
} // s (la reference) sort du scope, mais la valeur pointee n'est pas droppee

fn modifier_texte(s: &mut String) {
    s.push_str(", World");
}

// === Fonctions pour Slices ===

fn trouver_premier_mot(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

fn sommer_slice(nombres: &[i32]) -> i32 {
    nombres.iter().sum()
}

// === Lifetimes ===

fn le_plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Extrait<'a> {
    contenu: &'a str,
}

impl<'a> Extrait<'a> {
    fn new(texte: &'a str) -> Self {
        let fin = std::cmp::min(20, texte.len());
        Extrait {
            contenu: &texte[..fin],
        }
    }
}

// === Enum pour pattern matching ===

enum Message {
    Texte(String),
    Nombre(i32),
    Rien,
}

fn traiter_message(msg: &Message) {
    match msg {
        Message::Texte(t) => println!("  Texte : {}", t),
        Message::Nombre(n) => println!("  Nombre : {}", n),
        Message::Rien => println!("  (rien)"),
    }
}
