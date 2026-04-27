# Exercice 2 : Ownership et Borrowing

## Objectifs

- Comprendre la semantique move vs copy
- Maitriser les emprunts immutables et mutables
- Travailler avec les slices et les lifetimes

---

## Exercice 2.1 — Diagnostic d'ownership

Pour chaque extrait de code ci-dessous, indiquez s'il compile ou non. Si non, expliquez pourquoi et proposez une correction.

**Extrait A :**
```rust
fn main() {
    let s = String::from("hello");
    let t = s;
    println!("{} {}", s, t);
}
```

**Extrait B :**
```rust
fn afficher(s: String) {
    println!("{}", s);
}

fn main() {
    let msg = String::from("Bonjour");
    afficher(msg);
    afficher(msg);
}
```

**Extrait C :**
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let premier = &v[0];
    v.push(4);
    println!("{}", premier);
}
```

**Extrait D :**
```rust
fn main() {
    let x = 42;
    let y = x;
    println!("{} {}", x, y);
}
```

---

## Exercice 2.2 — Fonctions avec emprunts

Ecrivez les fonctions suivantes en utilisant les emprunts (pas de move) :

1. `compter_voyelles(s: &str) -> usize` — compte le nombre de voyelles (a, e, i, o, u) dans une chaine
2. `inverser_mots(s: &str) -> String` — inverse l'ordre des mots dans une phrase
3. `tronquer(s: &mut String, max: usize)` — tronque la chaine a `max` caracteres si elle est plus longue, en ajoutant "..." a la fin

Testez chaque fonction dans le `main`.

---

## Exercice 2.3 — Slices et recherche

Ecrivez les fonctions suivantes qui travaillent avec des slices :

1. `trouver_max(nombres: &[i32]) -> Option<&i32>` — retourne une reference vers le maximum
2. `est_trie(nombres: &[i32]) -> bool` — verifie si le slice est trie en ordre croissant
3. `elements_communs<'a>(a: &'a [i32], b: &[i32]) -> Vec<&'a i32>` — retourne les references vers les elements de `a` qui sont aussi dans `b`

---

## Exercice 2.4 — Struct avec emprunts

Creez un struct `Inventaire` qui possede un `Vec<Produit>` ou `Produit` est :

```rust
struct Produit {
    nom: String,
    prix: f64,
    quantite: u32,
}
```

Implementez ces methodes sur `Inventaire` :

1. `ajouter(&mut self, produit: Produit)`
2. `rechercher(&self, nom: &str) -> Option<&Produit>` — recherche par nom (emprunt immutable)
3. `modifier_prix(&mut self, nom: &str, nouveau_prix: f64) -> bool` — modifie le prix, retourne `true` si trouve
4. `les_plus_chers(&self, n: usize) -> Vec<&Produit>` — retourne les n produits les plus chers
5. `valeur_totale(&self) -> f64` — retourne la somme de (prix × quantite) pour tous les produits

---

## Exercice 2.5 — Lifetimes

Ecrivez une struct `Cache<'a>` qui stocke une reference vers une donnee et un timestamp :

```rust
struct Cache<'a> {
    donnee: &'a str,
    timestamp: u64,
}
```

Implementez :

1. `new(donnee: &'a str, timestamp: u64) -> Self`
2. `est_expire(&self, maintenant: u64, duree_vie: u64) -> bool`
3. Une fonction libre `selectionner_cache<'a>(caches: &[Cache<'a>]) -> Option<&Cache<'a>>` qui retourne le cache le plus recent non expire (utiliser maintenant = 1000, duree_vie = 500).

Creez un `main` qui demontre le bon fonctionnement avec des caches valides et expires.
