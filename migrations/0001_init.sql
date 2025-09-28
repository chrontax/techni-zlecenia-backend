CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

);

CREATE TABLE orders (
    order_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    order_name VARCHAR(100) NOT NULL,
    order_desc TEXT NOT NULL,
    price NUMERIC(10, 2) NOT NULL CHECK (price >= 0),
    image_urls VARCHAR(100) ARRAY NOT NULL DEFAULT '{}' CHECK (NULL != ANY(image_urls)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE offer_status AS ENUM ('pending', 'accepted', 'declined');
CREATE TABLE offers (
    offer_id SERIAL PRIMARY KEY,
    order_id INT NOT NULL REFERENCES offers(offer_id) ON DELETE CASCADE,
    user_id INT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    status offer_status NOT NULL DEFAULT 'pending',
    ordered_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,
    sender_id INT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    receiver_id INT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sent_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE reviews (
    review_id SERIAL PRIMARY KEY,
    user_reviewed INT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    user_reviewing INT REFERENCES users(user_id) ON DELETE CASCADE,
    rating INT NOT NULL CHECK (rating BETWEEN 1 AND 5),
    content TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE EXTENSION IF NOT EXISTS pg_trgm;


CREATE OR REPLACE FUNCTION notify_message_insert()
RETURNS TRIGGER AS $$
DECLARE
    payload JSON;
BEGIN
    payload := json_build_object(
        'message_id', NEW.message_id,
        'sender_id', NEW.sender_id,
        'receiver_id', NEW.receiver_id,
        'content', NEW.content,
        'sent_at', NEW.sent_at
    );

    PERFORM pg_notify(NEW.receiver_id::text, payload::text);
    PERFORM pg_notify(NEW.sender_id::text, payload::text);

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER message_insert_notify
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION notify_message_insert();
