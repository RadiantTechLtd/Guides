# Plots

So far we have a function that can calculate the number of iterations for a given point.
We can use this to create an image by looping over an array of points, calculating the number of iterations for each point and then plotting the result.

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
def area(real, imag, width, height, scale, max_iters):
    re = np.linspace(real - 0.5 * scale * width, real + 0.5 * scale * width, width)
    im = np.linspace(imag - 0.5 * scale * height, imag + 0.5 * scale * height, height)

    mandelbrot_set = np.zeros((height, width))

    for i in range(height):
        for j in range(width):
            mandelbrot_set[i, j] = point(re[j], im[i], max_iters)

    return mandelbrot_set
```

## 2 - Try it out

We can try out this function by running the following code in a `Python` REPL:

```bash
poetry run python -c "import mandy; print(mandy.sample.area(-0.5, 0, 17, 17, 0.2, 99))"
```

You should see an array of numbers printed looking like this:

```
[[ 0.  0.  0.  0.  0.  0.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.  0.]
 [ 0.  0.  0.  0.  0.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.]
 [ 0.  0.  0.  0.  1.  1.  1.  2.  2.  2.  1.  1.  1.  1.  1.  1.  1.]
 [ 0.  0.  0.  1.  2.  2.  2.  2.  3.  4.  5.  3.  2.  1.  1.  1.  1.]
 [ 0.  0.  1.  2.  2.  2.  2.  3.  4. 18. 19.  4.  3.  2.  1.  1.  1.]
 [ 0.  0.  2.  2.  2.  2.  3.  5. 19. 99. 99. 45. 14.  2.  2.  1.  1.]
 [ 0.  0.  2.  2.  4.  5.  5.  8. 99. 99. 99. 99. 42.  3.  2.  1.  1.]
 [ 0.  1.  3.  4.  6. 99. 99. 99. 99. 99. 99. 99. 99.  3.  2.  1.  1.]
 [ 0. 99. 99. 99. 99. 99. 99. 99. 99. 99. 99. 99.  7.  3.  2.  2.  1.]
 [ 0.  1.  3.  4.  6. 99. 99. 99. 99. 99. 99. 99. 99.  3.  2.  1.  1.]
 [ 0.  0.  2.  2.  4.  5.  5.  8. 99. 99. 99. 99. 42.  3.  2.  1.  1.]
 [ 0.  0.  2.  2.  2.  2.  3.  5. 19. 99. 99. 45. 14.  2.  2.  1.  1.]
 [ 0.  0.  1.  2.  2.  2.  2.  3.  4. 18. 19.  4.  3.  2.  1.  1.  1.]
 [ 0.  0.  0.  1.  2.  2.  2.  2.  3.  4.  5.  3.  2.  1.  1.  1.  1.]
 [ 0.  0.  0.  0.  1.  1.  1.  2.  2.  2.  1.  1.  1.  1.  1.  1.  1.]
 [ 0.  0.  0.  0.  0.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.]
 [ 0.  0.  0.  0.  0.  0.  1.  1.  1.  1.  1.  1.  1.  1.  1.  1.  0.]]
```
