import {Button, Modal, Image, Form} from "react-bootstrap";
import ShowMore from "~/components/ShowMore/ShowMore";
import Book from "~/types/book";
import {useEffect, useState, FormEventHandler} from "react";

import Memo from "~/types/memo";
import MemoList from "~/components/MemoList/MemoList";

interface props {
	book: Book,
	show: boolean,
	handleClose: () => void,
}

export default function BookDetailModal({book, show, handleClose}: props) {
	const memoUrl = "http://localhost:8000/book/" + book.isbn_13 + "/memo";

	const [memoList, setMemoList] = useState<Memo[] | undefined>(undefined);

	const getMemoList = async () => {
		const res = await fetch(memoUrl);
		if (!res.ok) {
			return;
		}
		const resMemoList: Memo[] = await res.json();
		setMemoList(resMemoList);
	};

	const handleMemoInputFormSubmit: FormEventHandler<HTMLFormElement> = (event) => {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		const memo = form.get("memo-input-form")?.toString();
		(async () => {
			const res = await fetch("http://localhost:8000/book/" + book.isbn_13 + "/memo", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					"text": memo,
				})
			});
			if(!res.ok) {
				alert("メモの追加に失敗しました");
				return;
			}
			await getMemoList();
		})();
	}

	useEffect(() => {
		getMemoList();
	}, [])

	return (
		<Modal size="xl" show={show} onHide={handleClose} centered>
			<Modal.Header closeButton>
				<Modal.Title>詳細</Modal.Title>
			</Modal.Header>

			<Modal.Body className="max-h-[400px] overflow-y-scroll">
				<div className="flex items-top gap-4 flex-wrap justify-center">
					<div className="min-w-[128px]">
						<h1 className="text-base">画像</h1>
						<Image className="w-[128px] h-[182px]" src={book.image_url}></Image>
					</div>
					<div className="max-w-[500px]">
						<h1 className="text-base">タイトル</h1>
						<p className="text-2xl">{book.title}</p>
						<h1 className="text-base">説明</h1>
						<ShowMore text={book.description}/>
						<h1 className="text-base">メモ</h1>
						{memoList?.map((memo) =>
							<MemoList
								text={memo.text}
								deleteApi={"http://localhost:8000/memo/" + memo.id}
								handleAfterDelete={getMemoList}
								key={memo.id} />)}
						<Form onSubmit={handleMemoInputFormSubmit}>
							<Form.Control name="memo-input-form" type="input"/>
						</Form>
					</div>
				</div>
			</Modal.Body>

			<Modal.Footer>
				<Button variant="secondary" onClick={handleClose}>閉じる</Button>
			</Modal.Footer>
		</Modal>
	)
}