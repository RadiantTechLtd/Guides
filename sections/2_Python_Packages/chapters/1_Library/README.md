# Adding Python

## 0 - Add sample.py

Let's start by adding a [`sample.py`](./mandy/sample.py) file to our project:

```bash
touch mandy/sample.py
```

## 1 - Link it to the package

In the [`__init__.py`](./mandy/__init__.py) file, we'll add a line that imports the `sample` sub-module:

```python
from . import sample
```

---

**NOTE**

`Python` contains a built-in `complex` type, which is used to represent complex numbers.
We can create a complex number by adding a `real` and an `imaginary` component together:

```python
1 + 2j
```

The `j` suffix is used to indicate that the number is imaginary.

---

## 2 - Add sample.point() function

Inside `sample.py` we'll add a function that calculates the number of iterations required to determine whether a given point location on the complex plane is within the Mandelbrot set:

```python
def point(real, imag, max_iters):
    c = real + imag * 1j
    z = 0j

    for i in range(max_iters):
        z = z * z + c

        if abs(z) > 2.0:
            return i

    return max_iters
```

You can read more about what this is actually calculating in the details sections below, but for now just know that this function takes in a complex number, and returns an integer between 0 and `max_iters`.

## 3 - Try it

We can try out this function by running the following code in a `Python` REPL:

```bash
poetry run python
```

```python
>>> import mandy
>>> mandy.sample.point(0, 0, 100)
100
```

REPL stands for "Read-Eval-Print-Loop", and it's a great way to test out code interactively.

## Details

The Mandelbrot set is a complex mathematical object that represents a specific set of complex numbers.
It is named after Benoit Mandelbrot, who studied it extensively in the 1970s and 1980s.

The Mandelbrot set is created by iterating a simple mathematical function, $z = z^2 + c$, where $z$ and $c$ are complex numbers.
Starting with a value of $c$, the function is repeatedly applied to $z$, and if the value of $z$ remains bounded as the iteration continues, then the value of $c$ is said to belong to the Mandelbrot set.
If the value of $z$ grows infinitely large, then $c$ is not part of the set.

The Mandelbrot set is visually represented as a fractal shape in the complex plane, with intricate patterns that repeat at smaller and smaller scales.
The set is connected and has a complex boundary, and the intricate shapes and patterns that emerge from it are visually stunning.
You'll see this for yourself soon!

The Mandelbrot set has many interesting mathematical properties, and it has been studied extensively in the field of complex dynamics.
It is also a popular subject of study and exploration for mathematicians, computer scientists, and artists, who have created many beautiful images and animations of the set.
