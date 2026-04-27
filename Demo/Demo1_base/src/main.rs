fn main() {
    println!("=== DEMO 1 : concepts de base ===");

    println!("--- variable et mutabilite ---");

    let x = 5;
    println!("x = {}",x);
    // x = 6;
    // println!("x = {}",x) cannot assign twice to immutable variable

    let mut y = 10;
    println!("y = {}",y);
    y = 20;
    println!("y (apres modifiication) = {}",y);

    let z = 5;
    let z = z +1;
    let z = z * 2 ;
    println!("z (apres shadowing) = {}",z);

    // 2. Types scalaires

    println!("\n--- types scalaires ---");
    let entier: i32 = -42;
    let non_signie: u64 = 1_000_000;
    let flottant: f64 = 3.14;
    let boolean: bool = true;
    let charactere: char= 'c';

    println!("entier = {}, non_signie = {}",entier,non_signie);
    println!("flottant = {}",flottant);
    println!("bool = {}, char = {}",boolean,charactere);

    let hex = 0xff;
    let octal = 0o77;
    let binaire = 0b1011_0111;
    let byte: u8 = b'A';
    println!("hex={}, octal={}, binaire={}, byte={}", hex, octal, binaire, byte);

    // 3. types compose
    println!("\n--- Types composes ---");

    // Tuples
    let point: (f64, String) = (3.5, String::from("Hello (String, heap)"));
    let (px, py) = point; // Destructuration
    println!("Point : ({}, {})", px, py);
    println!("Point.0 = {}", point.0);

    // Tableaux (taille fixe)
    let nombres: [i32;5] = [1, 2, 3, 4, 5];
    let zeros = [0; 10];
    println!("nombres = {:?}", nombres);
    println!("zeros = {:?}", zeros);
    println!("nombres[2] = {}", nombres[2]);

    // 4. Strings
    println!("\n--- Strings ---");

    let s1: &str = "Hello (string slice, statique)";
    let s2: String = String::from("Hello (String, heap)");
    let s3 = format!("{} + {}", s1, s2);
    println!("{}", s3);

    let mut s4 = String::from("Bonjour");
    s4.push_str(", monde!");
    s4.push('!');
    println!("{}", s4);

    // 5. Expressions et blocs

    println!("\n--- Expressions ---");

    let resultat = {
        let a = 10;
        let b = 20;
        a + b // Pas de ; = valeur de retour du bloc
    };
    println!("Resultat du bloc = {}", resultat);

    let statut = if resultat > 25 { "grand" } else { "petit" };
    println!("Statut = {}", statut);

    // 6. Fonctions
    println!("\n--- Fonctions ---");

    println!("additionner(3, 7) = {}", additionner(3, 7));
    println!("factorielle(6) = {}", factorielle(6));

    match diviser(10.0, 3.0) {
        Some(r) => println!("10 / 3 = {:.4}", r),
        None => println!("Division par zero!"),
    }
    match diviser(10.0, 0.0) {
        Some(r) => println!("10 / 0 = {:.4}", r),
        None => println!("Division par zero!"),
    }

    // 7. Conditions et match

    println!("\n--- Conditions et match ---");

    let temperature = 22;
    let description = match temperature {
        t if t < 0 => "glacial",
        0..=10 => "froid",
        11..=20 => "frais",
        21..=30 => "agreable",
        _ => "chaud",
    };
    println!("{}°C = {}", temperature, description);

    // if let
    let valeur: Option<i32> = Some(42);
    if let Some(v) = valeur {
        println!("Valeur extraite : {}", v);
    }

// 8. Boucles

    println!("\n--- Boucles ---");

    // loop avec valeur de retour
    let mut compteur = 0;
    let resultat_loop = loop {
        compteur += 1;
        if compteur == 5 {
            break compteur * 10;
        }
    };
    println!("Resultat loop = {}", resultat_loop);

    // for avec range
    print!("Range 0..5 : ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // for avec enumerate
    let fruits = ["pomme", "banane", "cerise"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  [{}] {}", i, fruit);
    }

    for (fruit) in fruits {
        println!("{}",  fruit);
    }

    // 9. Structs
    println!("\n--- Structs ---");

    let mut user = Utilisateur::new("Alice", "alice@example.com", 30);
    println!("{:?}", user);
    user.anniversaire();
    println!("Apres anniversaire : age = {}", user.age);


    #[derive(Debug)]
    struct Utilisateur {
        nom: String,
        email: String,
        age: u32,
    }

    impl Utilisateur {
        fn new(nom: &str, email: &str, age: u32) -> Self {
            Self {
                nom: String::from(nom),
                email: String::from(email),
                age,
            }
        }

        fn anniversaire(&mut self) {
            self.age += 1;
        }
    }
 
    // 10. Enums
    
    println!("\n--- Enums ---");

    let formes = vec![
        Forme::Cercle { rayon: 5.0 },
        Forme::Rectangle {
            largeur: 4.0,
            hauteur: 6.0,
        },
        Forme::Triangle(3.0, 4.0, 5.0),
    ];

    for forme in &formes {
        println!("{} -> aire = {:.2}", decrire_forme(forme), aire(forme));
    }

}


enum Forme {
    Cercle { rayon: f64 },
    Rectangle { largeur: f64, hauteur: f64 },
    Triangle(f64, f64, f64),
}

fn aire(forme: &Forme) -> f64 {
    match forme {
        Forme::Cercle { rayon } => std::f64::consts::PI * rayon * rayon,
        Forme::Rectangle { largeur, hauteur } => largeur * hauteur,
        Forme::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn decrire_forme(forme: &Forme) -> &str {
    match forme {
        Forme::Cercle { .. } => "Cercle",
        Forme::Rectangle { .. } => "Rectangle",
        Forme::Triangle(..) => "Triangle",
    }
}



fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

fn factorielle(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorielle(n - 1),
    }
}

fn diviser(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
