# Exercice 5 : Collections et Iterateurs

## Objectifs

- Maitriser Vec, HashMap, BTreeMap et HashSet
- Chainer des operations sur les iterateurs
- Implementer des algorithmes avec les collections

---

## Exercice 5.1 — Manipulation de Vec

Ecrivez les fonctions suivantes :

1. `dedup_stable(v: Vec<i32>) -> Vec<i32>` — supprime les doublons en gardant l'ordre d'apparition (sans utiliser `dedup()` ni `HashSet`)
2. `rotation_gauche(v: &mut Vec<i32>, k: usize)` — effectue une rotation a gauche de k positions (ex: [1,2,3,4,5] avec k=2 donne [3,4,5,1,2])
3. `intercaler(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>` — intercale les elements des deux vecteurs (ex: [1,2,3] et [a,b,c,d] donne [1,a,2,b,3,c,d])

---

## Exercice 5.2 — HashMap avance

Ecrivez un programme qui analyse un texte :

1. Comptez la frequence de chaque mot (insensible a la casse, en retirant la ponctuation)
2. Trouvez les 5 mots les plus frequents
3. Calculez la longueur moyenne des mots
4. Regroupez les mots par leur premiere lettre dans un `HashMap<char, Vec<String>>`

Utilisez ce texte de test :
```
"Rust est un langage de programmation. Rust est performant. 
Rust est sur. Le langage Rust est moderne et le compilateur Rust 
aide le developpeur a ecrire du code sur et performant."
```

---

## Exercice 5.3 — Chaines d'iterateurs

En utilisant **uniquement** des operations d'iterateurs (pas de boucles `for`), ecrivez :

1. A partir de `vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`, produisez un `Vec<String>` contenant "Fizz" pour les multiples de 3, "Buzz" pour les multiples de 5, "FizzBuzz" pour les multiples de 15, et le nombre en string sinon.
2. A partir de deux `Vec<i32>`, calculez le produit scalaire (somme des produits element par element) en une seule chaine d'iterateurs.
3. Generez les 10 premiers nombres premiers en utilisant un iterateur avec `filter`.
4. A partir d'un `Vec<Vec<i32>>` (matrice), calculez la somme de chaque ligne et la somme totale, en une seule expression.

---

## Exercice 5.4 — Iterateur personnalise : RLE

Implementez un iterateur pour le Run-Length Encoding (RLE) :

Creez un struct `RleEncoder<I>` qui prend un iterateur d'elements `PartialEq` et produit des paires `(element, count)`.

Exemple :
```
[1, 1, 1, 2, 2, 3, 1, 1] -> [(1, 3), (2, 2), (3, 1), (1, 2)]
"aaabbbccda" -> [('a', 3), ('b', 3), ('c', 2), ('d', 1), ('a', 1)]
```

Ecrivez aussi une fonction `rle_decode` qui fait l'inverse.

---

## Exercice 5.5 — Index inverse

Construisez un index inverse (comme un moteur de recherche simplifie) :

Creez un struct `IndexInverse` qui :
1. Indexe des documents (chaque document est un `(id: u32, contenu: String)`)
2. Permet de rechercher des mots et retourne les IDs des documents qui les contiennent
3. Supporte la recherche multi-mots (AND : tous les mots doivent etre presents)

Utilisez `HashMap<String, HashSet<u32>>` comme structure interne.

Testez avec au moins 5 documents et plusieurs requetes.
