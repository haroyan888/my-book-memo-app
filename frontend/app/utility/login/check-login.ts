import myFetch from "~/utility/fetch/my-fetch";

async function checkLoginStatus() {
    const res = await myFetch("http://localhost:8000/check-login-status",);
    if(!res.ok) return false;
    const data:　{"is_login": boolean} = await res.json();
    return data["is_login"];
}

export default checkLoginStatus;
