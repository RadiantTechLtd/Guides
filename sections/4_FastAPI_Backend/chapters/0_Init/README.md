# Poetry Init

## 0 - Add pyproject.toml

Create a new `Poetry` project:

```bash
poetry init
```

-   I'm going to call this project `mandy_api`
-   Starting at version `0.0.0`
-   My description will be: `Web-application for plotting the magical Mandelbrot set.`
-   I am: `FreddyWordingham <freddy@digilab.co.uk>`
-   I'm not going to add a license
-   Compatible with `Python` versions `3.10` and above
-   And I'm not going to add any dependencies right now

This will generate a [`pyproject.toml`](./pyproject.toml) file containing the project metadata.

## 1 - Add .gitignore

I'm going to add a [`.gitignore`](./.gitignore) file to my project.
`Python` projects use a lot of files that we don't want to commit to `git`, so we can use a template to generate a `.gitignore` file for us.

I'm going to pull the code from https://www.toptal.com/developers/gitignore/api/python into a `.gitignore` file using the `curl` command:

```bash
curl -L https://www.toptal.com/developers/gitignore/api/python > .gitignore
```

Alternatively, you can go to [gitignore.io](https://gitignore.io/) and generate a template more tailored to your needs.

## 2 - Add dependencies

We're going to need a few dependencies to get started.
`FastAPI` is the framework we're going to use to build the application, and `uvicorn` is a web server that we'll use to run the application.

```bash
poetry add fastapi
poetry add uvicorn
```

## 3 - Add API

Create an `api` directory to contain the code for the `FastAPI` application.

```bash
mkdir api
```

Then create a [`main.py`](./api/main.py) file to contain the code for the application.

```bash
touch api/main.py
```

In [`main.py`](./api/main.py), we'll add some code to create a `FastAPI` application.
First, we'll `import FastAPI`.
Then we'll instantiate an `app` object.
Finally, we'll add a `GET` route to the path "/" that returns a string saying "Hello, world!".

```python
from fastapi import FastAPI


app = FastAPI()


@app.get("/")
async def splashpage():
    return f"Hello, World!"
```

---

**NOTE**

The `async` keyword is used to mark functions that can be run asynchronously.
Asynchronous functions can be run in parallel and can be used to improve the performance of web applications.

---

## 4 - Try it out

We can run the application by running the following command:

```bash
poetry run uvicorn api.main:app --reload --port 8000
```

This will start a web server on port 8000.
We can visit http://localhost:8000/ to see the application running.

We should see the string `"Hello, World!"`.
