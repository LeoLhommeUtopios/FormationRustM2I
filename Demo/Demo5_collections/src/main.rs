// =============================================================
// DEMO 5 : Collections et Iterateurs
// =============================================================

use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    println!("=== DEMO 5 : Collections et Iterateurs ===\n");

    // ---------------------------------------------------------
    // 1. Vec<T>
    // ---------------------------------------------------------
    println!("--- Vec<T> ---");

    let mut nombres = vec![5, 3, 8, 1, 9, 2, 7];
    println!("  Initial : {:?}", nombres);

    nombres.push(6);
    nombres.sort();
    println!("  Trie : {:?}", nombres);

    nombres.retain(|&x| x > 3);
    println!("  Apres retain(>3) : {:?}", nombres);

    nombres.dedup();
    let tranche = &nombres[1..3];
    println!("  Tranche [1..3] : {:?}", tranche);

    // Methodes fonctionnelles
    let doubles: Vec<i32> = nombres.iter().map(|x| x * 2).collect();
    println!("  Doubles : {:?}", doubles);

    // windows et chunks
    let data = vec![1, 2, 3, 4, 5, 6];
    let moyennes_mobiles: Vec<f64> = data
        .windows(3)
        .map(|w| w.iter().sum::<i32>() as f64 / w.len() as f64)
        .collect();
    println!("  Moyennes mobiles (fenetre 3) : {:?}", moyennes_mobiles);

    // ---------------------------------------------------------
    // 2. HashMap<K, V>
    // ---------------------------------------------------------
    println!("\n--- HashMap<K, V> ---");

    let mut scores: HashMap<&str, Vec<u32>> = HashMap::new();
    scores.insert("Alice", vec![85, 92, 78]);
    scores.insert("Bob", vec![90, 88, 95]);
    scores.insert("Charlie", vec![72, 68, 80]);

    // Entry API
    scores
        .entry("Diana")
        .or_insert_with(Vec::new)
        .push(95);
    scores.entry("Diana").or_insert_with(Vec::new).push(88);

    for (nom, notes) in &scores {
        let moyenne: f64 = notes.iter().sum::<u32>() as f64 / notes.len() as f64;
        println!("  {} : {:?} (moy: {:.1})", nom, notes, moyenne);
    }

    // Compteur de mots
    let texte = "le chat le chien le chat un chien le poisson";
    let mut compteur: HashMap<&str, u32> = HashMap::new();
    for mot in texte.split_whitespace() {
        *compteur.entry(mot).or_insert(0) += 1;
    }
    println!("\n  Compteur de mots :");
    let mut mots_tries: Vec<_> = compteur.iter().collect();
    mots_tries.sort_by(|a, b| b.1.cmp(a.1));
    for (mot, count) in mots_tries {
        println!("    '{}' : {}", mot, count);
    }

    // ---------------------------------------------------------
    // 3. BTreeMap<K, V>
    // ---------------------------------------------------------
    println!("\n--- BTreeMap<K, V> (ordonne) ---");

    let mut evenements: BTreeMap<u32, &str> = BTreeMap::new();
    evenements.insert(2015, "Rust 1.0");
    evenements.insert(2021, "Rust Foundation");
    evenements.insert(2018, "Rust 2018 Edition");
    evenements.insert(2006, "Debut du projet");
    evenements.insert(2024, "Rust dans Linux kernel");

    println!("  Chronologie Rust :");
    for (annee, event) in &evenements {
        println!("    {} : {}", annee, event);
    }

    // Range queries
    println!("  Evenements 2015-2021 :");
    for (annee, event) in evenements.range(2015..=2021) {
        println!("    {} : {}", annee, event);
    }

    // ---------------------------------------------------------
    // 4. HashSet<T>
    // ---------------------------------------------------------
    println!("\n--- HashSet<T> ---");

    let langages_alice: HashSet<&str> =
        ["Rust", "Python", "Go", "TypeScript"].iter().cloned().collect();
    let langages_bob: HashSet<&str> =
        ["Java", "Python", "Rust", "C++"].iter().cloned().collect();

    println!("  Alice : {:?}", langages_alice);
    println!("  Bob : {:?}", langages_bob);
    println!(
        "  Communs (intersection) : {:?}",
        langages_alice.intersection(&langages_bob).collect::<Vec<_>>()
    );
    println!(
        "  Tous (union) : {:?}",
        langages_alice.union(&langages_bob).collect::<Vec<_>>()
    );
    println!(
        "  Uniques a Alice : {:?}",
        langages_alice.difference(&langages_bob).collect::<Vec<_>>()
    );

    // ---------------------------------------------------------
    // 5. Iterateurs en profondeur
    // ---------------------------------------------------------
    println!("\n--- Iterateurs avances ---");

    // chain
    let premiers = vec![1, 2, 3];
    let seconds = vec![4, 5, 6];
    let chaine: Vec<_> = premiers.iter().chain(seconds.iter()).collect();
    println!("  chain : {:?}", chaine);

    // zip
    let noms = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let personnes: Vec<_> = noms.iter().zip(ages.iter()).collect();
    println!("  zip : {:?}", personnes);

    // enumerate + filter_map
    let donnees = vec![Some(10), None, Some(30), None, Some(50)];
    let valides: Vec<(usize, i32)> = donnees
        .iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|v| (i, v)))
        .collect();
    println!("  filter_map : {:?}", valides);

    // flat_map
    let phrases = vec!["Bonjour le monde", "Rust est genial"];
    let mots: Vec<&str> = phrases.iter().flat_map(|p| p.split_whitespace()).collect();
    println!("  flat_map : {:?}", mots);

    // scan (etat accumule visible a chaque etape)
    let cumul: Vec<i32> = (1..=5).scan(0, |state, x| {
        *state += x;
        Some(*state)
    }).collect();
    println!("  scan (somme cumulative) : {:?}", cumul);

    // ---------------------------------------------------------
    // 6. Iterateur personnalise
    // ---------------------------------------------------------
    println!("\n--- Iterateur personnalise ---");

    let fib: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("  Fibonacci (10 premiers) : {:?}", fib);

    let fib_pairs: Vec<u64> = Fibonacci::new()
        .filter(|x| x % 2 == 0)
        .take(5)
        .collect();
    println!("  Fibonacci pairs (5 premiers) : {:?}", fib_pairs);

    // ---------------------------------------------------------
    // 7. Exemple pratique : pipeline de donnees
    // ---------------------------------------------------------
    println!("\n--- Pipeline de donnees ---");

    let commandes = vec![
        Commande { produit: "Laptop", quantite: 2, prix_unitaire: 999.99 },
        Commande { produit: "Souris", quantite: 10, prix_unitaire: 29.99 },
        Commande { produit: "Clavier", quantite: 2, prix_unitaire: 79.99 },
        Commande { produit: "Ecran", quantite: 3, prix_unitaire: 499.99 },
        Commande { produit: "Cable USB", quantite: 50, prix_unitaire: 9.99 },
    ];

    // Pipeline : filtrer, transformer, agreger
    let rapport: Vec<String> = commandes
        .iter()
        .filter(|c| c.total() > 200.0)
        .map(|c| format!("  {} : {} x {:.2} = {:.2} EUR", c.produit, c.quantite, c.prix_unitaire, c.total()))
        .collect();

    let ca_total: f64 = commandes.iter().map(|c| c.total()).sum();
    let ca_filtre: f64 = commandes.iter().filter(|c| c.total() > 200.0).map(|c| c.total()).sum();

    println!("  Commandes > 200 EUR :");
    for ligne in &rapport {
        println!("{}", ligne);
    }
    println!("  CA total : {:.2} EUR", ca_total);
    println!("  CA filtre (>200) : {:.2} EUR", ca_filtre);

    // Grouper par tranche de prix
    let par_tranche: HashMap<&str, Vec<&Commande>> = commandes.iter().fold(
        HashMap::new(),
        |mut map, cmd| {
            let tranche = match cmd.total() as u32 {
                0..=99 => "< 100",
                100..=499 => "100-499",
                500..=999 => "500-999",
                _ => ">= 1000",
            };
            map.entry(tranche).or_insert_with(Vec::new).push(cmd);
            map
        },
    );
    println!("\n  Par tranche :");
    for (tranche, cmds) in &par_tranche {
        let produits: Vec<&str> = cmds.iter().map(|c| c.produit).collect();
        println!("    {} : {:?}", tranche, produits);
    }
}

// === Iterateur personnalise : Fibonacci ===

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let resultat = self.a;
        let nouveau = self.a + self.b;
        self.a = self.b;
        self.b = nouveau;
        Some(resultat)
    }
}

// === Struct pour pipeline ===

#[derive(Debug)]
struct Commande {
    produit: &'static str,
    quantite: u32,
    prix_unitaire: f64,
}

impl Commande {
    fn total(&self) -> f64 {
        self.quantite as f64 * self.prix_unitaire
    }
}
