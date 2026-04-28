// =============================================================
// SOLUTIONS — Exercice 3 : Traits et Polymorphisme
// =============================================================

use std::fmt;

fn main() {
    println!("=== Solution Exercice 3 ===\n");
    solution_3_1();
    solution_3_2();
    solution_3_3();
    solution_3_4();
    solution_3_5();
}

// --- 3.1 : Trait Affichable ---

trait Affichable {
    fn en_texte(&self) -> String;

    fn en_json(&self) -> String {
        format!("{{\"value\": \"{}\"}}", self.en_texte())
    }
}

impl Affichable for i32 {
    fn en_texte(&self) -> String {
        self.to_string()
    }

    fn en_json(&self) -> String {
        format!("{{\"value\": {}}}", self)
    }
}

impl Affichable for String {
    fn en_texte(&self) -> String {
        self.clone()
    }
}

struct Adresse {
    rue: String,
    ville: String,
    code_postal: String,
}

impl Affichable for Adresse {
    fn en_texte(&self) -> String {
        format!("{}, {} {}", self.rue, self.code_postal, self.ville)
    }

    fn en_json(&self) -> String {
        format!(
            "{{\"rue\": \"{}\", \"ville\": \"{}\", \"code_postal\": \"{}\"}}",
            self.rue, self.ville, self.code_postal
        )
    }
}

fn afficher_tout(elements: &[&dyn Affichable]) {
    for elem in elements {
        println!("    Texte : {}", elem.en_texte());
        println!("    JSON  : {}", elem.en_json());
        println!();
    }
}

fn solution_3_1() {
    println!("--- 3.1 : Trait Affichable ---");
    let nombre = 42;
    let texte = String::from("Bonjour Rust");
    let adresse = Adresse {
        rue: "12 rue de la Paix".into(),
        ville: "Paris".into(),
        code_postal: "75002".into(),
    };

    let elements: Vec<&dyn Affichable> = vec![&nombre, &texte, &adresse];
    afficher_tout(&elements);
}

// --- 3.2 : Systeme de formes ---

trait Mesurable {
    fn aire(&self) -> f64;
    fn perimetre(&self) -> f64;
    fn nom(&self) -> &str;
}

struct Carre {
    cote: f64,
}
struct Cercle {
    rayon: f64,
}
struct TriangleRectangle {
    base: f64,
    hauteur: f64,
}

impl Mesurable for Carre {
    fn aire(&self) -> f64 {
        self.cote * self.cote
    }
    fn perimetre(&self) -> f64 {
        4.0 * self.cote
    }
    fn nom(&self) -> &str {
        "Carre"
    }
}

impl Mesurable for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI * self.rayon * self.rayon
    }
    fn perimetre(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.rayon
    }
    fn nom(&self) -> &str {
        "Cercle"
    }
}

impl Mesurable for TriangleRectangle {
    fn aire(&self) -> f64 {
        0.5 * self.base * self.hauteur
    }
    fn perimetre(&self) -> f64 {
        let hyp = (self.base * self.base + self.hauteur * self.hauteur).sqrt();
        self.base + self.hauteur + hyp
    }
    fn nom(&self) -> &str {
        "TriangleRectangle"
    }
}

fn la_plus_grande<'a>(formes: &'a [Box<dyn Mesurable>]) -> Option<&'a dyn Mesurable> {
    formes
        .iter()
        .max_by(|a, b| a.aire().partial_cmp(&b.aire()).unwrap())
        .map(|b| b.as_ref())
}

fn trier_par_aire(formes: &mut [Box<dyn Mesurable>]) {
    formes.sort_by(|a, b| a.aire().partial_cmp(&b.aire()).unwrap());
}

fn resume(formes: &[Box<dyn Mesurable>]) {
    println!("    {:<20} {:>10} {:>12}", "Forme", "Aire", "Perimetre");
    println!("    {:-<20} {:-<10} {:-<12}", "", "", "");
    for f in formes {
        println!(
            "    {:<20} {:>10.2} {:>12.2}",
            f.nom(),
            f.aire(),
            f.perimetre()
        );
    }
}

