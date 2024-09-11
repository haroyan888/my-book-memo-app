import {Modal, Button, Form, Spinner} from "react-bootstrap";
import {useState, ChangeEventHandler, FormEventHandler} from "react";

interface props {
	show: boolean,
	handleClose: () => void,
	createApi: string,
	afterCreateHandler: () => void,
}

export default function CreateBookModal({show, handleClose, createApi, afterCreateHandler}: props) {
	const [isbn13, setIsbn13] = useState<string>("");
	const [enableInput, setEnableInput] = useState<boolean>(true);
	const [enableSubmit, setEnableSubmit] = useState<boolean>(false);
	const [isLoading, setIsLoading] = useState<boolean>(false);
	const onChangeIsbn13Form: ChangeEventHandler<HTMLInputElement> = ({target}) => {
		const inputIsbn = target.value;
		setIsbn13(inputIsbn);
		setEnableSubmit(inputIsbn.length === 13);
	}
	const onSubmit = () => {
		setEnableInput(false);
		setEnableSubmit(false);
		setIsLoading(true);
		(async () => {
			const res = await fetch(
				createApi,
				{
					method: "POST",
					headers: {
						"Content-Type": "application/json",
					},
					body: JSON.stringify({
						"isbn_13": isbn13,
					})
				});
			setEnableInput(true);
			setEnableSubmit(true);
			setIsLoading(false);
			if(!res.ok) {
				if(res.statusText == "Bad Request") alert("登録済みです");
				if(res.statusText == "Not Found") alert("見つかりません");
				return;
			}
			afterCreateHandler();
			setIsbn13("");
		})();
	}
	return (
		<Modal show={show} onHide={handleClose} centered>
			<Modal.Header closeButton>
				<Modal.Title>本の登録</Modal.Title>
			</Modal.Header>

			<Modal.Body>
				<Form>
					<Form.Group
						onChange={onChangeIsbn13Form}
						className="mb-3"
						controlId="exampleForm.ControlInput1"
					>
						<Form.Label>ISBN 13</Form.Label>
						<Form.Control placeholder="1234567890123" disabled={!enableInput} />
					</Form.Group>
				</Form>
			</Modal.Body>

			<Modal.Footer>
				<Button variant="secondary" onClick={handleClose} className="h-10 w-20">閉じる</Button>
				<Button
					variant="primary"
					disabled={!enableSubmit}
					onClick={onSubmit}
					className="h-10 w-20"
				>
					{!isLoading
						? "登録"
						: <Spinner animation="border"  size="sm"/>
					}
				</Button>
			</Modal.Footer>
		</Modal>
	)
}