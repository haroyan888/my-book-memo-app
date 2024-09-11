import {Card} from "react-bootstrap";

import Book from "~/types/book";
import BookDetailModal from "~/components/BookDetailModal/BookDetailModal";
import {useState} from "react";

interface props {
	book: Book,
}

export default function BookCard({book}: props) {
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
						<Card.Img variant="top" src={book.image_url}/>
						<Card.Title>
							{book.title.length <= title_len_max
								? book.title
								: book.title.slice(0, title_len_max) + '...'}
						</Card.Title>
						<Card.Subtitle className="mb-2 text-muted">
							{book.authors.length == 1
								? book.authors[0]
								: book.authors[0] + ' ...'}
						</Card.Subtitle>
						<Card.Text>
							{book.description.length <= description_len_max
								? book.description
								: book.description.slice(0, description_len_max) + ' ...'}
						</Card.Text>
					</Card.Body>
				</Card>
			</button>
			<BookDetailModal book={book} show={show} handleClose={handleClose} />
		</>
	)
}