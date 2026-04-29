// =============================================================
// SOLUTIONS — Exercice 5 : Collections et Iterateurs
// =============================================================

use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== Solution Exercice 5 ===\n");
    solution_5_1();
    solution_5_2();
    solution_5_3();
    solution_5_4();
    solution_5_5();
}

// --- 5.1 : Manipulation de Vec ---

fn dedup_stable(v: Vec<i32>) -> Vec<i32> {
    let mut resultat = Vec::new();
    for x in v {
        if !resultat.contains(&x) {
            resultat.push(x);
        }
    }
    resultat
}

fn rotation_gauche(v: &mut Vec<i32>, k: usize) {
    if v.is_empty() {
        return;
    }
    let k = k % v.len();
    let debut: Vec<i32> = v.drain(..k).collect();
    v.extend(debut);
}

fn intercaler(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut resultat = Vec::new();
    let mut ia = a.into_iter();
    let mut ib = b.into_iter();

    loop {
        match (ia.next(), ib.next()) {
            (Some(va), Some(vb)) => {
                resultat.push(va);
                resultat.push(vb);
            }
            (Some(va), None) => {
                resultat.push(va);
                resultat.extend(ia);
                break;
            }
            (None, Some(vb)) => {
                resultat.push(vb);
                resultat.extend(ib);
                break;
            }
            (None, None) => break,
        }
    }
    resultat
}

fn solution_5_1() {
    println!("--- 5.1 : Manipulation de Vec ---");

    let v = vec![1, 3, 2, 1, 4, 3, 5, 2];
    println!("  dedup_stable({:?}) = {:?}", v, dedup_stable(v.clone()));

    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("  Avant rotation : {:?}", v2);
    rotation_gauche(&mut v2, 2);
    println!("  Apres rotation(2) : {:?}", v2);

    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30, 40];
    let result= intercaler(a.clone(), b.clone());
    println!("  intercaler({:?}, {:?}) = {:?}", a, b, result);

    println!();
}

// --- 5.2 : HashMap avance ---

fn solution_5_2() {
    println!("--- 5.2 : Analyse de texte ---");

    let texte = "Rust est un langage de programmation. Rust est performant. \
                 Rust est sur. Le langage Rust est moderne et le compilateur Rust \
                 aide le developpeur a ecrire du code sur et performant.";

    // Nettoyer et compter
    let mots_propres: Vec<String> = texte
        .split_whitespace()
        .map(|m| m.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|m| !m.is_empty())
        .collect();

    // 1. Frequence de chaque mot
    let mut freq: HashMap<String, u32> = HashMap::new();
    for mot in &mots_propres {
        *freq.entry(mot.clone()).or_insert(0) += 1;
    }

    // 2. Top 5
    let mut freq_vec: Vec<_> = freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("  Top 5 mots :");
    for (mot, count) in freq_vec.iter().take(5) {
        println!("    '{}' : {}", mot, count);
    }

    // 3. Longueur moyenne
    let longueur_moy: f64 =
        mots_propres.iter().map(|m| m.len()).sum::<usize>() as f64 / mots_propres.len() as f64;
    println!("  Longueur moyenne des mots : {:.2}", longueur_moy);

    // 4. Grouper par premiere lettre
    let mut par_lettre: HashMap<char, Vec<String>> = HashMap::new();
    for mot in &mots_propres {
        if let Some(c) = mot.chars().next() {
            let entry = par_lettre.entry(c).or_insert_with(Vec::new);
            if !entry.contains(mot) {
                entry.push(mot.clone());
            }
        }
    }
    println!("  Mots par premiere lettre :");
    let mut lettres: Vec<_> = par_lettre.keys().collect();
    lettres.sort();
    for lettre in lettres {
        println!("    '{}' : {:?}", lettre, par_lettre[lettre]);
    }

    println!();
}

// --- 5.3 : Chaines d'iterateurs ---

fn solution_5_3() {
    println!("--- 5.3 : Chaines d'iterateurs ---");

    // 1. FizzBuzz
    let fizzbuzz: Vec<String> = (1..=15)
        .map(|n| match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => n.to_string(),
        })
        .collect();
    println!("  FizzBuzz : {:?}", fizzbuzz);

    // 2. Produit scalaire
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![2, 3, 4, 5, 6];
    let produit_scalaire: i32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    println!("  Produit scalaire {:?} . {:?} = {}", a, b, produit_scalaire);

    // 3. 10 premiers nombres premiers
    let premiers: Vec<u64> = (2u64..)
        .filter(|&n| {
            if n < 2 {
                return false;
            }
            let limite = (n as f64).sqrt() as u64;
            (2..=limite).all(|d| n % d != 0)
        })
        .take(10)
        .collect();
    println!("  10 premiers nombres premiers : {:?}", premiers);

    // 4. Somme de matrice
    let matrice = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let sommes_lignes: Vec<i32> = matrice.iter().map(|ligne| ligne.iter().sum()).collect();
    let somme_totale: i32 = sommes_lignes.iter().sum();
    println!("  Sommes par ligne : {:?}", sommes_lignes);
    println!("  Somme totale : {}", somme_totale);

    println!();
}

