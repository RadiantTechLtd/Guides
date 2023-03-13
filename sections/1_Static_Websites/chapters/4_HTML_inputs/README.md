# HTML Inputs

There are many `HTML` tags that you can use to create a web page.
The following table lists some of the most commonly used tags.

## Table

| Type     | Description                                                                                             | Example                                                     | Value Output Type |
| -------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- | ----------------- |
| text     | Single-line input field for text.                                                                       | `<input type="text" name="username">`                       | String            |
| password | Single-line input field for passwords, where the characters are replaced by asterisks or other symbols. | `<input type="password" name="password">`                   | String            |
| email    | Single-line input field for email addresses.                                                            | `<input type="email" name="email">`                         | String            |
| number   | Single-line input field for numeric values.                                                             | `<input type="number" name="quantity" min="1" max="10">`    | Number            |
| date     | Single-line input field for dates.                                                                      | `<input type="date" name="date">`                           | String            |
| time     | Single-line input field for time.                                                                       | `<input type="time" name="time">`                           | String            |
| checkbox | Checkbox input field that allows the user to select one or more options.                                | `<input type="checkbox" name="option1" value="yes">`        | Boolean           |
| radio    | Radio button input field that allows the user to select one option from a group of options.             | `<input type="radio" name="gender" value="male">`           | String            |
| file     | Input field that allows the user to select one or more files to upload.                                 | `<input type="file" name="fileToUpload" id="fileToUpload">` | String            |
| submit   | Button that submits the form to the server.                                                             | `<input type="submit" value="Submit">`                      | n/a               |
| reset    | Button that resets the form to its initial values.                                                      | `<input type="reset" value="Reset">`                        | n/a               |

## Try it out

[`index.html`](./index.html) shows an example of each of the `<input>` types.
