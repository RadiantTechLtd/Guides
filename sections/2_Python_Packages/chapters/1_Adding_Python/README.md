# Adding Python

Let's start by adding a `sample.py` file to our project:

```bash
touch mandelbrot/sample.py
```

Inside we'll add a function that will sample a given location on the complex plane, defined by a `real` and `imaginary` component:

```python
def sample(real, imag):
    return real + imag * 1j
```
