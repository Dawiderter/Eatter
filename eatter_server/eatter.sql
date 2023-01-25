CREATE DATABASE eatter;
USE eatter;
CREATE USER 'server'@'localhost' IDENTIFIED BY 'server';
SET PASSWORD FOR 'server'@'localhost' = PASSWORD('<enter_pass>');
GRANT EXECUTE ON eatter.* TO 'server'@'localhost';
GRANT SELECT, INSERT ON eatter.meals TO 'server'@'localhost';
GRANT SELECT, INSERT ON eatter.comments TO 'server'@'localhost';
GRANT SELECT, INSERT ON eatter.locals TO 'server'@'localhost';
GRANT SELECT, INSERT ON eatter.reviews TO 'server'@'localhost';
GRANT SELECT, INSERT ON eatter.companies TO 'server'@'localhost';
GRANT SELECT ON eatter.mods TO 'server'@'localhost';

CREATE TABLE users (
	id int NOT NULL AUTO_INCREMENT,
	email varchar(30) NOT NULL UNIQUE, 
	nick varchar(15) NOT NULL,
	bio varchar(200),
	pass_hash varchar(256) NOT NULL,
	PRIMARY KEY (id)
);

CREATE TABLE sessions (
	session varchar(256) NOT NULL,
	user_id int NOT NULL,
	PRIMARY KEY (session),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE companies ( 
	id int NOT NULL AUTO_INCREMENT,
	name varchar(30) NOT NULL UNIQUE,
	user_id int UNIQUE,
	PRIMARY KEY (id),
	FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE locals (
	id int NOT NULL AUTO_INCREMENT,
	name varchar(30) NOT NULL,
	phone_num varchar(12) NOT NULL,
	contact_email varchar(30) NOT NULL,
	address varchar(60) NOT NULL,
	company_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (company_id) REFERENCES companies(id)
); 

CREATE TABLE meals (
	id int NOT NULL AUTO_INCREMENT,
	price decimal(6,2) NOT NULL,
	name varchar(30) NOT NULL, 
	local_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (local_id) REFERENCES locals(id)
);

CREATE TABLE reviews (
	id int NOT NULL AUTO_INCREMENT,
	body varchar(300) NOT NULL, 
	created_at datetime NOT NULL, 
	score int NOT NULL, 
	meal_id int NOT NULL, 
	author_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (meal_id) REFERENCES meals(id),
	FOREIGN KEY (author_id) REFERENCES users(id)
);

CREATE TABLE comments (
	id int NOT NULL AUTO_INCREMENT, 
	body varchar(150) NOT NULL, 
	created_at datetime NOT NULL, 
	review_id int NOT NULL, 
	author_id int NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (review_id) REFERENCES reviews(id),
	FOREIGN KEY (author_id) REFERENCES users(id)
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
	FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE followers (
	follower int NOT NULL, 
	followed int NOT NULL, 
	PRIMARY KEY (follower, followed),
	FOREIGN KEY (follower) REFERENCES users(id),
	FOREIGN KEY (followed) REFERENCES users(id)
);

CREATE VIEW comment_items AS SELECT c.id AS c_id, c.body AS c_body, c.created_at AS c_created_at, c.review_id AS r_id, u.id AS u_id FROM comments c JOIN users u ON u.id = c.author_id;  

CREATE VIEW feed AS SELECT r.id AS r_id, r.body AS r_body, r.created_at AS r_created_at, r.score AS r_score, u.id AS u_id, u.nick AS u_nick, m.id AS m_id, m.name AS m_name, l.id AS l_id, l.name AS l_name 
	FROM reviews r JOIN meals m ON r.meal_id = m.id JOIN locals l ON m.local_id = l.id JOIN users u ON u.id = r.author_id;

CREATE VIEW meal_items AS SELECT m.id AS m_id, m.price AS m_price, m.name AS m_name, l.id AS l_id, l.name AS l_name 
	FROM meals m JOIN locals l ON m.local_id = l.id;

CREATE VIEW user_items AS SELECT u.id AS u_id, u.nick AS u_nick, u.bio AS u_bio FROM users u;


GRANT SELECT, INSERT ON eatter.meal_items TO 'server'@'localhost';
GRANT SELECT, INSERT ON eatter.feed TO 'server'@'localhost';

DROP PROCEDURE createSession;
DELIMITER //
CREATE PROCEDURE createSession(IN user_id int)
BEGIN
    IF NOT EXISTS (SELECT * FROM sessions WHERE sessions.user_id = user_id) THEN
        INSERT INTO sessions(session, user_id) VALUES (SHA2(user_id,256), user_id);
    END IF;
    SELECT sessions.session FROM sessions WHERE sessions.user_id = user_id;
END//
DELIMITER ;

DROP PROCEDURE removeSessionFromId;
DELIMITER //
CREATE PROCEDURE removeSessionFromId(IN user_id int)
BEGIN
    DELETE FROM sessions WHERE sessions.user_id = user_id;
END//
DELIMITER ;
DROP PROCEDURE removeSession;
DELIMITER //
CREATE PROCEDURE removeSession(IN session varchar(256))
BEGIN
    DELETE FROM sessions WHERE sessions.session = session;
END//
DELIMITER ;

DROP PROCEDURE getUserFromSession;
DELIMITER //
CREATE PROCEDURE getUserFromSession(IN session varchar(256))
BEGIN
    SELECT sessions.user_id FROM sessions WHERE sessions.session = session;
END//
DELIMITER ;

DROP PROCEDURE getPassFromEmail;
DELIMITER //
CREATE PROCEDURE getPassFromEmail(IN email varchar(30))
BEGIN
    SELECT users.pass_hash FROM users WHERE users.email = email;
END//
DELIMITER ;

DROP PROCEDURE addUser;
DELIMITER //
CREATE PROCEDURE addUser(IN email varchar(30), IN nick varchar(15), IN pass varchar(256))
BEGIN
    INSERT INTO users(email, nick, pass_hash) VALUES (email, nick, pass);
END//
DELIMITER ;


DELIMITER //
CREATE FUNCTION verifyUser(email varchar(30), pass_hash varchar(256))
RETURNS int
BEGIN
    DECLARE user_id INT DEFAULT -1;
    SET user_id = (SELECT users.id FROM users WHERE users.email = email AND users.pass_hash = pass_hash);
    RETURN user_id;
END//
DELIMITER ;

DROP PROCEDURE loginUser;
DELIMITER //
CREATE PROCEDURE loginUser(IN email varchar(30), IN pass_hash varchar(256))
BEGIN
    DECLARE user_id INT;
    SET user_id = verifyUser(email, pass_hash);
    IF (user_id > -1) THEN
        CALL createSession(user_id);
    END IF;
END//
DELIMITER ;

DROP PROCEDURE getUserIDByEmail;
DELIMITER //
CREATE PROCEDURE getUserIDByEmail(IN email varchar(30))
    SELECT users.id FROM users WHERE users.email = email;
BEGIN
END//
DELIMITER ;

DROP PROCEDURE getLocalsForCompany;
DELIMITER //
CREATE PROCEDURE getLocalsForCompany(IN user_id int)
BEGIN
    DECLARE company_id int;
    SET company_id = (SELECT id FROM companies WHERE companies.user_id = user_id);
    SELECT id, name, phone_num, contact_email, address FROM locals WHERE locals.company_id = company_id;
END//
DELIMITER ;

DROP PROCEDURE addLocal;
DELIMITER //
CREATE PROCEDURE addLocal(IN user_id int, IN name varchar(30), IN phone_num varchar(12), IN contact_email varchar(30), IN address varchar(60))
BEGIN
    DECLARE company_id int;
    SET company_id = (SELECT id FROM companies WHERE companies.user_id = user_id);
    INSERT INTO locals(name, phone_num, contact_email, address, company_id) VALUES
    (name, phone_num, contact_email, address, company_id);
END //
DELIMITER ;

DROP PROCEDURE addMeal;
DELIMITER //
CREATE PROCEDURE addMeal(IN price INT, IN name varchar(30), local_id INT)
BEGIN
    INSERT INTO meals(price, name, local_id) VALUES
        (price, name, local_id);
END//
DELIMITER ;

DROP PROCEDURE addCompany;
DELIMITER //
CREATE PROCEDURE addCompany(IN email varchar(30), IN nick varchar(15), IN pass varchar(256), IN company_name varchar(30))
BEGIN
    DECLARE user_id int;
    CALL addUser(email, nick, pass);
    SET user_id = verifyUser(email, pass);
    INSERT INTO companies(name, user_id) VALUES (name, user_id);
END//
DELIMITER ;


DELIMITER //
CREATE PROCEDURE getMealsFromLocal(IN local_id INT)
BEGIN
    SELECT * FROM meals WHERE meals.local_id = local_id;
END//
DELIMITER ;


DELIMITER //
CREATE PROCEDURE getLocals()
BEGIN
    SELECT * FROM locals;
END//
DELIMITER ;


DELIMITER //
CREATE PROCEDURE addReview(IN body varchar(300), IN score INT, IN meal_id INT, IN author_id INT)
BEGIN
    DECLARE currdate datetime;
    SET currdate = NOW();
    INSERT INTO reviews(body, created_at, score, meal_id, author_id) VALUES
        (body, currdate, score, meal_id, author_id);
END//
DELIMITER ;


DELIMITER //
CREATE PROCEDURE getReviewsForMeal(IN meal_id INT)
BEGIN
    SELECT * FROM reviews WHERE reviews.meal_id = meal_id;
END//
DELIMITER ;


DROP PROCEDURE getGlobalFeed;
DELIMITER //
CREATE PROCEDURE getGlobalFeed()
BEGIN
	SELECT COUNT(c.id) AS comm_num, r.*, m.*, l.name as l_name FROM reviews r 
	JOIN meals m ON r.meal_id = m.id 
	JOIN locals l ON m.local_id = l.id 
	LEFT JOIN comments c ON c.review_id = r.id
	GROUP BY r.id; 
END//
DELIMITER ;


DELIMITER //
CREATE PROCEDURE addComment(IN body varchar(150), IN review_id INT, IN author_id INT)
BEGIN
    DECLARE currdate datetime;
    SET currdate = NOW();
    INSERT INTO comments(body, created_at, review_id, author_id) VALUES
        (body, currdate, review_id, author_id);
END//


DELIMITER //
CREATE PROCEDURE getPost(IN review_id INT)
BEGIN
    SELECT * FROM reviews WHERE reviews.id = review_id;
END//
DELIMITER ;


DELIMITER //
CREATE PROCEDURE addTagForMeal(IN tag_name varchar(30), IN meal_id INT)
BEGIN
	DECLARE tag_id INT;
	IF NOT EXISTS (SELECT * FROM tags WHERE tags.name = tag_name) THEN
		INSERT INTO tags(name) VALUES (name);
	END IF;
	SET tag_id = (SELECT tags.id FROM tags WHERE tags.name = tag_name);
	INSERT INTO meals_tags(meal_id, tag_id) VALUES(meal_id, tag_id);
END//
DELIMITER ;