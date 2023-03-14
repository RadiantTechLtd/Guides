# Notebooks

## Create a notebook

We'll create an `/examples` directory to contain our notebooks.

```shell
mkdir examples
```

We can create a new notebook by running the following command:

```shell
poetry run jupyter notebook examples
```

Although we could create notebooks anywhere, it's a good idea to keep them in a dedicated directory.
If we always keep our notebooks in a root directory named `\examples`, we'll always know where to find them when we revisit an old project.

## Add dependencies

I'm going to write a notebook to plotting some interesting locations in the Mandelbrot set (because of course I am).
First I'll need to install the package we just created.

We can do this either from `pypi`:

```shell
poetry add mandybrot
```

Or from a local directory:

```shell
poetry add ../../../2_Python_Packages/chapters/7_Publish
```

## Try it out

Run the notebook server:

```shell
poetry run jupyter notebook examples --port 8888
```

And then import the `mandybrot` package:

```python
import mandybrot
```

Open https://localhost:8888 in your browser and open the `mandybrot.ipynb` notebook.
Get plotting!
