import {Form} from "react-bootstrap";
import { TiDelete } from "react-icons/ti";
import myFetch from "~/utility/fetch/my-fetch";

interface props {
	text: string,
	deleteApi: string,
	handleAfterDelete: () => void,
}

export default function MemoList({text, deleteApi, handleAfterDelete}: props) {
	const handleClick = () => {
		(async () => {
			const res = await myFetch(deleteApi, {method: "DELETE"});
			if (!res.ok) {
				alert("削除に失敗しました");
				return;
			}
			handleAfterDelete();
		})();
	}
	return(
		<div className="flex gap-4 mt-4 mb-4">
			<Form.Control
				type="input"
				value={text}
				disabled
			/>
			<button className="text-red-500" onClick={handleClick}><TiDelete className="w-8 h-8" /></button>
		</div>
	)
}