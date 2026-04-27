// =============================================================
// SOLUTIONS — Exercice 1 : Variables, Types et Fonctions
// =============================================================


fn main() {
    println!("=== Solution Exercice 1 ===\n");
    solution_1_1();
    solution_1_2();
    solution_1_3();
    solution_1_4();
    solution_1_5();
}

// --- 1.1 : Conversions de temperature ---

fn celsius_vers_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_vers_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn solution_1_1() {
    println!("--- 1.1 : Conversions de temperature ---");
    println!("{:>6} | {:>8}", "°C", "°F");
    println!("{:-<6}-+-{:-<8}", "", "");
    for c in [-40.0, 0.0, 20.0, 37.0, 100.0] {
        println!("{:>6.0} | {:>8.1}", c, celsius_vers_fahrenheit(c));
    }
    println!("{:>6} | {:>8}", "°F", "°C");
    println!("{:-<6}-+-{:-<8}", "", "");
    for f in [-40.0, 32.0, 68.0,  98.6, 212.0] {
        println!("{:>6.0} | {:>8.1}", f, fahrenheit_vers_celsius(f));
    }
    println!();
}

// --- 1.2 : Analyse de notes ---

fn mention(note: u32) -> &'static str {
    match note {
        16..=20 => "Tres bien",
        14..=15 => "Bien",
        12..=13 => "Assez bien",
        10..=11 => "Passable",
        0..=9 => "Insuffisant",
        _ => "Note invalide",
    }
}

fn solution_1_2() {
    println!("--- 1.2 : Analyse de notes ---");
    for note in [18, 14, 12, 8, 25] {
        println!("  Note {} -> {}", note, mention(note));
    }
    println!();
}

// --- 1.3 : Fibonacci iteratif ---

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn solution_1_3() {
    println!("--- 1.3 : Fibonacci ---");
    print!("  ");
    for i in 0..20 {
        print!("{}", fibonacci(i));
        if i < 19 {
            print!(", ");
        }
    }
    println!("\n");
}

// --- 1.4 : Struct et methodes ---

struct Etudiant {
    nom: String,
    notes: Vec<u32>,
}

impl Etudiant {
    fn new(nom: &str) -> Self {
        Self {
            nom: String::from(nom),
            notes: Vec::new(),
        }
    }

    fn ajouter_note(&mut self, note: u32) {
        if note <= 20 {
            self.notes.push(note);
        }
    }

    fn moyenne(&self) -> f64 {
        if self.notes.is_empty() {
            return 0.0;
        }
        self.notes.iter().sum::<u32>() as f64 / self.notes.len() as f64
    }

    fn meilleure_note(&self) -> Option<u32> {
        self.notes.iter().max().copied()
    }

    fn a_valide(&self) -> bool {
        self.moyenne() >= 10.0
    }
}

fn solution_1_4() {
    println!("--- 1.4 : Struct Etudiant ---");
    let mut alice = Etudiant::new("Alice");
    alice.ajouter_note(15);
    alice.ajouter_note(18);
    alice.ajouter_note(12);
    alice.ajouter_note(25); // Ignoree (> 20)

    let mut bob = Etudiant::new("Bob");
    bob.ajouter_note(8);
    bob.ajouter_note(6);
    bob.ajouter_note(9);

    for etudiant in [&alice, &bob] {
        println!(
            "  {} : notes={:?}, moyenne={:.1}, meilleure={:?}, valide={}",
            etudiant.nom,
            etudiant.notes,
            etudiant.moyenne(),
            etudiant.meilleure_note(),
            etudiant.a_valide()
        );
    }
    println!();
}

// --- 1.5 : Enum et Pattern Matching ---

enum Operation {
    Addition(f64, f64),
    Soustraction(f64, f64),
    Multiplication(f64, f64),
    Division(f64, f64),
}

fn calculer(op: &Operation) -> Result<f64, String> {
    match op {
        Operation::Addition(a, b) => Ok(a + b),
        Operation::Soustraction(a, b) => Ok(a - b),
        Operation::Multiplication(a, b) => Ok(a * b),
        Operation::Division(a, b) => {
            if *b == 0.0 {
                Err(String::from("Division par zero"))
            } else {
                Ok(a / b)
            }
        }
    }
}

fn solution_1_5() {
    println!("--- 1.5 : Enum Operation ---");
    let operations = vec![
        ("10 + 3", Operation::Addition(10.0, 3.0)),
        ("10 - 3", Operation::Soustraction(10.0, 3.0)),
        ("10 * 3", Operation::Multiplication(10.0, 3.0)),
        ("10 / 3", Operation::Division(10.0, 3.0)),
        ("10 / 0", Operation::Division(10.0, 0.0)),
    ];

    for (label, op) in &operations {
        match calculer(op) {
            Ok(r) => println!("  {} = {:.2}", label, r),
            Err(e) => println!("  {} = ERREUR: {}", label, e),
        }
    }
}
