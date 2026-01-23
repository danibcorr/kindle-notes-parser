# Kindle Notes Parser

**Kindle Notes Parser** es una herramienta de CLI escrita en **Rust** diseñada para
extraer y filtrar las notas de tus libros de Kindle. Esta herramienta te permite
interactuar con tu archivo `.txt`, seleccionar un título específico y generar un archivo
limpio con tus anotaciones, ideal para ser procesado posteriormente por un LLM (ChatGPT,
Claude, etc.) o integrado en tu sistema de gestión de conocimiento.

## Instalación y Compilación

Asegúrate de tener instalado [Rust y Cargo](https://rustup.rs/).

1. **Clona el repositorio:**

   ```bash
   git clone https://github.com/danibcorr/kindle-notes-parser
   cd kindle-notes-parser
   ```

2. **Compila el proyecto:**

   ```bash
   cargo build
   ```

Encontrarás el ejecutable en `./target/debug/kindle-notes-parser`.

## Uso

Para iniciar el parseo de un archivo de notas, utiliza el comando `-p` seguido de la
ruta de tu archivo:

```bash
./target/debug/kindle-notes-parser -p /ruta/a/tus/anotaciones.txt
```

El contenido se guarda automáticamente en: `outputs/[Nombre del Libro].txt`

Este archivo contendrá únicamente los textos subrayados, separados por saltos de línea,
eliminando metadatos innecesarios como fechas o posiciones de página.
