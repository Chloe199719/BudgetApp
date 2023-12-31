-- Add up migration script here
CREATE TYPE transaction_type AS ENUM ('DEPOSIT', 'WITHDRAWAL');
CREATE TYPE alert_type AS ENUM ('OVER_LIMIT', 'BILL_REMINDER');

CREATE TABLE IF NOT EXISTS "categories" (
    Category_ID SERIAL PRIMARY KEY,
    USER_ID UUID  NOT NULL,
    CREATED_AT timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    Category_Name VARCHAR(255) NOT NULL,
    FOREIGN KEY (USER_ID) REFERENCES users(ID)
);

CREATE TABLE IF NOT EXISTS "transactions" (
    Transaction_ID SERIAL PRIMARY KEY,
    USER_ID UUID NOT NULL,
    AMOUNT FLOAT NOT NULL,
    type transaction_type NOT NULL,
    description VARCHAR(255) NOT NULL,
    DATE timestamptz NOT NULL,
    Category_ID INT NOT NULL,
    CREATED_AT timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (USER_ID) REFERENCES users(ID),
    FOREIGN KEY (Category_ID) REFERENCES categories(Category_ID)
);

CREATE TABLE Budgets (
    Budget_ID SERIAL PRIMARY KEY,
    USER_ID UUID  NOT NULL,
    AMOUNT FLOAT NOT NULL,
    CATEGORY_ID INT NOT NULL,
    Limit_Amount FLOAT NOT NULL,
    Start_Date timestamptz NOT NULL,
    End_Date timestamptz NOT NULL,
    CREATED_AT timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (USER_ID) REFERENCES users(ID),
    FOREIGN KEY (CATEGORY_ID) REFERENCES categories(Category_ID)
);

CREATE TABLE IF NOT EXISTS Alerts (
    Alert_ID SERIAL PRIMARY KEY,
    USER_ID UUID  NOT NULL,
    Message VARCHAR(255) NOT NULL,
    type alert_type NOT NULL,
    triggeredOn timestamptz,
    CREATED_AT timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (USER_ID) REFERENCES users(ID)
);
