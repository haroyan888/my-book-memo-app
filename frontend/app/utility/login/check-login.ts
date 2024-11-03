import myFetch from "~/utility/fetch/my-fetch";

async function isLoggedIn() {
    const res = await myFetch("http://localhost:8000/account",);
    return res.ok;
}

export default isLoggedIn;
