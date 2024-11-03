import {Button, Modal, Image, Form} from "react-bootstrap";
import ShowMore from "~/components/ShowMore/ShowMore";
import Book from "~/types/book";
import {useEffect, useState, FormEventHandler} from "react";

import Memo from "~/types/memo";
import MemoList from "~/components/MemoList/MemoList";
import ConfirmDialog from "~/components/ConfirmDialog/ConfirmDialog";
import myFetch from "~/utility/fetch/my-fetch";

interface props {
	book: Book,
	baseUrl: string,
	show: boolean,
	handleClose: () => void,
	handleAfterDelete: () => void,
}

export default function BookDetailModal({...props}: props) {
	const memoUrl = props.baseUrl + "/book/" + props.book.isbn_13 + "/memo";
	const deleteBookUrl = props.baseUrl + "/book/" + props.book.isbn_13;

	const [memoList, setMemoList] = useState<Memo[] | undefined>(undefined);
	const [showConfirmDialog, setShowConfirmDialog] = useState<boolean>(false);

	const handleConfirmDialogOpen = () => setShowConfirmDialog(true);
	const handleConfirmDialogClose = () => setShowConfirmDialog(false);

	const getMemoList = async () => {
		const res = await myFetch(memoUrl);
		if (!res.ok) {
			return;
		}
		const resMemoList: Memo[] = await res.json();
		setMemoList(resMemoList);
	};

	const deleteBook = async () => {
		const res = await myFetch(deleteBookUrl, { method: "DELETE" });
		if (!res.ok) {
			return;
		}
		handleConfirmDialogClose();
		props.handleClose();
		props.handleAfterDelete()
	}

	const handleMemoInputFormSubmit: FormEventHandler<HTMLFormElement> = (event) => {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		const memo = form.get("memo-input-form")?.toString();
		(async () => {
			const res = await myFetch("http://localhost:8000/book/" + props.book.isbn_13 + "/memo", {
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
		<>
			<Modal size="xl" show={props.show} onHide={props.handleClose} centered>
				<Modal.Header closeButton>
					<Modal.Title>詳細</Modal.Title>
				</Modal.Header>

				<Modal.Body className="max-h-[400px] overflow-y-scroll">
					<div className="flex items-top gap-11 flex-wrap justify-center">
						<div className="min-w-[128px]">
							<Image className="w-full" src={props.book.image_url}></Image>
							<Button className="mt-3 w-full" variant="danger" onClick={handleConfirmDialogOpen}>削除</Button>
						</div>
						<div className="max-w-[500px]">
							<h1 className="text-base">タイトル</h1>
							<p className="text-2xl">{props.book.title}</p>
							<p className="text-lg text-right">{props.book.publisher} {props.book.published_date}</p>
							<h1 className="text-base">著者</h1>
							<p className="text-lg">{props.book.authors.join(', ')}</p>
							<h1 className="text-base">説明</h1>
							<ShowMore text={props.book.description}/>
							<h1 className="text-base">メモ</h1>
							{memoList?.map((memo) =>
								<MemoList
									text={memo.text}
									deleteApi={"http://localhost:8000/memo/" + memo.id}
									handleAfterDelete={getMemoList}
									key={memo.id}/>)}
							<Form onSubmit={handleMemoInputFormSubmit}>
								<Form.Control name="memo-input-form" type="input"/>
							</Form>
						</div>
					</div>
				</Modal.Body>
			</Modal>
			<ConfirmDialog message={"削除しますか？"} variant="danger" show={showConfirmDialog} handleClose={handleConfirmDialogClose} handleConfirm={deleteBook} />
		</>
	)
}