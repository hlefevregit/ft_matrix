# Exercice 07 — Multiplication matrice-vecteur et matrice-matrice

Une matrice peut transformer un vecteur ou une autre matrice.

## Principe mathématique

Pour une matrice A et un vecteur x :

A × x = y

On calcule chaque composante de y comme une somme :

yi = Σ A[i][j] × x[j]

## Exemple visuel

Si A est diagonale :

A = [[2, 0], [0, 2]]

alors A × (4, 2) = (8, 4)

On dit que la matrice “étire” les coordonnées.

Pour multiplier deux matrices, on applique la même idée sur les lignes de la première et les colonnes de la seconde.

## Pourquoi c’est important ?

C’est le cœur des transformations linéaires : rotation, échelle, projection, etc.
