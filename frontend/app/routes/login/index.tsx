import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import {FormEvent} from "react";

import {ORIGIN} from "~/consts";

function Login() {
    const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const accountInfo = new FormData(event.currentTarget);
        const res = await fetch(
            "http://localhost:8000/login",
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
        <div className={"flex flex-col items-center"}>
            <Form className={"w-[80%]"} action="http://localhost:8000/login" method="POST">
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
                <Button variant="primary" type="submit" className={""}>
                    ログイン
                </Button>
            </Form>
        </div>
    );
}

export default Login;