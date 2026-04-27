// =============================================================
// SOLUTIONS — Exercice 2 : Ownership et Borrowing
// =============================================================

fn main() {
    println!("=== Solution Exercice 2 ===\n");
    solution_2_1();
    solution_2_2();
    solution_2_3();
    solution_2_4();
    solution_2_5();
}

// --- 2.1 : Diagnostic d'ownership ---

fn solution_2_1() {
    println!("--- 2.1 : Diagnostic ---");

    // Extrait A : NE COMPILE PAS (move de s vers t)
    // Correction : utiliser clone() ou des references
    let s = String::from("hello");
    let t = s.clone();
    println!("  A (corrige) : {} {}", s, t);

    // Extrait B : NE COMPILE PAS (msg est move au premier appel)
    // Correction : passer par reference
    fn afficher(s: &str) {
        println!("  B : {}", s);
    }
    let msg = String::from("Bonjour");
    afficher(&msg);
    afficher(&msg);

    // Extrait C : NE COMPILE PAS (emprunt immutable + mutation)
    // Correction : utiliser premier avant le push
    let mut v = vec![1, 2, 3];
    let premier = v[0]; // Copie (i32 est Copy)
    v.push(4);
    println!("  C (corrige) : premier = {}, v = {:?}", premier, v);

    // Extrait D : COMPILE (i32 est Copy)
    let x = 42;
    let y = x;
    println!("  D : {} {}", x, y);

    println!();
}

// --- 2.2 : Fonctions avec emprunts ---

fn compter_voyelles(s: &str) -> usize {
    s.chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}

fn inverser_mots(s: &str) -> String {
    let mots: Vec<&str> = s.split_whitespace().collect();
    let mut inverse = mots;
    inverse.reverse();
    inverse.join(" ")
}

fn tronquer(s: &mut String, max: usize) {
    if s.chars().count() > max {
        let tronque: String = s.chars().take(max).collect();
        *s = format!("{}...", tronque);
    }
}

fn solution_2_2() {
    println!("--- 2.2 : Fonctions avec emprunts ---");

    let texte = "Bonjour le monde de Rust";
    println!("  Voyelles dans '{}' : {}", texte, compter_voyelles(texte));
    println!("  Mots inverses : '{}'", inverser_mots(texte));

    let mut long = String::from("Une chaine tres longue qui depasse la limite");
    println!("  Avant tronquer : '{}'", long);
    tronquer(&mut long, 20);
    println!("  Apres tronquer : '{}'", long);

    println!();
}

// --- 2.3 : Slices et recherche ---

fn trouver_max(nombres: &[i32]) -> Option<&i32> {
    if nombres.is_empty() {
        return None;
    }
    let mut max = &nombres[0];
    for n in &nombres[1..] {
        if n > max {
            max = n;
        }
    }
    Some(max)
}

fn est_trie(nombres: &[i32]) -> bool {
    nombres.windows(2).all(|w| w[0] <= w[1])
}

fn elements_communs<'a>(a: &'a [i32], b: &[i32]) -> Vec<&'a i32> {
    a.iter().filter(|&&x| b.contains(&x)).collect()
}

// fn elements_communs<'a>(a: &'a [i32], b: &[i32]) -> Vec<&'a i32> {
//     let mut result = Vec::new();
//     for x in a {
//         if b.contains(x) {
//             result.push(x);
//         }
//     }
//     result
// }

fn solution_2_3() {
    println!("--- 2.3 : Slices ---");

    let nums = [3, 7, 1, 9, 4, 6];
    println!("  Max de {:?} : {:?}", nums, trouver_max(&nums));

    let trie = [1, 2, 3, 4, 5];
    let pas_trie = [1, 3, 2, 4, 5];
    println!("  {:?} est trie ? {}", trie, est_trie(&trie));
    println!("  {:?} est trie ? {}", pas_trie, est_trie(&pas_trie));

    let a = [1, 2, 3, 4, 5];
    let b = [3, 5, 7, 9];
    let communs = elements_communs(&a, &b);
    println!("  Communs entre {:?} et {:?} : {:?}", a, b, communs);

    println!();
}