// --- 5.4 : RLE Encoder ---

struct RleEncoder<I: Iterator> {
    iter: I,
    courant: Option<I::Item>,
}

impl<I: Iterator> RleEncoder<I> {
    fn new(mut iter: I) -> Self {
        let courant = iter.next();
        RleEncoder { iter, courant }
    }
}

impl<I> Iterator for RleEncoder<I>
where
    I: Iterator,
    I::Item: PartialEq + Clone,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let courant = self.courant.take()?;
        let mut count = 1;

        loop {
            match self.iter.next() {
                Some(val) if val == courant => {
                    count += 1;
                }
                other => {
                    self.courant = other;
                    return Some((courant, count));
                }
            }
        }
    }
}

fn rle_decode<T: Clone>(encoded: &[(T, usize)]) -> Vec<T> {
    encoded
        .iter()
        .flat_map(|(val, count)| std::iter::repeat(val.clone()).take(*count))
        .collect()
}

fn solution_5_4() {
    println!("--- 5.4 : RLE ---");

    // Entiers
    let data = vec![1, 1, 1, 2, 2, 3, 1, 1];
    let encoded: Vec<_> = RleEncoder::new(data.iter().cloned()).collect();
    println!("  Encode {:?} -> {:?}", data, encoded);

    let decoded = rle_decode(&encoded);
    println!("  Decode -> {:?}", decoded);
    assert_eq!(data, decoded);

    // Caracteres
    let texte = "aaabbbccda";
    let encoded_chars: Vec<_> = RleEncoder::new(texte.chars()).collect();
    println!("  Encode '{}' -> {:?}", texte, encoded_chars);

    let decoded_str: String = rle_decode(&encoded_chars).into_iter().collect();
    println!("  Decode -> '{}'", decoded_str);
    assert_eq!(texte, decoded_str);

    println!();
}

// --- 5.5 : Index inverse ---

struct IndexInverse {
    index: HashMap<String, HashSet<u32>>,
    documents: HashMap<u32, String>,
}

impl IndexInverse {
    fn new() -> Self {
        IndexInverse {
            index: HashMap::new(),
            documents: HashMap::new(),
        }
    }

    fn indexer(&mut self, id: u32, contenu: &str) {
        self.documents.insert(id, contenu.to_string());

        for mot in contenu.split_whitespace() {
            let mot_propre = mot
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if !mot_propre.is_empty() {
                self.index
                    .entry(mot_propre)
                    .or_insert_with(HashSet::new)
                    .insert(id);
            }
        }
    }

    fn rechercher(&self, mot: &str) -> HashSet<u32> {
        self.index
            .get(&mot.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    fn rechercher_multi(&self, mots: &[&str]) -> HashSet<u32> {
        let mut resultats: Option<HashSet<u32>> = None;
        for mot in mots {
            let ids = self.rechercher(mot);
            resultats = Some(match resultats {
                Some(existant) => existant.intersection(&ids).cloned().collect(),
                None => ids,
            });
        }
        resultats.unwrap_or_default()
    }
}

fn solution_5_5() {
    println!("--- 5.5 : Index inverse ---");

    let mut index = IndexInverse::new();
    index.indexer(1, "Rust est un langage de programmation systeme");
    index.indexer(2, "Python est un langage de programmation interprete");
    index.indexer(3, "Rust offre la securite memoire sans garbage collector");
    index.indexer(4, "Python est populaire pour le machine learning");
    index.indexer(5, "Rust et Python sont des langages modernes");

    // Recherche simple
    let res = index.rechercher("rust");
    println!("  'rust' -> documents {:?}", res);

    let res = index.rechercher("programmation");
    println!("  'programmation' -> documents {:?}", res);

    // Recherche multi-mots (AND)
    let res = index.rechercher_multi(&["rust", "securite"]);
    println!("  'rust' AND 'securite' -> documents {:?}", res);

    let res = index.rechercher_multi(&["langage", "programmation"]);
    println!("  'langage' AND 'programmation' -> documents {:?}", res);

    let res = index.rechercher_multi(&["python", "rust"]);
    println!("  'python' AND 'rust' -> documents {:?}", res);
}


