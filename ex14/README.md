# Exercice 14 — Matrice de projection

Une matrice de projection sert à passer d’un espace 3D à un espace 2D (l’écran, par exemple).

## Principe mathématique

On veut représenter un objet en perspective. Le calcul utilise :

- le champ de vision (FOV),
- le ratio de l’écran,
- les distances near et far.

La matrice de projection transforme les coordonnées 3D en coordonnées homogènes qui seront ensuite converties en écran.

## Intuition

Les objets plus loin semblent plus petits. La projection “réduit” les distances selon la perspective.

C’est la base du rendu 3D dans les jeux, les moteurs graphiques et les simulations.