// --- 2.4 : Struct Inventaire ---

#[derive(Debug)]
struct Produit {
    nom: String,
    prix: f64,
    quantite: u32,
}

struct Inventaire {
    produits: Vec<Produit>,
}

impl Inventaire {
    fn new() -> Self {
        Inventaire {
            produits: Vec::new(),
        }
    }

    fn ajouter(&mut self, produit: Produit) {
        self.produits.push(produit);
    }

    fn rechercher(&self, nom: &str) -> Option<&Produit> {
        self.produits.iter().find(|p| p.nom == nom)
    }

    fn modifier_prix(&mut self, nom: &str, nouveau_prix: f64) -> bool {
        if let Some(produit) = self.produits.iter_mut().find(|p| p.nom == nom) {
            produit.prix = nouveau_prix;
            true
        } else {
            false
        }
    }

    fn les_plus_chers(&self, n: usize) -> Vec<&Produit> {
        let mut refs: Vec<&Produit> = self.produits.iter().collect();
        refs.sort_by(|a, b| b.prix.partial_cmp(&a.prix).unwrap());
        refs.into_iter().take(n).collect()
    }

    fn valeur_totale(&self) -> f64 {
        self.produits
            .iter()
            .map(|p| p.prix * p.quantite as f64)
            .sum()
    }
}

fn solution_2_4() {
    println!("--- 2.4 : Inventaire ---");

    let mut inv = Inventaire::new();
    inv.ajouter(Produit { nom: "Laptop".into(), prix: 999.99, quantite: 5 });
    inv.ajouter(Produit { nom: "Souris".into(), prix: 29.99, quantite: 50 });
    inv.ajouter(Produit { nom: "Ecran".into(), prix: 499.99, quantite: 10 });
    inv.ajouter(Produit { nom: "Clavier".into(), prix: 79.99, quantite: 30 });

    if let Some(p) = inv.rechercher("Laptop") {
        println!("  Trouve : {} a {:.2} EUR", p.nom, p.prix);
    }

    inv.modifier_prix("Souris", 24.99);
    println!("  Prix souris modifie : {:.2}", inv.rechercher("Souris").unwrap().prix);

    println!("  Top 2 plus chers :");
    for p in inv.les_plus_chers(2) {
        println!("    {} : {:.2} EUR", p.nom, p.prix);
    }

    println!("  Valeur totale : {:.2} EUR", inv.valeur_totale());
    println!();
}

// --- 2.5 : Lifetimes avec Cache ---

#[derive(Debug)]
struct Cache<'a> {
    donnee: &'a str,
    timestamp: u64,
}

impl<'a> Cache<'a> {
    fn new(donnee: &'a str, timestamp: u64) -> Self {
        Cache { donnee, timestamp }
    }

    fn est_expire(&self, maintenant: u64, duree_vie: u64) -> bool {
        maintenant > self.timestamp + duree_vie
    }
}

fn selectionner_cache<'a>(caches: &'a [Cache<'a>], maintenant: u64, duree_vie: u64) -> Option<&'a Cache<'a>> {
    caches
        .iter()
        .filter(|c| !c.est_expire(maintenant, duree_vie))
        .max_by_key(|c| c.timestamp)
}

fn solution_2_5() {
    println!("--- 2.5 : Cache avec lifetimes ---");

    let data1 = String::from("donnee_recente");
    let data2 = String::from("donnee_ancienne");
    let data3 = String::from("donnee_expiree");

    let caches = vec![
        Cache::new(&data1, 900),
        Cache::new(&data2, 700),
        Cache::new(&data3, 200), // expire (200 + 500 < 1000)
    ];

    let maintenant = 1000;
    let duree_vie = 500;

    for cache in &caches {
        println!(
            "  {:?} - expire ? {}",
            cache,
            cache.est_expire(maintenant, duree_vie)
        );
    }

    if let Some(meilleur) = selectionner_cache(&caches, maintenant, duree_vie) {
        println!("  Meilleur cache : {:?}", meilleur);
    }
}
