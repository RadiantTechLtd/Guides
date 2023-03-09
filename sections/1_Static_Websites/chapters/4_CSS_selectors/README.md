# CSS Selectors

`CSS` selectors are used to select the elements that should be styled.
There are many different types of selectors, each with its own use case.
In this chapter, we'll look at the most commonly used selectors.

## Table

| Selector  | Description                            |
| --------- | -------------------------------------- |
| `*`       | Selects all elements                   |
| `element` | Selects all instances of an element    |
| `.class`  | Selects all instances of a class       |
| `#id`     | Selects the element with a specific ID |

You can combine selectors to select elements that match multiple criteria.

| Combinator          | Description                                                                                                                          |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `element.class`     | Selects all instances of an element with a specific class                                                                            |
| `element, element`  | Selects all instances of both elements                                                                                               |
| `element > element` | Selects all instances of the second element that are a direct child of the first element                                             |
| `element + element` | Selects all instances of the second element that directly follow the first element                                                   |
| `element ~ element` | Selects all instances of the second element that follow the first element, regardless of whether there are other elements in between |

You can also use attribute selectors to select elements based on their attributes, which may mutate over time.

| Attribute           | Description                                                                |
| ------------------- | -------------------------------------------------------------------------- |
| `[attribute]`       | Selects all elements with the specified attribute                          |
| `[attribute=value]` | Selects all elements where the specified attribute has the specified value |

## Try it out

[`index.html`](./index.html) is another ugly web page.
But it (along with [`style.css`](./style.css)) contains some of the `CSS` selectors above used to colour the text.
[`script.js`](./script.js) contains some `JavaScript` code that will update some of the attributes of the elements on the page and therefore change the way they are styled.
