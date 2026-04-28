// =============================================================
// SOLUTIONS — Exercice 4 : Closures et Ordre Superieur
// =============================================================

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Solution Exercice 4 ===\n");
    solution_4_1();
    solution_4_2();
    solution_4_3();
    solution_4_4();
    solution_4_5();
}

// --- 4.1 : Closures de base ---

fn solution_4_1() {
    println!("--- 4.1 : Closures de base ---");

    // 1. Closure carre
    let carre = |x: i32| x * x;
    println!("  carre(7) = {}", carre(7));

    // 2. Closure est_pair
    let est_pair = |x: i32| x % 2 == 0;
    println!("  est_pair(4) = {}, est_pair(7) = {}", est_pair(4), est_pair(7));

    // 3. Capture par reference
    let nom = String::from("Alice");
    let saluer = || println!("  Bienvenue, {} !", nom);
    saluer();
    saluer();
    println!("  nom est toujours valide : '{}'", nom);

    // 4. Closure mutable
    let mut count = 0u32;
    let mut compteur = || {
        count += 1;
        count
    };
    println!("  compteur() = {}", compteur());
    println!("  compteur() = {}", compteur());
    println!("  compteur() = {}", compteur());
    drop(compteur);
    println!("  count final = {}", count);

    println!();
}

// --- 4.2 : Fonctions d'ordre superieur ---

fn appliquer_a_tous(nombres: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    nombres.iter().map(|&x| f(x)).collect()
}

fn filtrer(nombres: &[i32], predicat: impl Fn(&i32) -> bool) -> Vec<i32> {
    nombres.iter().filter(|x| predicat(x)).copied().collect()
}

fn reduire(nombres: &[i32], init: i32, f: impl Fn(i32, i32) -> i32) -> i32 {
    nombres.iter().fold(init, |acc, &x| f(acc, x))
}

fn composer<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| g(f(x))
}

fn solution_4_2() {
    println!("--- 4.2 : Fonctions d'ordre superieur ---");

    let nums = [1, 2, 3, 4, 5];

    let doubles = appliquer_a_tous(&nums, |x| x * 2);
    println!("  appliquer_a_tous({:?}, *2) = {:?}", nums, doubles);

    let pairs = filtrer(&nums, |x| x % 2 == 0);
    println!("  filtrer({:?}, pair) = {:?}", nums, pairs);

    let somme = reduire(&nums, 0, |acc, x| acc + x);
    println!("  reduire({:?}, 0, +) = {}", nums, somme);

    let plus_un_fois_deux = composer(|x| x + 1, |x| x * 2);
    println!("  composer(+1, *2)(5) = {}", plus_un_fois_deux(5));

    println!();
}

// --- 4.3 : Pipeline de transformations ---

struct Pipeline<T> {
    valeur: T,
}

impl<T> Pipeline<T> {
    fn new(valeur: T) -> Self {
        Pipeline { valeur }
    }

    fn puis<U, F: FnOnce(T) -> U>(self, f: F) -> Pipeline<U> {
        Pipeline {
            valeur: f(self.valeur),
        }
    }

    fn resultat(self) -> T {
        self.valeur
    }
}

fn solution_4_3() {
    println!("--- 4.3 : Pipeline ---");

    let resultat = Pipeline::new("  Hello World  ".to_string())
        .puis(|s| s.trim().to_string())
        .puis(|s| s.to_uppercase())
        .puis(|s| format!(">>> {} <<<", s))
        .resultat();
    println!("  Pipeline string : '{}'", resultat);

    let resultat_num = Pipeline::new(vec![1, 2, 3, 4, 5])
        .puis(|v| v.into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>())
        .puis(|v| v.iter().map(|x| x * x).collect::<Vec<_>>())
        .puis(|v| v.iter().sum::<i32>())
        .resultat();
    println!("  Pipeline numerique : {}", resultat_num);

    println!();
}

// --- 4.4 : Validateurs fonctionnels ---

type Validateur<T> = Box<dyn Fn(&T) -> Result<(), String>>;

