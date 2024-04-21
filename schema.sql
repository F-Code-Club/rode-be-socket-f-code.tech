/* 
    Table definition generated using mysqldump
    For reference purpose only
*/

CREATE TABLE IF NOT EXISTS account (
    `id` varchar(36) NOT NULL,
    `fname` varchar(255) NOT NULL,
    `lname` varchar(255) NOT NULL,
    `email` varchar(255) NOT NULL,
    `studentId` varchar(255) NOT NULL,
    `phone` varchar(255) NOT NULL,
    `dob` datetime NOT NULL,
    `role` enum('admin','user') NOT NULL DEFAULT 'user',
    `isActive` tinyint(4) NOT NULL DEFAULT 1,
    `createdAt` datetime(6) NOT NULL DEFAULT current_timestamp(6),
    `isLoggedIn` tinyint(4) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE KEY `IDX_4c8f96ccf523e9a3faefd5bdd4` (`email`),
    UNIQUE KEY `IDX_2109940ab8cb8ef76e13ba6cef` (`studentId`),
    UNIQUE KEY `IDX_a13e2234cf22b150ea2e72fba6` (`phone`)
);

CREATE TABLE IF NOT EXISTS local_file (
    `id` varchar(36) NOT NULL,
    `path` varchar(255) NOT NULL,
    `isUsed` tinyint(4) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS question (
    `id` varchar(36) NOT NULL,
    `questionImage` varchar(255) NOT NULL,
    `maxSubmitTimes` int(11) NOT NULL DEFAULT 5,
    `colors` varchar(255) DEFAULT NULL,
    `codeTemplate` text DEFAULT NULL,
    `roomId` varchar(36) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `FK_a70cec821dffadff31117ff3027` (`roomId`),
    CONSTRAINT `FK_a70cec821dffadff31117ff3027` FOREIGN KEY (`roomId`) REFERENCES `room` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS question_test_case (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `input` varchar(255) NOT NULL,
    `output` varchar(255) NOT NULL,
    `questionId` varchar(36) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `FK_cc548b5d340bf0aa138b81fbffb` (`questionId`),
    CONSTRAINT `FK_cc548b5d340bf0aa138b81fbffb` FOREIGN KEY (`questionId`) REFERENCES `question` (`id`) ON DELETE CASCADE ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS room (
    `id` varchar(36) NOT NULL,
    `code` varchar(255) NOT NULL,
    `openTime` datetime NOT NULL,
    `closeTime` datetime DEFAULT NULL,
    `duration` int(11) DEFAULT NULL,
    `type` enum('BE','FE') NOT NULL,
    `isPrivate` tinyint(4) NOT NULL DEFAULT 0,
    `createdAt` datetime(6) NOT NULL DEFAULT current_timestamp(6),
    PRIMARY KEY (`id`),
    UNIQUE KEY `IDX_0ab3536ee398cffd79acd2803c` (`code`)
);

CREATE TABLE IF NOT EXISTS submit_history (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `score` float NOT NULL,
    `language` enum('C_CPP','JAVA','PYTHON') DEFAULT NULL,
    `submissions` text NOT NULL,
    `submittedAt` datetime(6) NOT NULL DEFAULT current_timestamp(6),
    `time` int(11) DEFAULT NULL,
    `space` int(11) DEFAULT NULL,
    `link` varchar(255) DEFAULT NULL,
    `accountId` varchar(36) DEFAULT NULL,
    `questionId` varchar(36) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `FK_f39dc2addfce15a98ea9ae512e1` (`accountId`),
    KEY `FK_ffa0a6aaa0b8a6032c1a25f975f` (`questionId`),
    CONSTRAINT `FK_f39dc2addfce15a98ea9ae512e1` FOREIGN KEY (`accountId`) REFERENCES `account` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT `FK_ffa0a6aaa0b8a6032c1a25f975f` FOREIGN KEY (`questionId`) REFERENCES `question` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS user_room (
    `id` varchar(36) NOT NULL,
    `joinTime` datetime(6) NOT NULL DEFAULT current_timestamp(6),
    `finishTime` datetime DEFAULT NULL,
    `attendance` tinyint(4) NOT NULL DEFAULT 0,
    `accountId` varchar(36) DEFAULT NULL,
    `roomId` varchar(36) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `FK_29d3d96325adb0468ac2ee1676d` (`accountId`),
    KEY `FK_a074a2b9287d9941dcf5144bffe` (`roomId`),
    CONSTRAINT `FK_29d3d96325adb0468ac2ee1676d` FOREIGN KEY (`accountId`) REFERENCES `account` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT `FK_a074a2b9287d9941dcf5144bffe` FOREIGN KEY (`roomId`) REFERENCES `room` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
);
