# Python Packages

`Python` code can be packaged into modules and libraries.
`Poetry` is a tool for managing and creating our own packages.

In this section we'll create a package for generating Mandelbrot plots, and finish off by publishing it to `PyPI`.

Each chapter will build on the previous one.
The `README.md` file in each chapter will contain a summary of the changes made in that chapter.

To explicitly see the changes made in each chapter, you can use `git diff`:

```bash
git diff chapters/0_Poetry_Init chapters/1_Adding_Python
```

## Chapters

-   [Init](./chapters/0_Init/README.md)
-   [Library](./chapters/1_Library/README.md)
-   [Scripts](./chapters/2_Scripts/README.md)

## Mandelbrot Set

We're going to be creating a package for generating Mandelbrot plots.
The Mandelbrot set is a cool mathematical pattern that is made by doing a simple calculation over and over again for different numbers.
If the calculation stays small enough, that number is part of the pattern.
If the calculation gets really big, that number isn't part of the pattern.
The pattern looks really cool and intricate, and it goes on forever in all directions.
People like to study it because it's interesting and beautiful.
