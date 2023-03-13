# CSS Selectors

`CSS` selectors are used to select the `HTML` elements you want to style.
They are also used to select the `HTML` elements you want to interact with using `JavaScript`.

## Table

| Selector         | Description                                                                             | Example                                                    |
| ---------------- | --------------------------------------------------------------------------------------- | ---------------------------------------------------------- |
| element          | Selects all elements of a certain type.                                                 | `p { color: blue; }`                                       |
| class            | Selects elements with a specific class name.                                            | `.my-class { font-size: 20px; }`                           |
| ID               | Selects an element with a specific ID attribute.                                        | `#my-id { background-color: yellow; }`                     |
| descendant       | Selects all elements that are descendants of a specified element.                       | `div p { text-align: center; }`                            |
| child            | Selects all direct child elements of a specified element.                               | `ul > li { font-weight: bold; }`                           |
| adjacent sibling | Selects an element that is directly after another specified element.                    | `h1 + p { font-size: 20px; }`                              |
| general sibling  | Selects elements that are siblings of a specified element.                              | `h2 ~ p { font-style: italic; }`                           |
| attribute        | Selects elements based on their attribute values.                                       | `a[href="https://www.example.com"] { color: green; }`      |
| pseudo-class     | Selects elements based on a state or condition that exists on the element.              | `a:hover { text-decoration: underline; }`                  |
| pseudo-element   | Selects a specific part of an element, such as the first letter or line of text.        | `p::first-letter { font-size: 2em; }`                      |
| :not()           | Selects all elements that do not match a specified selector.                            | `p:not(.my-class) { color: red; }`                         |
| :nth-child()     | Selects every nth child of an element, based on its position within its parent element. | `ul li:nth-child(odd) { background-color: lightgray; }`    |
| :nth-of-type()   | Selects every nth element of a specific type within a parent element.                   | `ul li:nth-of-type(even) { background-color: lightgray; }` |
| :first-child     | Selects the first child element of a parent element.                                    | `ul li:first-child { font-weight: bold; }`                 |
| :last-child      | Selects the last child element of a parent element.                                     | `ul li:last-child { color: red; }`                         |
| :first-of-type   | Selects the first element of a specific type within a parent element.                   | `ul li:first-of-type { font-weight: bold; }`               |
| :last-of-type    | Selects the last element of a specific type within a parent element.                    | `ul li:last-of-type { color: red; }`                       |

## Try it out

[`index.html`](./index.html) shows an example of each of the basic selectors in action.
