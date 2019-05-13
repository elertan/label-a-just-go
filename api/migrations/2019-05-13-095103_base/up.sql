CREATE TABLE user_account
(
    first_name VARCHAR(50) NOT NULL,
    surname VARCHAR(50) NOT NULL,
    UUID uuid NOT NULL,
    PRIMARY KEY (UUID)
);

CREATE TABLE picture
(
    file_name VARCHAR(200) NOT NULL,
    ID INT NOT NULL,
    UUID uuid NOT NULL,
    PRIMARY KEY (ID),
    FOREIGN KEY (UUID) REFERENCES user_account(UUID)
);

CREATE TABLE invitation
(
    User_UUID uuid NOT NULL,
    Event_ID INT NOT NULL,
    ID INT NOT NULL,
    UUID uuid NOT NULL,
    PRIMARY KEY (ID),
    FOREIGN KEY (UUID) REFERENCES user_account(UUID)
);

CREATE TABLE event
(
    ID INT NOT NULL,
    Inv_ID INT NOT NULL,
    PRIMARY KEY (ID),
    FOREIGN KEY (Inv_ID) REFERENCES invitation(ID)
);

CREATE TABLE registration
(
    ID INT NOT NULL,
    Event_ID INT NOT NULL,
    User_UUID uuid NOT NULL,
    PRIMARY KEY (ID),
    FOREIGN KEY (User_UUID) REFERENCES user_account(UUID),
    FOREIGN KEY (Event_ID) REFERENCES event(ID)
);