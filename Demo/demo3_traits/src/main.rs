// =============================================================
// DEMO 3 : Traits, Polymorphisme et Generiques
// =============================================================

use std::fmt;

fn main() {
    println!("=== DEMO 3 : Traits et Polymorphisme ===\n");

    // ---------------------------------------------------------
    // 1. Definir et implementer des traits
    // ---------------------------------------------------------
    println!("--- Traits de base ---");

    let cercle = Cercle { rayon: 5.0 };
    let rect = Rectangle {
        largeur: 4.0,
        hauteur: 6.0,
    };
    let tri = TriangleEquilateral { cote: 3.0 };

    println!("{}", cercle); // Utilise Display
    println!("  Aire = {:.2}, Perimetre = {:.2}", cercle.aire(), cercle.perimetre());
    println!("  {}", cercle.description()); // Methode par defaut

    println!("{}", rect);
    println!("  Aire = {:.2}, Perimetre = {:.2}", rect.aire(), rect.perimetre());

    println!("{}", tri);
    println!("  Aire = {:.2}", tri.aire());

    // ---------------------------------------------------------
    // 2. Dispatch statique (impl Trait / generiques)
    // ---------------------------------------------------------
    println!("\n--- Dispatch statique ---");

    afficher_info_statique(&cercle);
    afficher_info_statique(&rect);

    afficher_info_generique(&cercle);
    afficher_info_generique(&tri);

    // ---------------------------------------------------------
    // 3. Dispatch dynamique (dyn Trait)
    // ---------------------------------------------------------
    println!("\n--- Dispatch dynamique ---");

    let formes: Vec<Box<dyn Forme>> = vec![
        Box::new(Cercle { rayon: 3.0 }),
        Box::new(Rectangle {
            largeur: 10.0,
            hauteur: 2.0,
        }),
        Box::new(TriangleEquilateral { cote: 5.0 }),
    ];

    let aire_totale: f64 = formes.iter().map(|f| f.aire()).sum();
    println!("Aire totale de toutes les formes : {:.2}", aire_totale);

    for forme in &formes {
        afficher_info_dynamique(forme.as_ref());
    }

    // ---------------------------------------------------------
    // 4. Trait bounds multiples
    // ---------------------------------------------------------
    println!("\n--- Trait bounds multiples ---");

    let resultat = comparer_et_afficher(&cercle, &rect);
    println!("La plus grande forme a une aire de : {:.2}", resultat);

    // ---------------------------------------------------------
    // 5. Traits derives et standard
    // ---------------------------------------------------------
    println!("\n--- Traits derives ---");

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = p1.clone();

    println!("p1 = {:?}", p1);             // Debug
    println!("p1 == p2 ? {}", p1 == p2);   // PartialEq
    println!("p3 = {:?} (clone)", p3);      // Clone

    // ---------------------------------------------------------
    // 6. Implementer des traits standard
    // ---------------------------------------------------------
    println!("\n--- Traits standard implementes ---");

    let mut temperatures = vec![
        Temperature(22.5),
        Temperature(18.0),
        Temperature(30.2),
        Temperature(15.7),
    ];
    temperatures.sort(); // Utilise Ord
    println!("Temperatures triees : {:?}", temperatures);

    let t1 = Temperature(20.0);
    let t2 = Temperature(5.0);
    let somme = t1 + t2; // Utilise Add
    println!("{:?} + {:?} = {:?}", t1, t2, somme);

    // ---------------------------------------------------------
    // 7. Trait objects et polymorphisme avance
    // ---------------------------------------------------------
    println!("\n--- Polymorphisme avance ---");

    let journal = Journal::new();
    let formes_creees: Vec<Box<dyn Forme>> = journal
        .entrees
        .iter()
        .filter_map(|entree| creer_forme(entree))
        .collect();

    for forme in &formes_creees {
        println!("  {}", forme.description());
    }
}

// =============================================================
// Trait Forme
// =============================================================