struct ValidateurCompose<T> {
    validateurs: Vec<Validateur<T>>,
}

impl<T> ValidateurCompose<T> {
    fn new() -> Self {
        ValidateurCompose {
            validateurs: Vec::new(),
        }
    }

    fn ajouter(&mut self, validateur: Validateur<T>) {
        self.validateurs.push(validateur);
    }

    fn valider(&self, valeur: &T) -> Vec<String> {
        self.validateurs
            .iter()
            .filter_map(|v| v(valeur).err())
            .collect()
    }
}

fn longueur_min(n: usize) -> Validateur<String> {
    Box::new(move |s: &String| {
        if s.len() >= n {
            Ok(())
        } else {
            Err(format!("Doit contenir au moins {} caracteres (actuel: {})", n, s.len()))
        }
    })
}

fn longueur_max(n: usize) -> Validateur<String> {
    Box::new(move |s: &String| {
        if s.len() <= n {
            Ok(())
        } else {
            Err(format!("Doit contenir au plus {} caracteres (actuel: {})", n, s.len()))
        }
    })
}

fn contient_majuscule() -> Validateur<String> {
    Box::new(|s: &String| {
        if s.chars().any(|c| c.is_uppercase()) {
            Ok(())
        } else {
            Err("Doit contenir au moins une majuscule".to_string())
        }
    })
}

fn contient_chiffre() -> Validateur<String> {
    Box::new(|s: &String| {
        if s.chars().any(|c| c.is_ascii_digit()) {
            Ok(())
        } else {
            Err("Doit contenir au moins un chiffre".to_string())
        }
    })
}

fn solution_4_4() {
    println!("--- 4.4 : Validateurs ---");

    let mut validateur_mdp = ValidateurCompose::new();
    validateur_mdp.ajouter(longueur_min(8));
    validateur_mdp.ajouter(longueur_max(64));
    validateur_mdp.ajouter(contient_majuscule());
    validateur_mdp.ajouter(contient_chiffre());

    let mots_de_passe = vec![
        "abc",
        "abcdefgh",
        "Abcdefgh",
        "Abcdefg1",
        "Tr3sB0nMotDePasse",
    ];

    for mdp in &mots_de_passe {
        let erreurs = validateur_mdp.valider(&mdp.to_string());
        if erreurs.is_empty() {
            println!("  '{}' : VALIDE", mdp);
        } else {
            println!("  '{}' : {} erreur(s)", mdp, erreurs.len());
            for err in &erreurs {
                println!("    - {}", err);
            }
        }
    }

    println!();
}

// --- 4.5 : Retry avec backoff ---

fn retry_avec_backoff<T, E, F>(
    max_tentatives: u32,
    delai_initial_ms: u64,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut(u32) -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut derniere_erreur: Option<E> = None;

    for tentative in 1..=max_tentatives {
        println!("    Tentative {}/{}...", tentative, max_tentatives);
        match operation(tentative) {
            Ok(val) => {
                println!("    Succes a la tentative {} !", tentative);
                return Ok(val);
            }
            Err(e) => {
                println!("    Echec : {}", e);
                if tentative < max_tentatives {
                    let delai = delai_initial_ms * 2u64.pow(tentative - 1);
                    println!("    Attente {}ms avant prochaine tentative...", delai);
                    thread::sleep(Duration::from_millis(delai));
                }
                derniere_erreur = Some(e);
            }
        }
    }

    Err(derniere_erreur.unwrap())
}

fn solution_4_5() {
    println!("--- 4.5 : Retry avec backoff ---");

    let mut appels = 0;
    let resultat = retry_avec_backoff(5, 50, |tentative| -> Result<String, String> {
        appels += 1;
        if tentative <= 3 {
            Err(format!("Erreur simulee (tentative {})", tentative))
        } else {
            Ok(format!("Donnees recues a la tentative {}", tentative))
        }
    });

    match resultat {
        Ok(val) => println!("  Resultat final : {}", val),
        Err(e) => println!("  Echec final : {}", e),
    }
    println!("  Nombre total d'appels : {}", appels);
}
