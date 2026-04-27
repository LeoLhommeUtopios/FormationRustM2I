# Exercice 1 : Variables, Types et Fonctions

## Objectifs

- Manipuler les types de base en Rust
- Ecrire des fonctions avec differents types de retour
- Utiliser les expressions, le pattern matching et les conditions

---

## Exercice 1.1 — Conversions de temperature

Ecrivez deux fonctions :

- `celsius_vers_fahrenheit(c: f64) -> f64` qui convertit des degres Celsius en Fahrenheit (formule : F = C × 9/5 + 32)
- `fahrenheit_vers_celsius(f: f64) -> f64` qui fait l'inverse

Ecrivez un `main` qui affiche un tableau de conversion pour les temperatures suivantes : -40, 0, 20, 37, 100 degres Celsius.

**Sortie attendue** (approximative) :

```
   °C  |   °F
  -40  |  -40.0
    0  |   32.0
   20  |   68.0
   37  |   98.6
  100  |  212.0
```

---

## Exercice 1.2 — Analyse de notes

Creez une fonction `mention(note: u32) -> &'static str` qui retourne la mention correspondante :

| Note | Mention |
|---|---|
| 16 - 20 | "Tres bien" |
| 14 - 15 | "Bien" |
| 12 - 13 | "Assez bien" |
| 10 - 11 | "Passable" |
| 0 - 9 | "Insuffisant" |
| > 20 | "Note invalide" |

Utilisez un `match` pour implementer cette logique.

Ecrivez un `main` qui teste votre fonction avec les notes : 18, 14, 12, 8, 25.

---

## Exercice 1.3 — Fibonacci iteratif

Ecrivez une fonction `fibonacci(n: u32) -> u64` qui retourne le n-ieme nombre de Fibonacci de maniere iterative (sans recursion).

Rappel : F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2)

Affichez les 20 premiers nombres de Fibonacci dans le `main`.

---

## Exercice 1.4 — Struct et methodes

Creez un struct `Etudiant` avec les champs suivants :
- `nom: String`
- `notes: Vec<u32>`

Implementez les methodes suivantes :

- `new(nom: &str) -> Self`
- `ajouter_note(&mut self, note: u32)` — ajoute une note (ne fait rien si > 20)
- `moyenne(&self) -> f64` — retourne la moyenne des notes (0.0 si aucune note)
- `meilleure_note(&self) -> Option<u32>` — retourne la meilleure note
- `a_valide(&self) -> bool` — retourne `true` si la moyenne >= 10

Testez avec un `main` qui cree deux etudiants, ajoute des notes, et affiche leurs statistiques.

---

## Exercice 1.5 — Enum et Pattern Matching

Creez une enum `Operation` avec les variantes :
- `Addition(f64, f64)`
- `Soustraction(f64, f64)`
- `Multiplication(f64, f64)`
- `Division(f64, f64)`

Ecrivez une fonction `calculer(op: &Operation) -> Result<f64, String>` qui :
- Effectue le calcul correspondant
- Retourne une erreur `"Division par zero"` si la division est par zero

Testez avec plusieurs operations dans le `main`, en utilisant `match` pour afficher le resultat ou l'erreur.