trait Forme: fmt::Display {
    fn aire(&self) -> f64;
    fn perimetre(&self) -> f64;

    // Methode par defaut
    fn description(&self) -> String {
        format!(
            "{} (aire={:.2}, perimetre={:.2})",
            self,
            self.aire(),
            self.perimetre()
        )
    }

    fn est_plus_grand_que(&self, autre: &dyn Forme) -> bool {
        self.aire() > autre.aire()
    }
}

// =============================================================
// Types geometriques
// =============================================================

struct Cercle {
    rayon: f64,
}

struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

struct TriangleEquilateral {
    cote: f64,
}

// --- Implementations de Forme ---

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI * self.rayon * self.rayon
    }
    fn perimetre(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.rayon
    }
}

impl Forme for Rectangle {
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }
    fn perimetre(&self) -> f64 {
        2.0 * (self.largeur + self.hauteur)
    }
}

impl Forme for TriangleEquilateral {
    fn aire(&self) -> f64 {
        (3.0_f64.sqrt() / 4.0) * self.cote * self.cote
    }
    fn perimetre(&self) -> f64 {
        3.0 * self.cote
    }
}

// --- Implementations de Display ---

impl fmt::Display for Cercle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cercle(rayon={})", self.rayon)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle({}x{})", self.largeur, self.hauteur)
    }
}

impl fmt::Display for TriangleEquilateral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TriangleEquilateral(cote={})", self.cote)
    }
}

// =============================================================
// Dispatch statique vs dynamique
// =============================================================

fn afficher_info_statique(forme: &impl Forme) {
    println!("  [statique] {}", forme.description());
}

fn afficher_info_generique<T: Forme>(forme: &T) {
    println!("  [generique] {}", forme.description());
}

fn afficher_info_dynamique(forme: &dyn Forme) {
    println!("  [dynamique] {}", forme.description());
    // match forme {
    //     Forme::Cercle(c) => { c.rayon}
    //     Forme::Rectangle(r)=>{ r.lar}
    // }
}

fn comparer_et_afficher<T, U>(a: &T, b: &U) -> f64
where
    T: Forme + fmt::Display,
    U: Forme + fmt::Display,
{
    if a.aire() > b.aire() {
        println!("  {} est plus grand que {}", a, b);
        a.aire()
    } else {
        println!("  {} est plus grand ou egal a {}", b, a);
        b.aire()
    }
}

// =============================================================
// Traits derives et standard
// =============================================================

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Temperature(f64);

impl Eq for Temperature {
}

impl Ord for Temperature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::ops::Add for Temperature {
    type Output = Temperature;

    fn add(self, other: Temperature) -> Temperature {
        Temperature(self.0 + other.0)
    }
}

// =============================================================
// Factory pattern avec trait objects
// =============================================================

struct Journal {
    entrees: Vec<String>,
}

impl Journal {
    fn new() -> Self {
        Journal {
            entrees: vec![
                String::from("cercle:7.5"),
                String::from("rectangle:3:8"),
                String::from("triangle:6"),
                String::from("inconnu:0"),
            ],
        }
    }
}

fn creer_forme(spec: &str) -> Option<Box<dyn Forme>> {
    let parts: Vec<&str> = spec.split(':').collect();
    match parts[0] {
        "cercle" => {
            let rayon: f64 = parts[1].parse().ok()?;
            Some(Box::new(Cercle { rayon }))
        }
        "rectangle" => {
            let largeur: f64 = parts[1].parse().ok()?;
            let hauteur: f64 = parts[2].parse().ok()?;
            Some(Box::new(Rectangle { largeur, hauteur }))
        }
        "triangle" => {
            let cote: f64 = parts[1].parse().ok()?;
            Some(Box::new(TriangleEquilateral { cote }))
        }
        _ => {
            println!("  [factory] Type inconnu : '{}'", parts[0]);
            None
        }
    }
}
