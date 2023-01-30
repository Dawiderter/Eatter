INSERT INTO companies(name) VALUES ("MCDonalds"), ("KFC"), ("Burger King");

INSERT INTO locals(name, phone_num, contact_email, address, company_id) VALUES
('MCD Plac Grunwaldzki', '623243932', 'placmcd@gmail.com', 'plac grunwaldzki 5', 1),
('KFC Swidnicka', '590462442', 'kfcswidnicka@gmail.com', 'swidnicka 15', 2),
('Burger King Rynek', '199887164', 'burgerrynek@gmail.com', 'rynek 7', 3);
 
INSERT INTO meals(price, name, local_id) VALUES
(25, 'Big Mac', 1),
(30, 'Grander', 2),
(22, 'Whooper', 3),
(23, 'McChicken', 1),
(10.5, 'Hamburger', 2),
(5.75, 'Shake', 3);

INSERT INTO tags(name) VALUES ('burger'), ('big'), ('shake');

INSERT INTO meals_tags(meal_id, tag_id) VALUES (1,1), (2,1), (3,1), (4,1), (5,1), (1,2), (6,3), (6,2);

UPDATE companies SET user_id = 1 WHERE id = 1;