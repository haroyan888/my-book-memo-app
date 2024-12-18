import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
<<<<<<< Updated upstream
import {ChangeEvent, FormEvent, useState} from "react";
=======
import { Toast } from 'react-bootstrap';
import {ChangeEvent, useState} from "react";
import { useSearchParams, Link } from '@remix-run/react';
>>>>>>> Stashed changes

import {ORIGIN} from "~/consts";
import {useNavigate} from "@remix-run/react";
import myFetch from "~/utility/fetch/my-fetch";

function CreateAccount() {
    const [password, setPassword] = useState("");
    const [canCreateAccount, setCanCreateAccount] = useState(false);

    const onChangePasswordForm = (event: ChangeEvent<HTMLInputElement>) => setPassword(event.target.value)

    const onChangeReTypePassword = (event: ChangeEvent<HTMLInputElement>) => {
        const retypePassword = event.target.value;
        if (password === retypePassword) setCanCreateAccount(true);
        else setCanCreateAccount(false);
    }
    
    const [showWarnToast, setShowWarnToast] = useState(true);
    const closeWarnToast = () => setShowWarnToast(false);

    const [searchPrams] = useSearchParams();
    const failedMessage = searchPrams.get("failed");

    const navigate = useNavigate();
    const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const inputForm = new FormData(event.currentTarget);
        const urlEncodedData = new URLSearchParams();
        [...inputForm.entries()].forEach((value) =>
          urlEncodedData.append(value[0], value[1].toString()))
        const res = await myFetch(
          "http://localhost:8000/account",
          {
              method: "POST",
              body: urlEncodedData,
              credentials: "include",
          }
        );
        if (res.status === 500) {
            alert("アカウント作成中にエラーが発生しました。\n時間を置いてもう一度お試し下さい。");
            return;
        } else if (res.status === 400) {
            alert("入力情報のいずれかが間違っています。\nもう一度お試しください。");
            return;
        }

        navigate("/library");
    }

    return (
<<<<<<< Updated upstream
        <div className={"flex h-[100vh] items-center justify-center"}>
            <Form className={"border-solid border-4 rounded-2xl p-5 w-[80%]"} onSubmit={onSubmit}>
                <Form.Group className="mb-3" controlId="formEmail">
                    <Form.Label>メールアドレス</Form.Label>
                    <Form.Control type="email" name="email" placeholder="メールアドレスを入れてください" required />
                </Form.Group>
                <Form.Group className="mb-3" controlId="formPassword" onChange={onChangePasswordForm}>
                    <Form.Label>パスワード</Form.Label>
                    <Form.Control type="password" name="password" placeholder="パスワードを入れてください" required />
                </Form.Group>
                <Form.Group className="mb-3" controlId="formReTypingPassword" onChange={onChangeReTypePassword}>
                    <Form.Label>パスワード（再入力）</Form.Label>
                    <Form.Control type="password" name="re-typing-password" placeholder="パスワードを再入力してください" required />
                </Form.Group>
                <div className="flex flex-row justify-end">
                    <Button variant="primary" type="submit" className={"inline"} disabled={!canCreateAccount}>
                        アカウント作成
                    </Button>
                </div>
            </Form>
        </div>
=======
        <>
            <div className={"flex h-[100vh] items-center justify-center"}>
                <Form className={"border-solid border-4 rounded-2xl p-5 w-[80%]"} action="http://localhost:8000/account" method="POST">
                    <Form.Group className="mb-3" controlId="formEmail">
                        <Form.Label>メールアドレス</Form.Label>
                        <Form.Control type="email" name="email" placeholder="メールアドレスを入れてください" required />
                    </Form.Group>
                    <Form.Group className="mb-3" controlId="formPassword" onChange={onChangePasswordForm}>
                        <Form.Label>パスワード</Form.Label>
                        <Form.Control type="password" name="password" placeholder="パスワードを入れてください" required />
                    </Form.Group>
                    <Form.Group className="mb-3" controlId="formReTypingPassword" onChange={onChangeReTypePassword}>
                        <Form.Label>パスワード（再入力）</Form.Label>
                        <Form.Control type="password" name="re-typing-password" placeholder="パスワードを再入力してください" required />
                    </Form.Group>
                    <Form.Group className="mb-3" controlId="formNext">
                        <Form.Control type="hidden" name="next" value={ORIGIN + "/library"}/>
                    </Form.Group>
                    <Form.Group className="mb-3" controlId="formFailed">
                        <Form.Control type="hidden" name="failed" value={ORIGIN + "/create-account"}/>
                    </Form.Group>
                    <div className="flex flex-row justify-between">
                        <Link to={"/login"}>ログインはこちら</Link>
                        <Button variant="primary" type="submit" className={"inline"} disabled={!canCreateAccount}>
                            アカウント作成
                        </Button>
                    </div>
                </Form>
            </div>
            {failedMessage
                ?<Toast
                    className="absolute left-2 bottom-2"
                    bg="danger"
                    onClose={closeWarnToast}
                    show={showWarnToast}
                    delay={3000}
                    autohide
                >
                    <Toast.Body>{failedMessage}</Toast.Body>
                </Toast>
                : undefined
            }
        </>
>>>>>>> Stashed changes
    );
}

export default CreateAccount;
