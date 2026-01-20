Anotaciones:

- [x] Extraer los títulos de los libros. Para sacar los títulos de los libros en las
      notas, parece que corresponde con la primera fila del documento, y después de los
      10 elementos '='.

- [ ] Mostrar menú con los títulos y el indice para que el usuario elija que libro tener
      en cuenta para el resto del proceso.

- [ ] Una vez que el usuario ha seleccionado una opción, la idea es eliminar el
      contenido que no corresponde con el título.

- [ ] Eliminado el cotenido, luego tenemos que concatenar toda la información relevante
      junta, eliminando indicadores de páginas, y otros formatos que se añaden al texto,
      para que solo quede texto con información en si.

- [ ] Una vez obtenida la información concatenada, igual no tiene mucho sentido, pero se
      podría exportar el documento en un txt y que lo procese posteriormente un LLM para
      generar un texto enriquecido. Por lo que las notas sería parte del contexto e
      información.

Otras anotaciones:

Por lo que veo el formato es siempre el mismo:

```
Título del libro (Autor)
- Mi subrayado en la página 12 | Posición 185-186 | Añadido el viernes, 7 de junio de 2024 22:47:03

Texto del libro
==========
```

Por tanto:

+ LA primera fila es el titulo de libro/autor.
+ La segunda fila es informacion del subrayado.
+ La tercera fila es un espacio en blanco.
+ La cuarta fila es la informacion del subrayado.
+ LA quitan fila es un delimitador

Para quedarme con informacion de