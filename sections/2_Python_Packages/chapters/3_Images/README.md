# Images

So far we have a function that can calculate the number of iterations for a given point.
We can use this to create an image by looping over an array of points, calculating the number of iterations for each point and then turing those iterations into a colour.

## 0 - Add numpy

We'll use `numpy` arrays to store the image data.

```bash
poetry add numpy
```

## 1 - Add sample.area()

Within the [`sample.py`](./mandy/sample.py) file we'll add the `numpy` import:

```python
import numpy as np
```

And then at the bottom, we'll add a function called `area()`:

```python
def area(real, imag, width, height, max_iterations=100):
    re = np.linspace(real - 0.5 * width, real + 0.5 * width, width)
    im = np.linspace(imag - 0.5 * height, imag + 0.5 * height, height)

    mandelbrot_set = np.zeros((height, width))

    for i in range(height):
        for j in range(width):
            mandelbrot_set[i, j] = point(re[i], im[j], max_iterations)

    return mandelbrot_set
```

## 2 - Try it out

We can try out this function by running the following code in a `Python` REPL:

```bash
poetry run python -c "import mandy; print(mandy.sample.area(0, 0, 7, 7))"
```
