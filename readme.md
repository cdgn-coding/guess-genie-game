# Guess Genie Game

Este repositorio contiene un juego de terminal cuyos participantes son el humano y el genio.
Tú, el humano, debes pensar en un animal y el genio hará sucesivas preguntas hasta adivinar el animal.
Puede suceder que el genio no conozca el animal, en ese caso, podrás enseñarle el nombre y el genio recordará
ese animal en futuros juegos. En el siguiente video se ve la demostración:

[![Alt text](https://img.youtube.com/vi/TpvyR2Pwqvc/0.jpg)](https://www.youtube.com/watch?v=TpvyR2Pwqvc)

Este juego se inspira en el [guessing game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) de la página oficial de Rust, en el cual el usuario debe adivinar una cadena de caracteres, así como también en el famoso juego *Akinator*, que utiliza un mecanismo muy similar al que se desarrolla en este repositorio.

## Funcionamiento

El juego está programado mediante el algoritmo de árbol de desición. La entrada de este algoritmo
es una lista de animales y características, la cual vive en un archivo `animals.json`. Cada nodo del árbol
consiste en una respuesta, una pregunta (característica), la rama de *sí* y la rama de *no*.

```rust
struct DesicionTreeNode {
    answer: Option<String>,
    characteristic: Option<String>,
    yes_branch: Option<Box<DesicionTreeNode>>,
    no_branch: Option<Box<DesicionTreeNode>>
}
```

Todos los nodos internos representan una pregunta respecto de una característica del animal. Dependiendo de la respuesta del jugador (si/no), se recorre el arbol por una rama u otra. Cuando se llega a un nodo hoja, el genio puede saber el animal perfectamente con el atributo `answer`.

Para desarrollar este arbol, se seleccionan características que tengan mayor varianza estadística de forma sucesiva para decidir qué preguntas hacer. Esto permite que el genio haga la menor cantidad de preguntas al jugador. Dado que las características son variables aleatorias booleanas, se utilizó la distribución de Bernouilli como referencia.

En consecuencia, para una característica cualquiera, la varianza se calcula como `n * p * (1 - p)`. Donde `p` es la probabilidad de que un animal tenga la característica. Cuando ya no quedan más características para desambiguar, se crea un nodo hoja con el animal que quede (se van descartando).

## License

Copyright 2022 Carlos David Gonzalez Nexans

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.