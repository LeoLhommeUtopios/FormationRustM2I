// =============================================================
// DEMO 6 : Concurrence sans peur
// =============================================================

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("=== DEMO 6 : Concurrence sans peur ===\n");

    // ---------------------------------------------------------
    // 1. Lancer des threads de base
    // ---------------------------------------------------------
    println!("--- Threads de base ---");

    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  [thread fils] iteration {}", i);
            thread::sleep(Duration::from_millis(50));
        }
        42 // Valeur de retour du thread
    });

    for i in 1..=3 {
        println!("  [thread main] iteration {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    let resultat = handle.join().unwrap();
    println!("  Thread fils a retourne : {}", resultat);

    // ---------------------------------------------------------
    // 2. Move closures et threads
    // ---------------------------------------------------------
    println!("\n--- Move closures ---");

    let noms = vec!["Alice", "Bob", "Charlie"];

    let handle = thread::spawn(move || {
        for nom in &noms {
            println!("  Bonjour, {} !", nom);
        }
        noms // Retourne l'ownership
    });

    let noms_retournes = handle.join().unwrap();
    println!("  Noms recuperes : {:?}", noms_retournes);

    // ---------------------------------------------------------
    // 3. Arc (Atomic Reference Counting)
    // ---------------------------------------------------------
    println!("\n--- Arc ---");

    let donnees = Arc::new(vec![1, 2, 3, 4, 5]);

    let handles: Vec<_> = (0..3)
        .map(|id| {
            let donnees = Arc::clone(&donnees);
            thread::spawn(move || {
                let somme: i32 = donnees.iter().sum();
                println!("  Thread {} : somme = {}", id, somme);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    println!("  References Arc restantes : {}", Arc::strong_count(&donnees));

    // ---------------------------------------------------------
    // 4. Mutex (exclusion mutuelle)
    // ---------------------------------------------------------
    println!("\n--- Arc + Mutex ---");

    let compteur = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for id in 0..5 {
        let compteur = Arc::clone(&compteur);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let mut val = compteur.lock().unwrap();
                *val += 1;
            }
            println!("  Thread {} termine", id);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("  Compteur final = {} (attendu: 500)", *compteur.lock().unwrap());

    // ---------------------------------------------------------
    // 5. RwLock (lecteurs multiples / ecrivain unique)
    // ---------------------------------------------------------
    println!("\n--- Arc + RwLock ---");

    let config = Arc::new(RwLock::new(Config {
        max_connexions: 100,
        timeout_ms: 5000,
        mode_debug: false,
    }));

    // Plusieurs lecteurs simultanes
    let mut handles = vec![];
    for id in 0..3 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let cfg = config.read().unwrap();
            println!(
                "  [Lecteur {}] max_conn={}, timeout={}ms",
                id, cfg.max_connexions, cfg.timeout_ms
            );
        }));
    }

    // Un ecrivain
    let config_w = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let mut cfg = config_w.write().unwrap();
        cfg.mode_debug = true;
        cfg.timeout_ms = 3000;
        println!("  [Ecrivain] Config mise a jour");
    }));

    for h in handles {
        h.join().unwrap();
    }
    let cfg = config.read().unwrap();
    println!(
        "  Config finale : debug={}, timeout={}ms",
        cfg.mode_debug, cfg.timeout_ms
    );

    // ---------------------------------------------------------
    // 6. Channels (mpsc)
    // ---------------------------------------------------------
    println!("\n--- Channels (mpsc) ---");

    let (tx, rx) = mpsc::channel();

    // Plusieurs producteurs
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            let messages = vec![
                format!("[{}] Premier message", id),
                format!("[{}] Deuxieme message", id),
            ];
            for msg in messages {
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
    }
    drop(tx); // Important : dropper l'emetteur original

    println!("  Messages recus :");
    for msg in rx {
        println!("    {}", msg);
    }

    // ---------------------------------------------------------
    // 7. Channel avec types structures
    // ---------------------------------------------------------
    println!("\n--- Channel structure ---");

    let (tx, rx) = mpsc::channel::<Tache>();

    // Producteur de taches
    thread::spawn(move || {
        let taches = vec![
            Tache::Calculer(vec![1, 2, 3, 4, 5]),
            Tache::Transformer("hello world".to_string()),
            Tache::Calculer(vec![10, 20, 30]),
            Tache::Arreter,
        ];
        for tache in taches {
            tx.send(tache).unwrap();
            thread::sleep(Duration::from_millis(30));
        }
    });

    // Consommateur
    loop {
        match rx.recv().unwrap() {
            Tache::Calculer(donnees) => {
                let somme: i32 = donnees.iter().sum();
                println!("  Calcul {:?} = {}", donnees, somme);
            }
            Tache::Transformer(texte) => {
                println!("  Transforme '{}' -> '{}'", texte, texte.to_uppercase());
            }
            Tache::Arreter => {
                println!("  Signal d'arret recu");
                break;
            }
        }
    }

    // ---------------------------------------------------------
    // 8. Exemple pratique : Map-Reduce parallele
    // ---------------------------------------------------------
    println!("\n--- Map-Reduce parallele ---");

    let texte = "le chat mange le poisson le chien dort le chat dort \
                 le poisson nage le chat mange le chien joue";

    let debut = Instant::now();
    let resultat = map_reduce_parallele(texte, 3);
    let duree = debut.elapsed();

    println!("  Comptage de mots (parallele, {} threads) :", 3);
    let mut mots_tries: Vec<_> = resultat.iter().collect();
    mots_tries.sort_by(|a, b| b.1.cmp(a.1));
    for (mot, count) in &mots_tries {
        println!("    '{}' : {}", mot, count);
    }
    println!("  Duree : {:?}", duree);
}

// === Types ===

#[derive(Debug)]
struct Config {
    max_connexions: u32,
    timeout_ms: u32,
    mode_debug: bool,
}

enum Tache {
    Calculer(Vec<i32>),
    Transformer(String),
    Arreter,
}

// === Map-Reduce ===

fn map_reduce_parallele(texte: &str, nb_threads: usize) -> HashMap<String, u32> {
    let mots: Vec<&str> = texte.split_whitespace().collect();
    let taille_chunk = (mots.len() + nb_threads - 1) / nb_threads;

    let (tx, rx) = mpsc::channel();

    for chunk in mots.chunks(taille_chunk) {
        let tx = tx.clone();
        let chunk: Vec<String> = chunk.iter().map(|s| s.to_string()).collect();

        thread::spawn(move || {
            // Phase Map : comptage local
            let mut local: HashMap<String, u32> = HashMap::new();
            for mot in chunk {
                *local.entry(mot).or_insert(0) += 1;
            }
            tx.send(local).unwrap();
        });
    }
    drop(tx);

    // Phase Reduce : fusion des resultats
    let mut global: HashMap<String, u32> = HashMap::new();
    for local in rx {
        for (mot, count) in local {
            *global.entry(mot).or_insert(0) += count;
        }
    }
    global
}
