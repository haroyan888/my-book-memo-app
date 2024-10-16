import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import {FormEvent} from "react";

import {ORIGIN} from "~/consts";
import {Link} from "@remix-run/react";

function CreateAccount() {
    const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const accountInfo = new FormData(event.currentTarget);
        const res = await fetch(
            "http://localhost:8000/create-account",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                redirect: "follow",
                body: JSON.stringify({
                    "email": accountInfo.get("email"),
                    "password": accountInfo.get("password"),
                    "next": window.location.origin + "/library",
                })
            }
        );
        if (res.status === 500) {
            alert("ログイン中にエラーが発生しました。\n時間を置いてもう一度お試し下さい。");
            return;
        } else if (res.status === 400) {
            alert("入力情報のいずれかが間違っています。\nもう一度お試しください。");
            return;
        }
    }

    return (
        <div className={"flex h-[100vh] items-center justify-center"}>
            <Form className={"border-solid border-4 rounded-2xl p-5 w-[80%]"} action="http://localhost:8000/login" method="POST">
                <Form.Group className="mb-3" controlId="formEmail">
                    <Form.Label>メールアドレス</Form.Label>
                    <Form.Control type="email" name="email" placeholder="メールアドレスを入れてください" required />
                </Form.Group>

                <Form.Group className="mb-3" controlId="formPassword">
                    <Form.Label>パスワード</Form.Label>
                    <Form.Control type="password" name="password" placeholder="パスワードを入れてください" required />
                </Form.Group>
                <Form.Group className="mb-3" controlId="formNext">
                    <Form.Control type="hidden" name="next" value={ORIGIN + "/library"}/>
                </Form.Group>
                <div className="flex flex-row justify-between">
                    <Link className="no-underline" to={"/create-account"}>アカウント作成はこちら</Link>
                    <Button variant="primary" type="submit" className={"inline"}>
                        ログイン
                    </Button>
                </div>
            </Form>
        </div>
    );
}

export default CreateAccount;
