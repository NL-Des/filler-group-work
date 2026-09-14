# Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

## Commande de lancement de deux robots :
`docker run -v "$(pwd)/solution":/filler/solution -it filler`
`./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
  
## Tests
Pour lancer les tests unitaires et vérifier la fonctionnalité du programme : `cargo test`

## Visualiseur terminal

Le visualiseur n'altère pas les coordonnées envoyées au moteur : dans un conteneur interactif, il s'affiche directement sur le terminal. Active-le lors du lancement du moteur :

`FILLER_VISUALIZE=1 ./linux_game_engine -q -f maps/map01 -p1 /filler/solution/filler/target/debug/filler -p2 linux_robots/bender`
