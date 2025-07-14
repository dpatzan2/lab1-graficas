# Lab 1: Filling any polygon 🎨

## Características ✨

- **Relleno de Polígonos** 🎯: Implementa algoritmo de relleno por líneas de barrido
- **Contornos de Polígonos** 🖊️: Dibuja bordes de polígonos con colores personalizables
- **Framebuffer** 🖼️: Manipulación manual de píxeles para operaciones gráficas

## Estructura del Proyecto 📁

```
src/
├── main.rs          # Aplicación principal con definiciones de polígonos
├── framebuffer.rs   # Implementación del framebuffer para operaciones de píxeles
└── polygon.rs       # Algoritmos de relleno de polígonos
```

## Salida 🖥️

El programa genera múltiples polígonos de colores con diferentes formas:

1. **Forma de Estrella Amarilla** ⭐ (poly1) - Polígono con múltiples vértices
2. **Cuadrilátero Azul** 🔷 (poly2) - Polígono simple de 4 lados
3. **Triángulo Rojo** 🔺 (poly3) - Forma triangular básica
4. **Polígono Verde Complejo** 🟢 (poly4) - Polígono grande con un agujero
5. **Agujero** 🕳️ (poly5) - Sustraído de poly4 para crear efecto de agujero

<img width="800" height="600" alt="out" src="https://github.com/user-attachments/assets/c75749de-bba9-484b-94c8-f8ad36897d87" />

## Dependencias 📦

- `raylib` - Para matemáticas vectoriales y definiciones de color
- `image` - Para salida de archivos PNG/BMP

## Uso 🚀

```bash
cargo run
```

Esto generará:
- `out.bmp` - Salida en formato BMP