fn solution_3_2() {
    println!("--- 3.2 : Systeme de formes ---");
    let mut formes: Vec<Box<dyn Mesurable>> = vec![
        Box::new(Carre { cote: 5.0 }),
        Box::new(Cercle { rayon: 3.0 }),
        Box::new(TriangleRectangle {
            base: 6.0,
            hauteur: 8.0,
        }),
        Box::new(Carre { cote: 2.0 }),
    ];

    if let Some(plus_grande) = la_plus_grande(&formes) {
        println!("  Plus grande : {} (aire={:.2})", plus_grande.nom(), plus_grande.aire());
    }

    trier_par_aire(&mut formes);
    println!("  Tableau (trie par aire) :");
    resume(&formes);
    println!();
}

// --- 3.3 : Trait generique Transformable ---

trait Transformable<T> {
    fn transformer(&self) -> T;
}

impl Transformable<Vec<char>> for String {
    fn transformer(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl Transformable<i32> for Vec<i32> {
    fn transformer(&self) -> i32 {
        self.iter().sum()
    }
}

impl Transformable<f64> for (f64, f64) {
    fn transformer(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

fn appliquer_transformation<T, U>(valeur: &T) -> U
where
    T: Transformable<U>,
{
    valeur.transformer()
}

fn solution_3_3() {
    println!("--- 3.3 : Transformable ---");

    let texte = String::from("Rust");
    let chars: Vec<char> = appliquer_transformation(&texte);
    println!("  '{}' -> {:?}", texte, chars);

    let nombres = vec![1, 2, 3, 4, 5];
    let somme: i32 = appliquer_transformation(&nombres);
    println!("  {:?} -> {}", nombres, somme);

    let point = (3.0, 4.0);
    let distance: f64 = appliquer_transformation(&point);
    println!("  {:?} -> distance = {:.2}", point, distance);
    println!();
}

// --- 3.4 : Trait From/Into pour Temperature ---

#[derive(Debug)]
struct Celsius(f64);
#[derive(Debug)]
struct Fahrenheit(f64);
#[derive(Debug)]
struct Kelvin(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Celsius> for Kelvin {
    fn from(c: Celsius) -> Self {
        Kelvin(c.0 + 273.15)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

impl From<Kelvin> for Celsius {
    fn from(k: Kelvin) -> Self {
        Celsius(k.0 - 273.15)
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}°F", self.0)
    }
}

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}K", self.0)
    }
}

fn solution_3_4() {
    println!("--- 3.4 : From/Into Temperature ---");

    let c = Celsius(100.0);
    let f: Fahrenheit = Celsius(100.0).into();
    let k: Kelvin = Celsius(100.0).into();
    println!("  {} = {} = {}", c, f, k);

    let c2 = Celsius(0.0);
    let k2: Kelvin = Celsius(0.0).into();
    println!("  {} = {}", c2, k2);

    let f3 = Fahrenheit(98.6);
    let c3: Celsius = Fahrenheit(98.6).into();
    println!("  {} = {}", f3, c3);
    println!();
}

// --- 3.5 : Iterator Syracuse ---

struct SuiteSyracuse {
    n: u64,
    termine: bool,
}

impl SuiteSyracuse {
    fn new(depart: u64) -> Self {
        SuiteSyracuse {
            n: depart,
            termine: false,
        }
    }
}

impl Iterator for SuiteSyracuse {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.termine {
            return None;
        }
        let courant = self.n;
        if courant == 1 {
            self.termine = true;
            return Some(1);
        }
        if self.n % 2 == 0 {
            self.n /= 2;
        } else {
            self.n = 3 * self.n + 1;
        }
        Some(courant)
    }
}

fn solution_3_5() {
    println!("--- 3.5 : Syracuse ---");

    // Suite pour n = 27
    let suite27: Vec<u64> = SuiteSyracuse::new(27).collect();
    println!("  Syracuse(27) : longueur = {}", suite27.len());
    println!("  Premiers termes : {:?}", &suite27[..10.min(suite27.len())]);
    println!("  Max atteint : {}", suite27.iter().max().unwrap());

    // Plus longue suite parmi 1..100
    let (meilleur_n, meilleure_longueur) = (1u64..=100)
        .map(|n| (n, SuiteSyracuse::new(n).count()))
        .max_by_key(|&(_, len)| len)
        .unwrap();
    println!(
        "  Plus longue suite (1-100) : n={}, longueur={}",
        meilleur_n, meilleure_longueur
    );
}
