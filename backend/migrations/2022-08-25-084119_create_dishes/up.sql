-- Your SQL goes here
CREATE TABLE IF NOT EXISTS dishes
(
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    image         TEXT,
    content       TEXT,
    PRIMARY KEY (id)
);

INSERT INTO dishes (name, image, content) VALUES ('コンビニ弁当', 'image.webp', '食べ過ぎ注意')