// =============================================================
// DEMO 4 : Closures, Fn/FnMut/FnOnce et Ordre Superieur
// =============================================================

fn main() {
    println!("=== DEMO 4 : Closures et Ordre Superieur ===\n");

    // ---------------------------------------------------------
    // 1. Closures de base
    // ---------------------------------------------------------
    println!("--- Closures de base ---");

    let doubler = |x: i32| x * 2;
    let ajouter = |a: i32, b: i32| a + b;
    let saluer = || println!("  Bonjour depuis une closure !");

    println!("  doubler(5) = {}", doubler(5));
    println!("  ajouter(3, 7) = {}", ajouter(3, 7));
    saluer();

    // Closure multi-lignes
    let analyser = |texte: &str| -> (usize, usize) {
        let mots = texte.split_whitespace().count();
        let chars = texte.len();
        (mots, chars)
    };
    let (mots, chars) = analyser("Rust est genial pour la concurrence");
    println!("  Mots: {}, Caracteres: {}", mots, chars);

    // ---------------------------------------------------------
    // 2. Capture par reference (Fn)
    // ---------------------------------------------------------
    println!("\n--- Capture par reference (Fn) ---");

    let nom = String::from("Alice");
    let prefixe = "Dr.";

    // Capture &nom et &prefixe (emprunts immutables)
    let presenter = || {
        println!("  Bonjour, {} {}", prefixe, nom);
    };

    presenter();
    presenter(); // Peut etre appelee plusieurs fois
    println!("  nom est toujours valide : {}", nom);

    // ---------------------------------------------------------
    // 3. Capture par reference mutable (FnMut)
    // ---------------------------------------------------------
    println!("\n--- Capture par reference mutable (FnMut) ---");

    let mut total = 0;
    let mut journal: Vec<String> = Vec::new();

   {
     // Capture &mut total et &mut journal
    let mut enregistrer = |montant: i32, description: &str| {
        total += montant;
        journal.push(format!("  {} : +{}", description, montant));
    };

    enregistrer(100, "Salaire");
    enregistrer(50, "Freelance");
    enregistrer(-30, "Courses");
   }

    // drop(enregistrer); // Libere les emprunts mutables

    println!("  Total : {}", total);
    for entree in &journal {
        println!("{}", entree);
    }

    // ---------------------------------------------------------
    // 4. Capture par move (FnOnce)
    // ---------------------------------------------------------
    println!("\n--- Capture par move (FnOnce) ---");

    let donnees = vec![1, 2, 3, 4, 5];

    // move : prend l'ownership de donnees
    let consommer = move || {
        let somme: i32 = donnees.iter().sum();
        println!("  Somme des donnees = {}", somme);
        donnees // Retourne l'ownership
    };


    // println!("{:?}", donnees); // ERREUR : donnees a ete moved
    let donnees_retournees = consommer();
    println!("  Donnees recuperees : {:?}", donnees_retournees);

    // ---------------------------------------------------------
    // 5. Closures comme parametres de fonction
    // ---------------------------------------------------------
    println!("\n--- Closures comme parametres ---");

    // Fn : closure qui ne modifie rien
    let resultat = appliquer_fn(|x| x * x, 7);
    println!("  7^2 = {}", resultat);

    // FnMut : closure qui peut modifier son environnement
    let mut compteur = 0;
    repeter_fn_mut(3, || {
        compteur += 1;
        println!("  Iteration {}", compteur);
    });

    println!("  Iteration {}", compteur);

    // FnOnce : closure qui consomme ses captures
    let message = String::from("Message important !");
    executer_fn_once(move || {
        println!("  {}", message);
        // message est consomme ici
    });


    // ---------------------------------------------------------
    // 6. Fonctions qui retournent des closures
    // ---------------------------------------------------------
    println!("\n--- Retourner des closures ---");

    let doubler_fn = creer_multiplicateur(2);
    let tripler_fn = creer_multiplicateur(3);

    println!("  doubler(5) = {}", doubler_fn(5));
    println!("  tripler(5) = {}", tripler_fn(5));


    let pipeline = creer_pipeline(vec![
        Box::new(|x| x + 10),
        Box::new(|x| x * 2),
        Box::new(|x| x - 5),
    ]);
    println!("  pipeline(3) = (3+10)*2-5 = {}", pipeline(3));

    // ---------------------------------------------------------
    // 7. Iterateurs et closures (usage reel)
    // ---------------------------------------------------------
    println!("\n--- Iterateurs et closures ---");

    let nombres = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Chaine d'operations
    let resultat: Vec<String> = nombres
        .iter()
        .filter(|&&x| x % 2 == 0) // Pairs
        .map(|&x| x * x) // Au carre
        .map(|x| format!("{}", x)) // En String
        .collect();
    println!("  Pairs au carre : {:?}", resultat);

    // fold (reduce)
    let stats = nombres.iter().fold(
        Stats {
            somme: 0,
            count: 0,
            min: i32::MAX,
            max: i32::MIN,
        },
        |mut acc, &x| {
            acc.somme += x;
            acc.count += 1;
            acc.min = acc.min.min(x);
            acc.max = acc.max.max(x);
            acc
        },
    );
    println!(
        "  Stats : somme={}, count={}, min={}, max={}, moy={:.1}",
        stats.somme,
        stats.count,
        stats.min,
        stats.max,
        stats.somme as f64 / stats.count as f64
    );

    // ---------------------------------------------------------
    // 8. Pattern : Strategy via closures
    // ---------------------------------------------------------
    println!("\n--- Pattern Strategy ---");

    let mut processeur = Processeur::new();

    processeur.ajouter_etape("Trim", Box::new(|s: String| s.trim().to_string()));
    processeur.ajouter_etape("Majuscules", Box::new(|s: String| s.to_uppercase()));
    processeur.ajouter_etape(
        "Exclamation",
        Box::new(|s: String| format!("{}!!!", s)),
    );

    let resultat = processeur.executer(String::from("  hello world  "));
    println!("  Resultat : '{}'", resultat);
}

// === Fonctions d'ordre superieur ===

fn appliquer_fn<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn repeter_fn_mut<F: FnMut()>(n: u32, mut f: F) {
    for _ in 0..n {
        f();
    }
}

fn executer_fn_once<F: FnOnce()>(f: F) {
    f();
}

// === Retour de closures ===

fn creer_multiplicateur(facteur: i32) -> impl Fn(i32) -> i32 {
    move |x| x * facteur
}

fn creer_pipeline(etapes: Vec<Box<dyn Fn(i32) -> i32>>) -> impl Fn(i32) -> i32 {
    move |mut x| {
        for etape in &etapes {
            x = etape(x);
        }
        x
    }
}

// === Types auxiliaires ===

struct Stats {
    somme: i32,
    count: i32,
    min: i32,
    max: i32,
}

struct Processeur {
    etapes: Vec<(&'static str, Box<dyn Fn(String) -> String>)>,
}

impl Processeur {
    fn new() -> Self {
        Processeur { etapes: Vec::new() }
    }

    fn ajouter_etape(&mut self, nom: &'static str, f: Box<dyn Fn(String) -> String>) {
        self.etapes.push((nom, f));
    }

    fn executer(&self, mut input: String) -> String {
        for (nom, etape) in &self.etapes {
            input = etape(input);
            println!("    Apres '{}' : '{}'", nom, input);
        }
        input
    }
}
