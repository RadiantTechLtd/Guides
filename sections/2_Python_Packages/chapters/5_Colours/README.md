# Colours

Let's add a better way to colour our Mandelbrot set.

## 0 - Add a colouring function

This one replicates the jet colour map from `matplotlib`.

```python
def jet_map(value, max_iters):
    v = value / max_iters

    r, g, b = 0, 0, 0

    if v < 0.125:
        r = 0
        g = 0
        b = 0.5 + 0.5 * (4 * v)
    elif v < 0.375:
        r = 0
        g = 4 * (v - 0.125)
        b = 1
    elif v < 0.625:
        r = 4 * (v - 0.375)
        g = 1
        b = 1 - 4 * (v - 0.375)
    elif v < 0.875:
        r = 1
        g = 1 - 4 * (v - 0.625)
        b = 0
    else:
        r = 1 - 0.5 * (v - 0.875)
        g = 0
        b = 0

    return (int(r * 255), int(g * 255), int(b * 255))
```

But you could use any function that takes a number between 0 and 1 and returns an 8-bit RGB colour!

## 1 - Update the script

Update the [run.py](./scripts/run.py) script to use the new colouring function.

```python
cols = mandy.colour.image(data, args.max_iters, mandy.colour.jet_map)
```

## 2 - Try it out

If we run the script now we'll see colours!

```bash
poetry run python scripts/run.py -0.761574 -0.0847596 1000 500 1e-4 200
```

Check out the wonders of $-0.761574 -0.0847596j$:

![Mandelbrot set](./mandy.png)
