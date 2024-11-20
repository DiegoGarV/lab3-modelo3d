# RENDERIZADO 3D EN RUST

para correr el código primero se debe poner el comando
```
cargo build --release
```

y luego usar este cada vez que se quiera observar la ventana con el render
```
cargo run --release
```

Con la ventana abierta utilizar las siguientes teclas para mover la nave
- Flechas: mueven la nave para todas las direcciones
- N y M: quitan y dan zoom al render, respectivamente
- A y D: rotación transversal al eje y
- W y S: rotación transversal al eje x
- Q y E: rotación transversal al eje z

Un ejemplo del render se puede encontrar dentro del GIF en el repositorio
[![nave video.gif](nave%20video.gif)](nave%20video.gif)