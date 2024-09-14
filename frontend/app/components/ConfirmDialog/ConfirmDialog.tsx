import {Button, Modal} from "react-bootstrap";
import Color from "~/types/color";

interface props {
	message: string,
	variant: Color,
	show: boolean,
	handleClose: () => void,
	handleConfirm: () => void,
}

export default function ConfirmDialog({...props}: props) {
	return (
		<Modal show={props.show} onHide={props.handleClose}>
			<Modal.Header closeButton>
				<Modal.Title>確認</Modal.Title>
			</Modal.Header>
			<Modal.Body>
				<p>{props.message}</p>
			</Modal.Body>
			<Modal.Footer>
				<Button variant="secondary" onClick={props.handleClose}>キャンセル</Button>
				<Button variant={props.variant} onClick={props.handleConfirm}>決定</Button>
			</Modal.Footer>
		</Modal>
	)
}