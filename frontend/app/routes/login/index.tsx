import { Form, Button, Col, Row } from 'react-bootstrap';
import {FormEvent} from "react";
import {Link, useNavigate} from "@remix-run/react";

import {ORIGIN} from "~/consts";
import myFetch from "~/utility/fetch/my-fetch";

function Login() {
    const navigate = useNavigate();
    const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const inputForm = new FormData(event.currentTarget);
        const urlEncodedData = new URLSearchParams();
        [...inputForm.entries()].forEach((value) =>
            urlEncodedData.append(value[0], value[1].toString()))
        const res = await myFetch(
            "http://localhost:8000/login",
            {
                method: "POST",
                body: urlEncodedData,
                credentials: "include",
            }
        );
        if (res.status === 500) {
            alert("ログイン中にエラーが発生しました。\n時間を置いてもう一度お試し下さい。");
            return;
        } else if (res.status === 400) {
            alert("入力情報のいずれかが間違っています。\nもう一度お試しください。");
            return;
        }

        navigate("/library");
    }

    return (
        <div className={"flex h-[100vh] items-center justify-center"}>
            <Form className={"border-solid border-4 rounded-2xl p-5 w-[80%]"} onSubmit={onSubmit}>
                <Form.Group className="mb-3" controlId="formEmail">
                    <Form.Label>メールアドレス</Form.Label>
                    <Form.Control type="email" name="email" placeholder="メールアドレスを入れてください" required />
                </Form.Group>
                <Form.Group className="mb-3" controlId="formPassword">
                    <Form.Label>パスワード</Form.Label>
                    <Form.Control type="password" name="password" placeholder="パスワードを入れてください" required />
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

export default Login;
