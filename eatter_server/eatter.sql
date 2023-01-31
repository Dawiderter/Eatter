CREATE DATABASE eatter;
USE eatter;
CREATE USER 'server'@'localhost' IDENTIFIED BY 'server';
SET PASSWORD FOR 'server'@'localhost' = PASSWORD('<enter_pass>');
GRANT EXECUTE ON eatter.* TO 'server'@'localhost';

CREATE TABLE users (
	id int NOT NULL AUTO_INCREMENT,
	email varchar(30) NOT NULL UNIQUE,
	pass_hash varchar(256) NOT NULL,
	PRIMARY KEY (id)
);

CREATE TABLE users_ext (
	id int NOT NULL,
	nick varchar(15) NOT NULL,
	bio varchar(200),
	PRIMARY KEY (id),
	FOREIGN KEY (id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE sessions (
	session varchar(256) NOT NULL,
	user_id int NOT NULL,
	expires_at datetime NOT NULL,
	PRIMARY KEY (session),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE companies ( 
	id int NOT NULL AUTO_INCREMENT,
	name varchar(30) NOT NULL UNIQUE,
	user_id int UNIQUE,
	PRIMARY KEY (id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE locals (
	id int NOT NULL AUTO_INCREMENT,
	name varchar(30) NOT NULL,
	phone_num varchar(12) NOT NULL,
	contact_email varchar(30) NOT NULL,
	address varchar(60) NOT NULL,
	company_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE
); 

CREATE TABLE meals (
	id int NOT NULL AUTO_INCREMENT,
	price decimal(6,2) UNSIGNED NOT NULL,
	name varchar(30) NOT NULL, 
	local_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (local_id) REFERENCES locals(id) ON DELETE CASCADE
);

CREATE TABLE reviews (
	id int NOT NULL AUTO_INCREMENT,
	body varchar(300) NOT NULL, 
	created_at datetime NOT NULL, 
	score int UNSIGNED NOT NULL, 
	meal_id int NOT NULL, 
	author_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (meal_id) REFERENCES meals(id) ON DELETE CASCADE,
	FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
	UNIQUE (meal_id, author_id),
	CHECK(score>0 AND score<6)
);

CREATE TABLE comments (
	id int NOT NULL AUTO_INCREMENT, 
	body varchar(150) NOT NULL, 
	created_at datetime NOT NULL, 
	review_id int NOT NULL, 
	author_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (review_id) REFERENCES reviews(id) ON DELETE CASCADE,
	FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
	CHECK(LENGTH(body) > 5)
);

CREATE TABLE tags (
	id int NOT NULL AUTO_INCREMENT, 
	name varchar(30) NOT NULL UNIQUE,
	PRIMARY KEY (id)
);

CREATE TABLE meals_tags ( 
	meal_id int NOT NULL,
	tag_id int NOT NULL,
	PRIMARY KEY (meal_id, tag_id)
);

CREATE TABLE mods (
	id int NOT NULL AUTO_INCREMENT, 
	user_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE followers (
	follower int NOT NULL, 
	followed int NOT NULL, 
	PRIMARY KEY (follower, followed),
	FOREIGN KEY (follower) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (followed) REFERENCES users(id) ON DELETE CASCADE,
	CHECK(follower <> followed)
);

CREATE TABLE review_ext(
	id int NOT NULL,
	c_num int NOT NULL DEFAULT 0,
	PRIMARY KEY (id),
	FOREIGN KEY (id) REFERENCES reviews(id) ON DELETE CASCADE
);

CREATE TABLE meal_ext(
	id int NOT NULL,
	r_num int NOT NULL DEFAULT 0,
	r_avg decimal(3,2) DEFAULT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (id) REFERENCES meals(id) ON DELETE CASCADE
);

CREATE VIEW comment_items AS SELECT c.id AS c_id, c.body AS c_body, c.created_at AS c_created_at, c.review_id AS r_id, u.id AS u_id, ue.nick AS u_nick 
	FROM comments c JOIN users u ON u.id = c.author_id JOIN users_ext ue ON ue.id = u.id;  

CREATE VIEW feed AS SELECT 
	r.id AS r_id, r.body AS r_body, r.created_at AS r_created_at, r.score AS r_score, 
	u.id AS u_id, ue.nick AS u_nick, 
	m.id AS m_id, m.name AS m_name, 
	l.id AS l_id, l.name AS l_name, 
	re.c_num AS r_c_num
	FROM reviews r 
	JOIN meals m ON r.meal_id = m.id 
	JOIN locals l ON m.local_id = l.id 
	JOIN users u ON u.id = r.author_id 
	JOIN users_ext ue ON ue.id = u.id
	JOIN review_ext re ON re.id = r.id;

CREATE VIEW meal_items AS SELECT 
	m.id AS m_id, m.price AS m_price, m.name AS m_name, 
	l.id AS l_id, l.name AS l_name,
	me.r_num AS m_r_num, me.r_avg AS m_r_avg
	FROM meals m JOIN locals l ON m.local_id = l.id JOIN meal_ext me ON me.id = m.id;

CREATE VIEW user_items AS SELECT u.id AS u_id, ue.nick AS u_nick, ue.bio AS u_bio FROM users u JOIN users_ext ue ON ue.id = u.id;

CREATE VIEW local_items AS SELECT l.id AS l_id, l.name AS l_name, l.phone_num AS l_phone_num, l.contact_email AS l_contact_email, l.address AS l_address, c.id AS c_id, c.name AS c_name 
	FROM locals l JOIN companies c ON l.company_id = c.id;

GRANT SELECT ON eatter.meal_items TO 'server'@'localhost';
GRANT SELECT ON eatter.local_items TO 'server'@'localhost';
GRANT SELECT ON eatter.user_items TO 'server'@'localhost';
GRANT SELECT ON eatter.comment_items TO 'server'@'localhost';
GRANT SELECT ON eatter.feed TO 'server'@'localhost';

GRANT SELECT, INSERT, UPDATE, DELETE ON eatter.meals TO 'server'@'localhost';
GRANT SELECT, INSERT, DELETE ON eatter.comments TO 'server'@'localhost';
GRANT SELECT, INSERT, UPDATE, DELETE ON eatter.locals TO 'server'@'localhost';
GRANT SELECT, INSERT, DELETE ON eatter.reviews TO 'server'@'localhost';
GRANT SELECT, INSERT, DELETE ON eatter.companies TO 'server'@'localhost';
GRANT SELECT, INSERT, UPDATE ON eatter.users_ext TO 'server'@'localhost';
GRANT SELECT, INSERT, DELETE ON eatter.followers TO 'server'@'localhost';
GRANT SELECT ON eatter.meals_tags TO 'server'@'localhost';
GRANT SELECT ON eatter.tags TO 'server'@'localhost';
GRANT SELECT ON eatter.mods TO 'server'@'localhost';

DROP TRIGGER r_ext_ins;
DELIMITER //
CREATE TRIGGER r_ext_ins AFTER INSERT ON reviews 
FOR EACH ROW
BEGIN
	INSERT INTO review_ext(id) VALUES (NEW.id);
END//
DELIMITER ;

DROP TRIGGER r_ext_upd;
DELIMITER //
CREATE TRIGGER r_ext_upd AFTER INSERT ON comments 
FOR EACH ROW
BEGIN
	UPDATE review_ext SET c_num = (SELECT COUNT(id) FROM comments WHERE review_id = NEW.review_id) WHERE id = NEW.review_id;
END//
DELIMITER ;


DROP TRIGGER r_ext_del;
DELIMITER //
CREATE TRIGGER r_ext_del AFTER DELETE ON comments 
FOR EACH ROW
BEGIN
	UPDATE review_ext SET c_num = (SELECT COUNT(id) FROM comments WHERE review_id = OLD.review_id) WHERE id = OLD.review_id;
END//
DELIMITER ;

DROP TRIGGER m_ext_ins;
DELIMITER //
CREATE TRIGGER m_ext_ins AFTER INSERT ON meals 
FOR EACH ROW
BEGIN
	INSERT INTO meal_ext(id) VALUES (NEW.id);
END//
DELIMITER ;

DROP TRIGGER m_ext_upd;
DELIMITER //
CREATE TRIGGER m_ext_upd AFTER INSERT ON reviews 
FOR EACH ROW
BEGIN
	UPDATE meal_ext SET r_num = (SELECT COUNT(id) FROM reviews WHERE meal_id = NEW.meal_id), 
	r_avg = (SELECT AVG(score) FROM reviews WHERE meal_id = NEW.meal_id) WHERE id = NEW.meal_id;
END//
DELIMITER ;


DROP TRIGGER m_ext_del;
DELIMITER //
CREATE TRIGGER m_ext_del AFTER DELETE ON reviews 
FOR EACH ROW
BEGIN
	UPDATE meal_ext SET r_num = (SELECT COUNT(id) FROM reviews WHERE meal_id = OLD.meal_id), 
	r_avg = (SELECT AVG(score) FROM reviews WHERE meal_id = OLD.meal_id) WHERE id = OLD.meal_id;
END//
DELIMITER ;

DROP PROCEDURE createSession;
DELIMITER //
CREATE PROCEDURE createSession(IN user_id int, IN token varchar(256), IN expires_at datetime)
BEGIN
    INSERT INTO sessions(session, user_id, expires_at) VALUES (token, user_id, expires_at);
END//
DELIMITER ;

DELIMITER ;
DROP PROCEDURE removeSession;
DELIMITER //
CREATE PROCEDURE removeSession(IN session varchar(256))
BEGIN
    DELETE FROM sessions WHERE sessions.session = session;
END//
DELIMITER ;

DROP PROCEDURE getSession;
DELIMITER //
CREATE PROCEDURE getSession(IN token varchar(256))
BEGIN
    SELECT session, user_id, expires_at FROM sessions WHERE sessions.session = token;
END//
DELIMITER ;

DROP PROCEDURE changeBio;
DELIMITER //
CREATE PROCEDURE changeBio(IN user_id int, IN bio_text varchar(200))
BEGIN
    UPDATE users SET bio = bio_text WHERE id = user_id;
END//
DELIMITER ;

DROP PROCEDURE getUserFromEmail;
DELIMITER //
CREATE PROCEDURE getUserFromEmail(IN email varchar(30))
BEGIN
    SELECT users.pass_hash, users.id FROM users WHERE users.email = email;
END//
DELIMITER ;

DROP PROCEDURE addUser;
DELIMITER //
CREATE PROCEDURE addUser(IN email varchar(30), IN pass_hash varchar(256))
BEGIN
    INSERT INTO users(email, pass_hash) VALUES (email, pass_hash);
	SELECT id FROM users u WHERE u.email = email AND u.pass_hash = pass_hash;
END//
DELIMITER ;

DROP PROCEDURE addTagForMeal;
DELIMITER //
CREATE PROCEDURE addTagForMeal(IN tag_name varchar(30), IN meal_id INT)
BEGIN
	DECLARE tag_id INT;
	IF NOT EXISTS (SELECT * FROM tags WHERE tags.name = tag_name) THEN
	INSERT INTO tags(name) VALUES (tag_name);
	END IF;
	SET tag_id = (SELECT tags.id FROM tags WHERE tags.name = tag_name);
	INSERT INTO meals_tags(meal_id, tag_id) VALUES(meal_id, tag_id);
END//
DELIMITER ;

-- DROP PROCEDURE removeSessionFromId;
-- DELIMITER //
-- CREATE PROCEDURE removeSessionFromId(IN user_id int)
-- BEGIN
--     DELETE FROM sessions WHERE sessions.user_id = user_id;
-- END//


-- DELIMITER //
-- CREATE FUNCTION verifyUser(email varchar(30), pass_hash varchar(256))
-- RETURNS int
-- BEGIN
--     DECLARE user_id INT DEFAULT -1;
--     SET user_id = (SELECT users.id FROM users WHERE users.email = email AND users.pass_hash = pass_hash);
--     RETURN user_id;
-- END//
-- DELIMITER ;

-- DROP PROCEDURE loginUser;
-- DELIMITER //
-- CREATE PROCEDURE loginUser(IN email varchar(30), IN pass_hash varchar(256))
-- BEGIN
--     DECLARE user_id INT;
--     SET user_id = verifyUser(email, pass_hash);
--     IF (user_id > -1) THEN
--         CALL createSession(user_id);
--     END IF;
-- END//
-- DELIMITER ;

-- DROP PROCEDURE getUserIDByEmail;
-- DELIMITER //
-- CREATE PROCEDURE getUserIDByEmail(IN email varchar(30))
--     SELECT users.id FROM users WHERE users.email = email;
-- BEGIN
-- END//
-- DELIMITER ;

-- DROP PROCEDURE getLocalsForCompany;
-- DELIMITER //
-- CREATE PROCEDURE getLocalsForCompany(IN user_id int)
-- BEGIN
--     DECLARE company_id int;
--     SET company_id = (SELECT id FROM companies WHERE companies.user_id = user_id);
--     SELECT id, name, phone_num, contact_email, address FROM locals WHERE locals.company_id = company_id;
-- END//
-- DELIMITER ;

-- DROP PROCEDURE addLocal;
-- DELIMITER //
-- CREATE PROCEDURE addLocal(IN user_id int, IN name varchar(30), IN phone_num varchar(12), IN contact_email varchar(30), IN address varchar(60))
-- BEGIN
--     DECLARE company_id int;
--     SET company_id = (SELECT id FROM companies WHERE companies.user_id = user_id);
--     INSERT INTO locals(name, phone_num, contact_email, address, company_id) VALUES
--     (name, phone_num, contact_email, address, company_id);
-- END //
-- DELIMITER ;

-- DROP PROCEDURE addMeal;
-- DELIMITER //
-- CREATE PROCEDURE addMeal(IN price INT, IN name varchar(30), local_id INT)
-- BEGIN
--     INSERT INTO meals(price, name, local_id) VALUES
--         (price, name, local_id);
-- END//
-- DELIMITER ;

-- DROP PROCEDURE addCompany;
-- DELIMITER //
-- CREATE PROCEDURE addCompany(IN email varchar(30), IN nick varchar(15), IN pass varchar(256), IN company_name varchar(30))
-- BEGIN
--     DECLARE user_id int;
--     CALL addUser(email, nick, pass);
--     SET user_id = verifyUser(email, pass);
--     INSERT INTO companies(name, user_id) VALUES (name, user_id);
-- END//
-- DELIMITER ;


-- DELIMITER //
-- CREATE PROCEDURE getMealsFromLocal(IN local_id INT)
-- BEGIN
--     SELECT * FROM meals WHERE meals.local_id = local_id;
-- END//
-- DELIMITER ;


-- DELIMITER //
-- CREATE PROCEDURE getLocals()
-- BEGIN
--     SELECT * FROM locals;
-- END//
-- DELIMITER ;


-- DELIMITER //
-- CREATE PROCEDURE addReview(IN body varchar(300), IN score INT, IN meal_id INT, IN author_id INT)
-- BEGIN
--     DECLARE currdate datetime;
--     SET currdate = NOW();
--     INSERT INTO reviews(body, created_at, score, meal_id, author_id) VALUES
--         (body, currdate, score, meal_id, author_id);
-- END//
-- DELIMITER ;


-- DELIMITER //
-- CREATE PROCEDURE getReviewsForMeal(IN meal_id INT)
-- BEGIN
--     SELECT * FROM reviews WHERE reviews.meal_id = meal_id;
-- END//
-- DELIMITER ;


-- DROP PROCEDURE getGlobalFeed;
-- DELIMITER //
-- CREATE PROCEDURE getGlobalFeed()
-- BEGIN
-- 	SELECT COUNT(c.id) AS comm_num, r.*, m.*, l.name as l_name FROM reviews r 
-- 	JOIN meals m ON r.meal_id = m.id 
-- 	JOIN locals l ON m.local_id = l.id 
-- 	LEFT JOIN comments c ON c.review_id = r.id
-- 	GROUP BY r.id; 
-- END//
-- DELIMITER ;


-- DELIMITER //
-- CREATE PROCEDURE addComment(IN body varchar(150), IN review_id INT, IN author_id INT)
-- BEGIN
--     DECLARE currdate datetime;
--     SET currdate = NOW();
--     INSERT INTO comments(body, created_at, review_id, author_id) VALUES
--         (body, currdate, review_id, author_id);
-- END//


-- DELIMITER //
-- CREATE PROCEDURE getPost(IN review_id INT)
-- BEGIN
--     SELECT * FROM reviews WHERE reviews.id = review_id;
-- END//
-- DELIMITER ;





-- DELIMITER //
-- CREATE TRIGGER check_user_data_ins BEFORE INSERT ON users FOR EACH getReviewsForMeal
-- BEGIN
-- END//
-- DELIMITER ;