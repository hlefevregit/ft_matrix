# Exercice 02 — Interpolation linéaire

L’interpolation linéaire sert à “se déplacer progressivement” d’un point de départ à un point d’arrivée.

## Principe mathématique

On part de a et on va vers b selon un paramètre t entre 0 et 1 :

L(t) = (1 - t) × a + t × b

- si t = 0, on est au point a
- si t = 0.5, on est au milieu
- si t = 1, on est au point b

Exemple :

a = (2, 1)
b = (4, 2)
t = 0.3

L(0.3) = (1 - 0.3) × (2, 1) + 0.3 × (4, 2) = (2.6, 1.3)

## Pourquoi c’est utile ?

C’est la base de l’animation, de la morphing, des transitions et de nombreux calculs visuels.
