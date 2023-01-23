CREATE DATABASE eatter;
USE eatter;
CREATE USER 'server'@'localhost' IDENTIFIED BY 'server';
SET PASSWORD FOR 'server'@'localhost' = PASSWORD('<enter_pass>');
GRANT EXECUTE ON eatter.* TO 'server'@'localhost';

CREATE TABLE users (
	id int NOT NULL AUTO_INCREMENT,
	email varchar(30) NOT NULL UNIQUE, 
	nick varchar(15) NOT NULL,
	pass_hash varchar(256) NOT NULL,
	PRIMARY KEY (id)
);

CREATE TABLE sessions (
	session varchar(256) NOT NULL,
	user_id int NOT NULL,
	PRIMARY KEY (session),
	FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE companies ( 
	id int NOT NULL AUTO_INCREMENT,
	name varchar(30) NOT NULL UNIQUE,
	user_id int NOT NULL UNIQUE,
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
	price int NOT NULL,
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



DROP PROCEDURE createSession;
DELIMITER //
CREATE PROCEDURE createSession(IN user_id int)
BEGIN
    IF NOT EXISTS (SELECT * FROM sessions WHERE sessions.user_id = user_id) THEN
        INSERT INTO sessions(session, user_id) VALUES (PASSWORD(user_id), user_id);
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

DROP PROCEDURE addUser;
DELIMITER //
CREATE PROCEDURE addUser(IN email varchar(30), IN nick varchar(15), IN pass varchar(256))
BEGIN
    INSERT INTO users(email, nick, pass_hash) VALUES
    (email, nick, PASSWORD(pass));
END//
DELIMITER ;


DELIMITER //
CREATE FUNCTION verifyUser(email varchar(30), pass varchar(256))
RETURNS int
BEGIN
    DECLARE user_id INT DEFAULT -1;
    SET user_id = (SELECT users.id FROM users WHERE users.email = email AND users.pass_hash = PASSWORD(pass));
    RETURN user_id;
END//
DELIMITER ;

DROP PROCEDURE loginUser;
DELIMITER //
CREATE PROCEDURE loginUser(IN email varchar(30), IN pass varchar(256))
BEGIN
    DECLARE user_id INT;
    SET user_id = verifyUser(email, pass);
    IF (user_id > -1) THEN
        CALL createSession(user_id);
    END IF;
END//
DELIMITER ;

DROP PROCEDURE getUserIDByEmail;
DELIMITER //
CREATE PROCEDURE getUserIDByEmail(IN email varchar(30))
    SELECT users.id INTO user_id FROM users WHERE users.email = email;
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