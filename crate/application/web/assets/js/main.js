function getAuthToken() {
    return localStorage.getItem('authToken');
}

function saveAuthToken(token) {
    return localStorage.setItem('authToken', token);
}

htmx.on("htmx:configRequest", (e) => {
    e.detail.headers["AUTH"] = getAuthToken()
})