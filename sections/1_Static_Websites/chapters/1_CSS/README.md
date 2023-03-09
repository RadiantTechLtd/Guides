# `CSS` - Styling

`CSS` stands for **C**ascading **S**tyle **S**heets.
Like `HTML` it is a declarative language.
`CSS` is a language for describing the presentation of web pages, including colors, layout, and fonts. It allows one to adapt the presentation to different types of devices, such as large screens, small screens, or printers.

## Example

A simple `CSS` stylesheet might look like this:

```css
body {
    background-color: #f0f0f0;
    font-family: sans-serif;
}

h1 {
    color: green;
    font-weight: bold;
}

p {
    color: #0000ff;
}
```

Here, we've targeted all the `<h1>` and `<p>` elements using CSS selectors and specified different styles for each.
`background-color` changes the background colour of the element.
The `color` property changes the colour of the text, `font-size` changes the size of the text, and `font-family` changes the font family used for the text.

We can save the code in the snippet above into a file called [`style.css`](./style.css) and then link it to our `HTML` document using the `<link>` tag in the `<head>` element:

```html
<!DOCTYPE html>
<html>
    <head>
        <title>HTML + CSS</title>
        <link rel="stylesheet" href="style.css" />
    </head>
    <body>
        <h1>Hello, again!</h1>
        <p>This is my second HTML document.</p>
        <button>Click me!</button>
    </body>
</html>
```

## Try it out

Now, if we open up the file [`index.html`](./index.html) in our browser, we should see the rendered web page with styling applied.

## Details `CSS` separates the presentation of a document from its content, allowing developers to control the visual aspects of a web page independently of its

structure and content. By using `CSS`, developers can specify font styles, colours, margins, padding, and other layout properties for elements on a web page.
Some key features of `CSS` include: - Cascading rules: `CSS` uses cascading rules to determine which styles should be applied to an element when there are
conflicting style rules. - Selectors: `CSS` uses selectors to select the elements that should be styled. - Inheritance: `CSS` properties can be inherited from
parent elements to child elements, reducing the amount of code needed to style a web page. - Box model: `CSS` uses a box model to describe the layout of
elements on a web page, including their padding, borders, and margins. - Responsive design: `CSS` provides tools for creating responsive designs, allowing web
pages to adjust their layout and style based on the size and orientation of the screen they are viewed on. Overall, `CSS` is a powerful tool for controlling the
layout and style of web pages. Its support for cascading rules, selectors, inheritance, box model, and responsive design make it an essential part of modern web
development.
