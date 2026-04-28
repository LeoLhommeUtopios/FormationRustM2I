# Exercice 4 : Closures et Ordre Superieur

## Objectifs

- Ecrire des closures avec capture par reference, mutable et move
- Comprendre Fn, FnMut, FnOnce
- Construire des fonctions d'ordre superieur

---

## Exercice 4.1 — Closures de base

Ecrivez les closures suivantes et testez-les :

1. Une closure `carre` qui prend un `i32` et retourne son carre
2. Une closure `est_pair` qui prend un `i32` et retourne un `bool`
3. Une closure `saluer` qui capture une variable `nom: String` par reference et affiche un message de bienvenue
4. Une closure mutable `compteur` qui capture un `mut count: u32` et l'incremente a chaque appel, retournant la nouvelle valeur

Verifiez que `nom` est toujours utilisable apres l'appel de `saluer`.

---

## Exercice 4.2 — Fonctions d'ordre superieur

Ecrivez les fonctions suivantes :

1. `appliquer_a_tous(nombres: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32>` — applique `f` a chaque element
2. `filtrer(nombres: &[i32], predicat: impl Fn(&i32) -> bool) -> Vec<i32>` — garde les elements qui satisfont le predicat
3. `reduire(nombres: &[i32], init: i32, f: impl Fn(i32, i32) -> i32) -> i32` — reduit le slice a une valeur unique
4. `composer<F, G>(f: F, g: G) -> impl Fn(i32) -> i32` ou `F: Fn(i32) -> i32` et `G: Fn(i32) -> i32` — retourne une closure qui applique d'abord `f` puis `g`

Testez :
```
appliquer_a_tous(&[1,2,3], |x| x * 2)        => [2, 4, 6]
filtrer(&[1,2,3,4,5], |x| x % 2 == 0)        => [2, 4]
reduire(&[1,2,3,4,5], 0, |acc, x| acc + x)   => 15
composer(|x| x + 1, |x| x * 2)(5)            => 12
```

---

## Exercice 4.3 — Pipeline de transformations

Creez un struct `Pipeline<T>` generique qui enchaine des transformations :

```rust
struct Pipeline<T> {
    valeur: T,
}
```

Implementez :
- `new(valeur: T) -> Self`
- `puis<U, F: FnOnce(T) -> U>(self, f: F) -> Pipeline<U>` — applique la transformation et retourne un nouveau pipeline
- `resultat(self) -> T` — retourne la valeur finale

Exemple d'utilisation :
```rust
let resultat = Pipeline::new("  Hello World  ".to_string())
    .puis(|s| s.trim().to_string())
    .puis(|s| s.to_uppercase())
    .puis(|s| format!(">>> {} <<<", s))
    .resultat();
// ">>> HELLO WORLD <<<"
```

---

## Exercice 4.4 — Validateurs fonctionnels

Creez un systeme de validation fonctionnel :

```rust
type Validateur<T> = Box<dyn Fn(&T) -> Result<(), String>>;
```

Ecrivez une struct `ValidateurCompose<T>` qui stocke un `Vec<Validateur<T>>` et implementez :

1. `ajouter(&mut self, validateur: Validateur<T>)`
2. `valider(&self, valeur: &T) -> Vec<String>` — retourne la liste des erreurs

Creez des fonctions factory pour les validateurs courants :
- `longueur_min(n: usize) -> Validateur<String>`
- `longueur_max(n: usize) -> Validateur<String>`
- `contient_majuscule() -> Validateur<String>`
- `contient_chiffre() -> Validateur<String>`

Testez en validant des mots de passe.

---

## Exercice 4.5 — Retry avec backoff

Ecrivez une fonction `retry_avec_backoff` :

```rust
fn retry_avec_backoff<T, E, F>(
    max_tentatives: u32,
    delai_initial_ms: u64,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut(u32) -> Result<T, E>,
    E: std::fmt::Display,
```

La fonction doit :
- Appeler `operation` avec le numero de tentative (1-indexed)
- En cas d'echec, attendre `delai_initial_ms * 2^(tentative - 1)` millisecondes
- Afficher un message a chaque tentative et chaque echec
- Retourner le premier succes ou la derniere erreur

Testez avec une operation qui echoue les 3 premieres fois puis reussit.
