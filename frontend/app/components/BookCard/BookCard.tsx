import {Card} from "react-bootstrap";

import Book from "~/types/book";
import BookDetailModal from "~/components/BookDetailModal/BookDetailModal";
import {useState} from "react";

interface props {
	book: Book,
	handleAfterDelete: () => void,
}

export default function BookCard({...props}: props) {
	const title_len_max = 30;
	const description_len_max = 40;

	const [show, setShow] = useState<boolean>(false);
	const handleOpen = () => setShow(true);
	const handleClose = () => setShow(false);

	const onClick = () => handleOpen();
	return(
		<>
			<button onClick={onClick} style={{margin: "5px"}}>
				<Card style={{width: '17.5rem', height: "530px"}}>
					<Card.Body>
						<Card.Img variant="top" src={props.book.image_url}/>
						<Card.Title>
							{props.book.title.length <= title_len_max
								? props.book.title
								: props.book.title.slice(0, title_len_max) + '...'}
						</Card.Title>
						<Card.Subtitle className="mb-2 text-muted">
							{props.book.authors.length == 1
								? props.book.authors[0]
								: props.book.authors[0] + ' ...'}
						</Card.Subtitle>
						<Card.Text>
							{props.book.description.length <= description_len_max
								? props.book.description
								: props.book.description.slice(0, description_len_max) + ' ...'}
						</Card.Text>
					</Card.Body>
				</Card>
			</button>
			<BookDetailModal book={props.book} show={show} handleClose={handleClose} handleAfterDelete={props.handleAfterDelete}/>
		</>
	)
}