function read_values() {
    const USERNAME_INPUT = document.getElementById("username_input").value;
    const PASSWORD_INPUT = document.getElementById("password_input").value;
    const EMAIL_INPUT = document.getElementById("email_input").value;
    const NUMBER_INPUT = document.getElementById("number_input").value;
    const DATE_INPUT = document.getElementById("date_input").value;
    const TIME_INPUT = document.getElementById("time_input").value;
    const CHECKBOX_INPUT = document.getElementById("checkbox_input").checked;
    const RADIO_INPUT = document.getElementById("radio_input").value;
    const FILE_INPUT = document.getElementById("file_input").value;
    const SUBMIT_INPUT = document.getElementById("submit_input").value;
    const RESET_INPUT = document.getElementById("reset_input").value;

    console.log(`username_input: ${USERNAME_INPUT}`);
    console.log(`password_input: ${PASSWORD_INPUT}`);
    console.log(`email_input: ${EMAIL_INPUT}`);
    console.log(`number_input: ${NUMBER_INPUT}`);
    console.log(`date_input: ${DATE_INPUT}`);
    console.log(`time_input: ${TIME_INPUT}`);
    console.log(`checkbox_input: ${CHECKBOX_INPUT}`);
    console.log(`radio_input: ${RADIO_INPUT}`);
    console.log(`file_input: ${FILE_INPUT}`);
    console.log(`submit_input: ${SUBMIT_INPUT}`);
    console.log(`reset_input: ${RESET_INPUT}`);

    alert("Values printed to console.");
}
