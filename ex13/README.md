# Exercice 13 — Rang d’une matrice

Le rang d’une matrice mesure le nombre de directions indépendantes dans ses lignes ou ses colonnes.

## Principe mathématique

On regarde combien de lignes sont réellement utiles.

Si une ligne est un multiple d’une autre, elle ne crée pas une nouvelle direction.

## Exemple

[[1, 2], [2, 4]]

La deuxième ligne est 2 fois la première.

Donc le rang est 1, pas 2.

## Pourquoi c’est utile ?

Le rang dit si un système a une solution unique, plusieurs solutions, ou aucune solution.

C’est aussi un concept central en analyse de données et en vision par ordinateur.
