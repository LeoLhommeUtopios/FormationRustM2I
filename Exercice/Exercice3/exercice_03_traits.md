# Exercice 3 : Traits et Polymorphisme

## Objectifs

- Definir et implementer des traits
- Utiliser le polymorphisme statique et dynamique
- Implementer des traits standard (Display, From, Iterator)

---

## Exercice 3.1 — Trait Affichable

Definissez un trait `Affichable` avec les methodes :

- `en_texte(&self) -> String` (obligatoire)
- `en_json(&self) -> String` (methode par defaut qui retourne `{"value": "<en_texte()>"}`)

Implementez ce trait pour :
- `i32`
- `String`
- Un struct `Adresse { rue: String, ville: String, code_postal: String }`

Ecrivez une fonction `afficher_tout(elements: &[&dyn Affichable])` qui affiche chaque element en texte et en JSON.

---

## Exercice 3.2 — Systeme de formes avec aires

Definissez un trait `Mesurable` avec :
- `aire(&self) -> f64`
- `perimetre(&self) -> f64`
- `nom(&self) -> &str`

Creez les structs : `Carre`, `Cercle`, `TriangleRectangle` (avec base et hauteur).

Implementez `Mesurable` pour chacun, puis ecrivez :

1. `la_plus_grande(formes: &[Box<dyn Mesurable>]) -> Option<&dyn Mesurable>` — retourne la forme avec la plus grande aire
2. `trier_par_aire(formes: &mut [Box<dyn Mesurable>])` — trie les formes par aire croissante
3. `resume(formes: &[Box<dyn Mesurable>])` — affiche un tableau recapitulatif

---

## Exercice 3.3 — Trait generique : Transformable

Definissez un trait generique :

```rust
trait Transformable<T> {
    fn transformer(&self) -> T;
}
```

Implementez-le pour :
- `String` -> `Vec<char>` (explose en caracteres)
- `Vec<i32>` -> `i32` (somme des elements)
- `(f64, f64)` -> `f64` (distance euclidienne depuis l'origine)

Ecrivez une fonction generique `appliquer_transformation<T, U>(valeur: &T) -> U where T: Transformable<U>` et testez-la.

---

## Exercice 3.4 — Trait From et Into

Creez un struct `Temperature` et implementez les conversions :

```rust
struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);
```

Implementez `From<Celsius>` pour `Fahrenheit`, `From<Celsius>` pour `Kelvin`, et les conversions inverses.

Implementez egalement `std::fmt::Display` pour chaque type.

Ecrivez un `main` qui demontre les conversions dans les deux sens :

```rust
let c = Celsius(100.0);
let f: Fahrenheit = c.into();
let k: Kelvin = Celsius(0.0).into();
println!("{} = {} = {}", c, f, k);
```

---

## Exercice 3.5 — Trait Iterator personnalise

Creez un struct `SuiteSyracuse` qui genere la suite de Syracuse (ou suite de Collatz) :

- Si n est pair : n / 2
- Si n est impair : 3n + 1
- La suite s'arrete quand n == 1

Implementez le trait `Iterator` pour `SuiteSyracuse`.

Ecrivez un `main` qui :
1. Affiche la suite de Syracuse pour n = 27
2. Calcule la longueur de la suite (nombre de termes avant d'atteindre 1)
3. Trouve la valeur maximale atteinte
4. Parmi les nombres de 1 a 100, trouve celui qui produit la suite la plus longue
