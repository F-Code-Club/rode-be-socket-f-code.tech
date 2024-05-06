/* 
    Table definition for reference purpose only
*/

CREATE TABLE IF NOT EXISTS scores(
    id char(36) NOT NULL,
    room_id int NOT NULL,
    team_id char(36) NOT NULL,
    total_scores int unsigned NOT NULL,
    last_submit_time timestamp NOT NULL
 );

CREATE TABLE IF NOT EXISTS rooms(
    id int NOT NULL AUTO_INCREMENT,
    code varchar(12) NOT NULL,
    stack_id char(36) NOT NULL,
    size int NOT NULL,
    type enum("FE", "BE") NOT NULL, 
    open_time datetime NOT NULL,
    close_time datetime NOT NULL,
    created_at date NOT NULL,
    is_privated boolean NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS question_stacks(
    id char(36) NOT NULL,
    stack_max int NOT NULL,
    name varchar(128) NOT NULL,
    status enum("DRAFT", "ACTIVE", "DE_ACTIVE", "USED") NOT NULL,
    created_at date not NULL,
    type enum("FE", "BE") NOT NULl
);

CREATE TABLE IF NOT EXISTS questions(
    id char(36) NOT NULL,
    stack_id char(36) NOT NULL,
    max_submit_time int NOT NULL,
    score int unsigned NOT NULL
);

CREATE TABLE IF NOT EXISTS templates(
    id int NOT NULL AUTO_INCREMENT,
    question_id char(36) NOT NULL,
    local_path varchar(64) NOT NULL,
    url varchar(64) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS testcases(
    id int NOT NULL AUTO_INCREMENT,
    question_id char(36) NOT NULL,
    input text NOT NULL,
    output text NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS submit_histories(
    question_id char(36) NOT NULL,
    score_id int unsigned NOT NULL,
    member_id char(36) NOT NULL,
    submit_number int NOT NULL,
    run_tme int unsigned NOT NULL,
    score int unsigned NOT NULL,
    language enum("JAVA", "Python", "C_CPP", "CSS") NOT NULL,
    character_count int unsigned NOT NULL,
    last_submit_time timestamp NOT NULL,
    submissions text NOT NULL
);

CREATE TABLE IF NOT EXISTS teams(
    id int NOT NULL AUTO_INCREMENT,
    name varchar(128) NOT NULL,
    member_count int NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS members(
    id int NOT NULL AUTO_INCREMENT,
    team_id int unsigned NOT NULL,
    account_id char(36) NOT NULL,
    has_joined_room boolean NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS accounts(
    id char(36) NOT NULL,
    full_name varchar(48) NOT NULL,
    student_id varchar(24) NOT NULL,
    email varchar(30) NOT NULL,
    password varchar(128) NOT NULL,
    phone varchar(12) NOT NULL,
    dob date NOT NULL,
    role enum("admin", "manager", 'user') NOT NULL,
    created_at date NOT NULL,
    updated_at date NOT NULL,
    is_locked boolean NOT NULL,
    is_logged_in boolean NOT NULL,
    is_enabled boolean NOT NULL
)
