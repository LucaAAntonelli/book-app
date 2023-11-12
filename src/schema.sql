CREATE TABLE Books (
    book_id SERIAL PRIMARY KEY,
    title VARCHAR(63) NOT NULL,
    num_pages INTEGER NOT NULL,
    acquisition_date DATE NOT NULL,
    start_date DATE,
    end_date DATE,
    price_ebook DECIMAL(5,2),
    price_paperback DECIMAL(5,2)
);

CREATE TABLE Authors (
    author_id SERIAL PRIMARY KEY,
    name VARCHAR(63)
);

CREATE TABLE BookAuthors (
    book_id INTEGER REFERENCES Books(book_id),
    author_id INTEGER REFERENCES Authors(author_id),
    PRIMARY KEY (book_id, author_id)
);