import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import {ChangeEvent, useState} from "react";

import {ORIGIN} from "~/consts";

function CreateAccount() {
    const [password, setPassword] = useState("");
    const [canCreateAccount, setCanCreateAccount] = useState(false);

    const onChangePasswordForm = (event: ChangeEvent<HTMLInputElement>) => setPassword(event.target.value)

    const onChangeReTypePassword = (event: ChangeEvent<HTMLInputElement>) => {
        const retypePassword = event.target.value;
        if (password === retypePassword) setCanCreateAccount(true);
        else setCanCreateAccount(false);
    }

    return (
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
                <div className="flex flex-row justify-end">
                    <Button variant="primary" type="submit" className={"inline"} disabled={!canCreateAccount}>
                        アカウント作成
                    </Button>
                </div>
            </Form>
        </div>
    );
}

export default CreateAccount;
