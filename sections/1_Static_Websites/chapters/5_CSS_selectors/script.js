function change_style() {
    let element = document.getElementById("another_id_selector");
    const current_value = element.getAttribute("my-custom-attribute");

    if (current_value === "something") {
        element.setAttribute("my-custom-attribute", "something_else");
        return;
    }
    element.setAttribute("my-custom-attribute", "something");
}
