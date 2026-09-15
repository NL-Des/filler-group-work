# Filler

## Lancer le projet

Depuis la racine du dépôt, construire l'image puis ouvrir un terminal dans le conteneur :

```bash
docker build -t filler .
cargo build
docker run --rm -it -v "$(pwd)/solution":/filler/solution filler
```

Le binaire est alors disponible à `/filler/solution/filler/target/debug/filler`.

## Jouer une partie

Depuis le conteneur, après la compilation :

```bash
./linux_game_engine -q -f maps/map01 \
  -p1 /filler/solution/filler/target/debug/filler \
  -p2 linux_robots/bender
```

L'option `-q` masque les échanges bruts entre le moteur et les robots. Retirez-la pour les afficher.

### Jouer contre Terminator

Le bonus Terminator utilise le même binaire : aucune option particulière n'est à injecter. Le robot a été validé en joueur 1 avec cette commande reproductible :

```bash
./linux_game_engine -q -s 42 -f maps/map01 \
  -p1 /filler/solution/filler/target/debug/filler \
  -p2 linux_robots/terminator
```

`-s 42` fixe la seed du match.

## Visualiseur terminal

Activez le visualiseur avec `FILLER_VISUALIZE=1`. Conservez `-q`, sinon le moteur affiche aussi le plateau et l'affichage apparaît en double.

```bash
FILLER_VISUALIZE=1 ./linux_game_engine -q -f maps/map01 \
  -p1 /filler/solution/filler/target/debug/filler \
  -p2 linux_robots/bender
```

## Tests

Dans le conteneur :

```bash
cd /filler/solution/filler
cargo test
```

Sur Mac Apple Silicon, remplacez `linux_game_engine` et `linux_robots` par `m1_game_engine` et `m1_robots`.
