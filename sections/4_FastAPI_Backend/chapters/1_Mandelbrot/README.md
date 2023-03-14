# Mandelbrot

## 0 - Add dependencies

As we published our mandlebrot package to PyPI, we can now install it using `poetry`:

```bash
poetry add mandybrot
```

Alternatively, we could have installed it from a local directory:

```bash
poetry add mandybrot@../../../2_Python_Packages/chapters/7_Publish
```

## 1 - Import

We can now import the `mandybrot` package into our [`main.py`](./api/main.py) file:

```python
import mandybrot as mandy
```

## 2 - Add route

We can now add a route to our application that calls the `mandy.sample.point()` function:

```python
@app.get("/sample/{real}/{imag}")
async def sample_point(real: float, imag: float):
    max_iters = 100
    result = mandy.sample.point(real, imag, max_iters)
    return f"({real}, {imag}) -> {result}"
```

## 3 - Try it out

Make sure the application is running:

```bash
poetry run uvicorn api.main:app --reload --port 8000
```

And now we can visit a route like http://localhost:8000/sample/-0.761574/-0.0847596 to see the result of the `mandy.sample.point()` function.
