# JavaScript - Interactivity

`JavaScript` is a programming language that runs in the browser.
It is used to make websites interactive.

## Example

A simple `JavaScript` program might look like this:

```js
function say_hello() {
    alert("Hello, World!");
}
```

This defines a simple function called `say_hello` that displays an alert box with the message `"Hello, World!"`.

We can save the above code snippet into a file called `script.js` and include it in our `HTML` document using a `<script>` element:

```html
<!DOCTYPE html>
<html>
    <head>
        <title>HTML + CSS + JavaScript</title>
        <link rel="stylesheet" href="style.css" />
        <script src="script.js"></script>
    </head>
    <body>
        <h1>Hello, again!</h1>
        <p>This is my second HTML document.</p>
        <button onClick="say_hello()">Click me!</button>
    </body>
</html>
```

We can then trigger this function via the added an `onClick` attribute in the `<button>` element in our `HTML` document.

## Try it out

If we open [`index.html`](./index.html) in our browser, we should see the rendered web page with styling applied and an alert box that appears when we click the button.

## Details

`JavaScript` is a high-level, interpreted programming language that is used to create interactive and dynamic websites.
It was first introduced in 1995 and has since become one of the most widely used programming languages in the world.

`JavaScript` is primarily used for client-side web development, meaning that it runs on the user's browser and is responsible for handling user interactions and updating the page in real-time. It is also used for server-side web development through the use of platforms like `Node.js`.

Some key features of `JavaScript` include:

-   Dynamic typing: `JavaScript` is dynamically typed, meaning that data types are determined at runtime rather than during compilation.
-   Prototypal inheritance: `JavaScript` uses prototypal inheritance to define object classes and their properties and methods.
-   Functional programming: `JavaScript` supports functional programming techniques, such as closures and higher-order functions.
-   Event-driven programming: `JavaScript` is event-driven, meaning that it responds to user interactions and other events in real-time.
-   Wide support: `JavaScript` is supported by all major browsers and can be used on a wide range of platforms, including desktop and mobile devices.

Overall, `JavaScript` is a versatile and powerful programming language that is widely used for web development.
Its support for dynamic typing, prototypal inheritance, functional programming, and event-driven programming make it a popular choice for developers who are building interactive and dynamic web applications.
