import {Link} from "@remix-run/react";
import {useEffect, useState} from "react";
import {Spinner} from "react-bootstrap";

import checkLogin from "~/utility/login/check-login";

function Index() {
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        console.log(import.meta.env.VITE_BACKEND_URL);
        (async () => {
            setIsLoading(true);
            setIsLoggedIn(await checkLogin());
            setIsLoading(false);
        })();
    }, []);

    return (
        <>
            <Spinner animation="border" hidden={!isLoading}/>
            <div hidden={isLoading}>
                {isLoggedIn
                    ?<Link to="/library">マイライブラリへ</Link>
                    :<Link to="/login">ログインページへ</Link>}
            </div>
        </>
    )
}

export default Index;