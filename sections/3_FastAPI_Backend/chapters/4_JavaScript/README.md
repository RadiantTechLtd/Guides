# JavaScript

Next we'll add some inputs to our page, ready to sample the API.

## Add script.js

In the `static` folder, create a new file called `script.js` and add the following code:

```js
console.log("script.js loaded.");
```

## Link script.js

Update the [`base.html`](./templates/base.html) file to link to the `script.js` file:

```html
<head>
    ...
    <script src="/static/script.js"></script>
</head>
```

If you run the app now, you should see the message `"script.js loaded."` in the browser console.
