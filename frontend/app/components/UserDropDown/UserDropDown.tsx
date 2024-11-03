import {useEffect, useState} from "react";
import {Dropdown} from "react-bootstrap";
import {useNavigate} from "@remix-run/react";

import myFetch from "~/utility/fetch/my-fetch";
import User from "~/types/user";
import ConfirmDialog from "~/components/ConfirmDialog/ConfirmDialog";

type props = {
    baseURL: string,
};

function UserDropDown({baseURL}: props) {
    const getUserInfoURL = baseURL + "/account";

    const [user, setUser] = useState<User | null>(null);
    const [loading, setLoading] = useState(false);
    const [showLogoutConfirmDialog, setShowLogoutConfirmDialog] = useState(false);
    const navigate = useNavigate();

    const getUserInfo = async () => {
        setLoading(true);
        const res = await myFetch(getUserInfoURL, {"method": "GET"});
        setLoading(false);
        if (!res.ok) {
            setUser(null);
            return;
        }
        const user: User = await res.json();
        setUser(user);
    }

    useEffect(() => {
        void getUserInfo();
    }, []);

    const onClickLogoutButton = () => setShowLogoutConfirmDialog(true);
    const closeLogoutConfirmDialog = () => setShowLogoutConfirmDialog(false);
    const confirmLogoutHandler = async () => {
        const res = await myFetch(baseURL + "/logout");
        if (!res.ok) {
            alert("ログアウトに失敗しました");
            closeLogoutConfirmDialog();
            return;
        }
        closeLogoutConfirmDialog();
        navigate("/");
    };

    return (
        <>
            {!loading
                ? <Dropdown>
                    <Dropdown.Toggle variant="success" id="dropdown-basic">
                        {user ? user.email : "ログインして下さい"}
                    </Dropdown.Toggle>

                    {user
                        ? <Dropdown.Menu>
                            <Dropdown.Item onClick={onClickLogoutButton}>ログアウト</Dropdown.Item>
                            <Dropdown.Item>アカウントの削除</Dropdown.Item>
                        </Dropdown.Menu>
                        : <Dropdown.Menu>
                            <Dropdown.Item href={"/login"}>ログイン</Dropdown.Item>
                        </Dropdown.Menu>
                    }
                </Dropdown>
                : undefined}
            <ConfirmDialog
                message="ログアウト"
                variant="danger"
                show={showLogoutConfirmDialog}
                handleClose={closeLogoutConfirmDialog}
                handleConfirm={confirmLogoutHandler}
            />
        </>
    )
}

export default UserDropDown;