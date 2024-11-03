import type { MetaFunction } from "@remix-run/node";
import {useEffect, useState} from "react";
import {Button} from "react-bootstrap";

import Book from "~/types/book";
import BookCard from "~/components/BookCard/BookCard";
import CreateBookModal from "~/components/CreateBookModal/CreateBookModal";
import myFetch from "~/utility/fetch/my-fetch";
import UserDropDown from "~/components/UserDropDown/UserDropDown";

export default function Index() {

    const baseURL = "http://localhost:8000"
    const bookUrl = baseURL + '/book';

    const [books, setBooks] = useState<Book[]>([]);
    const [show, setShow] = useState(false);
    const handleClose = () => setShow(false);
    const handleShow = () => setShow(true);

    const getBooksInfo = async () => {
        let res = await myFetch(bookUrl);
        if (!res.ok) {
            setBooks([]);
            return;
        }

        let books: Book[] = await res.json();
        setBooks(books);
    };

    const afterCreateHandler = () => {
        handleClose();
        void getBooksInfo();
    };

    useEffect(() => {
        void getBooksInfo();
    }, []);

    return (
        <>
            <header className="w-full p-3 flex justify-end bg-blue-500">
                <UserDropDown baseURL={baseURL} />
            </header>
            <div className="font-sans p-4 flex flex-wrap justify-content-center">
                {books.map((book) =>
                        <BookCard book={book} baseUrl={baseURL} key={book.isbn_13} handleAfterDelete={getBooksInfo}/>)}
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
