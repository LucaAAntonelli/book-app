DROP TABLE IF EXISTS owned_books, read_books, authors, book_authors, series_info, did_not_finish_books;

CREATE TABLE owned_books (
    book_id SERIAL PRIMARY KEY,
    title VARCHAR(63) UNIQUE NOT NULL,
    num_pages INTEGER NOT NULL,
    acquisition_date DATE NOT NULL,
    url VARCHAR(255) UNIQUE NOT NULL,
    cover_url VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE read_books (
    book_id INTEGER REFERENCES owned_books(book_id) ON DELETE CASCADE,
    start_date DATE NOT NULL,
    end_date DATE,
    CONSTRAINT check_order CHECK (start_date <= end_date)
);

CREATE TABLE authors (
    author_id SERIAL PRIMARY KEY,
    name VARCHAR(63) NOT NULL
);

CREATE TABLE book_authors (
    book_id INTEGER REFERENCES owned_books(book_id) ON DELETE CASCADE,
    author_id INTEGER REFERENCES authors(author_id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, author_id)
);

CREATE TABLE series_info (
    book_id INTEGER REFERENCES owned_books(book_id) ON DELETE CASCADE,
    series_name VARCHAR(30) NOT NULL,
    volume NUMERIC(3,1) NOT NULL
);

CREATE TABLE did_not_finish_books (
    book_id INTEGER REFERENCES owned_books(book_id) ON DELETE CASCADE
);