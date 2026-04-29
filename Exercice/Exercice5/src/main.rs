// =============================================================
// SOLUTIONS — Exercice 5 : Collections et Iterateurs
// =============================================================

use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== Solution Exercice 5 ===\n");
    solution_5_1();
    solution_5_2();
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

