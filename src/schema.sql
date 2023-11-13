DROP TABLE IF EXISTS Books, Authors, BookAuthors;

CREATE TABLE Books (
    book_id SERIAL PRIMARY KEY,
    title VARCHAR(63) NOT NULL,
    num_pages INTEGER NOT NULL,
    acquisition_date DATE NOT NULL,
    start_date DATE,
    end_date DATE,
    price_ebook DECIMAL(5,2),
    price_paperback DECIMAL(5,2),
    CONSTRAINT no_negative_prices CHECK (price_ebook >= 0 AND price_paperback >= 0),
    CONSTRAINT check_order CHECK (acquisition_date <= start_date AND start_date <= end_date)
);

CREATE TABLE Authors (
    author_id SERIAL PRIMARY KEY,
    name VARCHAR(63) UNIQUE NOT NULL
);

CREATE TABLE BookAuthors (
    book_id INTEGER REFERENCES Books(book_id) ON DELETE CASCADE,
    author_id INTEGER REFERENCES Authors(author_id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, author_id)
);