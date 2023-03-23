const baseURL = "http://127.0.0.1:3000";
// const baseURL = "https://0hku56rtzd.execute-api.eu-west-1.amazonaws.com/Prod";

const plot = () => {
    // Definitions
    const real = document.getElementById("real").value;
    const imag = document.getElementById("imag").value;
    const scale = document.getElementById("scale").value;

    const url = baseURL + "/plot";

    const params = {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            // Uncommenting the below makes this fail
            // TODO: Figure out CORS headers
            // "Access-Control-Allow-Headers": "Content-Type",
            // "Access-Control-Allow-Origin": "*",
            // "Access-Control-Allow-Credentials": "true",
            // "Access-Control-Expose-Headers": "x-api-id",
            // "Access-Control-Max-Age": "300",
            // "Access-Control-Allow-Methods": "*",
        },
        body: JSON.stringify({
            real: real,
            imag: imag,
            scale: scale,
        }),
    };

    // Fetch
    console.log("Request sent");
    fetch(url, params)
        .then((response) => response.json())
        .then((response) => {
            console.log("Response: ");
            console.log(response);
        })
        .catch((error) => {
            console.log("Error:", error);
        });
};
