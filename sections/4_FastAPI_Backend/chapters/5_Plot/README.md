# JavaScript

We want plots really.
We'll start by generating the image on the server, and then telling the client to set it as the background image.

## Edit main.py

At the top of the [`main.py`](./app/main.py) file and add the following import:

```python
from pydantic import BaseModel
```

And then add the following route code to the end of the file:

```python
class ImageInput(BaseModel):
    real: float
    imag: float
    width: int
    height: int
    zoom: float
    max_iter: int


@app.post("/image")
async def image(input: ImageInput):
    data = mandy.sample.area(
        input.real, input.imag, input.width, input.height, input.zoom, input.max_iter
    )
    cols = mandy.colour.image(data, input.max_iter, mandy.colour.jet_map)
    img = mandy.colour.encode(cols)
    img.save("static/mandy.png")

    return "Done."
```

We need quite a few inputs to our plotting function `area()`.
So instead of passing them all in the URL (path) we'll pass them in the body of the request in `JSON` format.
As `FastAPI` works on types, we need to create a new `ImageInput` class (which must be derived from `BaseModel`) and pass that as the input to the function.

## Edit index.html

In the [`index.html`](./templates/index.html) file, modify the inputs to look like this:

```html
<input type="number" id="real_input" name="number" value="-0.101096374" />
<input type="number" id="imag_input" name="number" value="0.956286515" />
<br />
<input type="number" id="width_input" name="number" value="1920" />
<input type="number" id="height_input" name="number" value="1080" />
<br />
<input type="number" id="zoom_input" value="0.001" />
<input type="number" id="max_iter_input" value="100" />
<br />
<button onClick="plot()">Plot</button>
```

## Edit script.js

In the [`script.js`](./static/script.js) file, add a `plot()` function that reads like this:

```js
function plot() {
    const real = document.getElementById("real_input").value;
    const imag = document.getElementById("imag_input").value;
    const width = document.getElementById("width_input").value;
    const height = document.getElementById("height_input").value;
    const zoom = document.getElementById("zoom_input").value;
    const max_iter = document.getElementById("max_iter_input").value;

    const url = "http://localhost:8000/plot";
    const params = {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            real: real,
            imag: imag,
            width: width,
            height: height,
            zoom: zoom,
            max_iter: max_iter,
        }),
    };

    fetch(url, params)
        .then((_response) => {
            document.body.style.backgroundImage = "url(/static/mandy.png?r=" + Math.random() + ")";
        })
        .catch((error) => {
            console.log(error);
        });
}
```

We read the values from the inputs, and then send a `POST` request to the `/image` route.
The `POST` request has a `JSON` body, which is created from the values we read from the inputs.

We use a `fetch()` request to send the request.
The `fetch()` request returns a `Promise`, so we use the `then()` and `catch()` methods to handle the response.

If we get a response, we set the background image of the `body` element to the image we just generated.
Otherwise, we log the error.

## Try it out

Run the app:

```bash
poetry run uvicorn app.main:app --reload --port 8000
```

Navigate to http://localhost:8000/ and try entering some numbers into the inputs, and then clicking the `[Plot]` button.
Wait a few seconds, and you should see the plot appear in the background.
