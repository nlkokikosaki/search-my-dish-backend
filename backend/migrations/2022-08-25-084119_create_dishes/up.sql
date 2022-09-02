-- Your SQL goes here
CREATE TABLE IF NOT EXISTS dishes
(
    id SERIAL NOT NULL,
    name VARCHAR(100) NOT NULL,
    created_at DATE DEFAULT CURRENT_DATE,
    image         VARCHAR(100),
    content       VARCHAR(100),
    PRIMARY KEY (id)
);

INSERT INTO dishes (name, image, content) VALUES 
('コンビニ弁当', 'image1.webp', '食べ過ぎ注意'),
('牛丼', 'image2.webp', 'どこ派？'),
('カツ丼', 'image3.webp', '作るとなると大変'),
('海鮮丼', 'image4.webp', '意外とお店がなくて困る'),
('ハンバーグ', 'image5.webp', '王道ファミレス'),
('ステーキ', 'image6.webp', '王道ファミレス'),
('オムライス', 'image7.webp', 'ふわとろかしっかり焼きか'),
('やきそば', 'image8.webp', 'やきそば弁当美味しいよ'),
('カップ麺', 'image9.webp', '食べ過ぎ注意'),
('パスタサラダ', 'image10.webp', 'たまにはヘルシー路線で'),
('うどん', 'image11.webp', '冷凍うどんは常備しとくと便利'),
('ラーメン', 'image12.webp', '味噌');