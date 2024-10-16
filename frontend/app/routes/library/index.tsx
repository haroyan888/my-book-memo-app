import type { MetaFunction } from "@remix-run/node";
import {useEffect, useState} from "react";
import {Button} from "react-bootstrap";

import Book from "~/types/book";
import BookCard from "~/components/BookCard/BookCard";
import CreateBookModal from "~/components/CreateBookModal/CreateBookModal";
import myFetch from "~/utility/fetch/my-fetch";

export const meta: MetaFunction = () => {
    return [
        { title: "New Remix App" },
        { name: "description", content: "Welcome to Remix!" },
    ];
};

export default function Index() {

    const bookUrl = 'http://localhost:8000/book';

    const [books, setBooks] = useState<Book[] | undefined>(undefined);
    const [show, setShow] = useState(false);
    const handleClose = () => setShow(false);
    const handleShow = () => setShow(true);

    const getBooksInfo = async () => {
        let res = await myFetch(bookUrl);
        if (!res.ok) {
            setBooks(undefined);
            return;
        }

        let books: Book[] = await res.json();
        console.log(books);
        setBooks(books);
    };
    const afterCreateHandler = () => {
        handleClose();
        getBooksInfo();
    };

    useEffect(() => {
        getBooksInfo();
    }, []);

    return (
        <>
            <div className="font-sans p-4 flex flex-wrap justify-content-center">
                {books
                    ? books.map((book) =>
                        <BookCard book={book} key={book.isbn_13} handleAfterDelete={getBooksInfo}/>)
                    : undefined
                }
            </div>
            <Button className="fixed bottom-[12px] right-[12px]" variant="primary" onClick={handleShow}>本を追加</Button>
            <CreateBookModal
                show={show}
                handleClose={handleClose}
                createApi={bookUrl}
                afterCreateHandler={afterCreateHandler}
            />
        </>
    );
}
