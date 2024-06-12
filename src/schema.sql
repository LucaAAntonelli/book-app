DROP TABLE IF EXISTS owned_books, read_books, authors, book_authors;

CREATE TABLE owned_books (
    book_id SERIAL PRIMARY KEY,
    title VARCHAR(63) UNIQUE NOT NULL,
    num_pages INTEGER NOT NULL,
    acquisition_date DATE NOT NULL,
    price_ebook DECIMAL(5,2),
    price_paperback DECIMAL(5,2),
    CONSTRAINT no_negative_prices CHECK (price_ebook >= 0 AND price_paperback >= 0)
);

CREATE TABLE read_books (
    book_id INTEGER REFERENCES owned_books(book_id) ON DELETE CASCADE,
    start_date DATE,
    end_date DATE,
    CONSTRAINT check_order CHECK (start_date <= end_date)
);

CREATE TABLE authors (
    author_id SERIAL PRIMARY KEY,
    name VARCHAR(63) UNIQUE NOT NULL
);

CREATE TABLE book_authors (
    book_id INTEGER REFERENCES owned_books(book_id) ON DELETE CASCADE,
    author_id INTEGER REFERENCES authors(author_id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, author_id)
);