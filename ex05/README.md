# Exercice 05 — Angle entre deux vecteurs

On veut savoir à quel point deux vecteurs sont alignés.

## Principe mathématique

Le cosinus de l’angle θ entre u et v est :

cos(θ) = (u · v) / (||u|| × ||v||)

Le produit scalaire est divisé par les longueurs de chacun.

## Intuition

- si cos(θ) = 1 → même direction
- si cos(θ) = 0 → perpendiculaires
- si cos(θ) = -1 → directions opposées

## Exemple

u = (1, 0)
v = (0, 1)

u · v = 0

||u|| = 1, ||v|| = 1

cos(θ) = 0 → angle de 90°
