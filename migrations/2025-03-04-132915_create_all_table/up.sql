-- Your SQL goes here

CREATE TABLE IF NOT EXISTS "article" (
	"id" VARCHAR(255) NOT NULL  ,
	"done" TINYINT NOT NULL DEFAULT '0' ,
	"publishDate" VARCHAR(255) NOT NULL DEFAULT '' ,
	"sectionId" VARCHAR(255) NOT NULL  ,
	"title" VARCHAR(255) NOT NULL  ,
	PRIMARY KEY ("id"),
	FOREIGN KEY("sectionId") REFERENCES "section" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
CREATE INDEX "FK_3a0b4db0ebe5e37bc9b09c4129c" ON "article" ("sectionId");
CREATE TABLE IF NOT EXISTS "article_comment" (
	"id" VARCHAR(255) NOT NULL  ,
	"content" TEXT NOT NULL  ,
	"likeCount" INTEGER NOT NULL  ,
	"nickName" VARCHAR(255) NOT NULL  ,
	"articleId" VARCHAR(255) NULL  ,
	"parentCommentId" VARCHAR(255) NULL  ,
	PRIMARY KEY ("id"),
	FOREIGN KEY("articleId") REFERENCES "article" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY("parentCommentId") REFERENCES "article_comment" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
CREATE INDEX "FK_4d5ab30629a42bad659fe1d4da6" ON "article_comment" ("articleId");
CREATE INDEX "FK_58ace9f389e6aecc2e8fd78c488" ON "article_comment" ("parentCommentId");
CREATE TABLE IF NOT EXISTS "course" (
	"id" VARCHAR(255) NOT NULL  ,
	"brief" VARCHAR(255) NOT NULL  ,
	"teacherName" VARCHAR(255) NOT NULL  ,
	"teacherTitle" VARCHAR(255) NOT NULL  ,
	"image" VARCHAR(255) NOT NULL  ,
	"articleCount" INTEGER NOT NULL DEFAULT '0' ,
	"purchasedCount" VARCHAR(255) NOT NULL  ,
	"done" TINYINT NOT NULL DEFAULT '0' ,
	"price" INTEGER NOT NULL  ,
	"title" VARCHAR(255) NOT NULL  ,
	PRIMARY KEY ("id")
);
CREATE TABLE IF NOT EXISTS "course_description" (
	"courseId" VARCHAR(255) NOT NULL  ,
	"content" TEXT NOT NULL  ,
	PRIMARY KEY ("courseId")
);
CREATE UNIQUE INDEX "REL_9b0571665a2fdd821fb8d3f6d0" ON "course_description" ("courseId");
CREATE TABLE IF NOT EXISTS "course_tend" (
	"courseId" VARCHAR(255) NOT NULL  ,
	"userId" VARCHAR(255) NOT NULL  ,
	"type" TEXT NOT NULL  ,
	PRIMARY KEY ("courseId", "userId")
);
CREATE TABLE IF NOT EXISTS "section" (
	"id" VARCHAR(255) NOT NULL  ,
	"courseId" VARCHAR(255) NOT NULL  ,
	"title" VARCHAR(255) NOT NULL  ,
	PRIMARY KEY ("id")
);
CREATE TABLE IF NOT EXISTS "user" (
	"id" VARCHAR(255) NOT NULL  ,
	"role" TEXT NOT NULL DEFAULT 'ORDINARY' ,
	PRIMARY KEY ("id")
);
CREATE TABLE IF NOT EXISTS "user_role" (
	"user_id" VARCHAR(255) NOT NULL  ,
	"role" INTEGER NOT NULL  ,
	"created_at" BIGINT NOT NULL  ,
	"valid_before" BIGINT NOT NULL  ,
	"id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT
);
CREATE INDEX "user_role_user_id_IDX" ON "user_role" ("user_id");
CREATE TABLE IF NOT EXISTS "user_study_info" (
	"id" INTEGER NOT NULL  PRIMARY KEY AUTOINCREMENT,
	"course_id" VARCHAR(255) NOT NULL  ,
	"article_id" VARCHAR(255) NOT NULL  ,
	"last_study_at" BIGINT NOT NULL DEFAULT '0' ,
	"study_percent" FLOAT NOT NULL DEFAULT '0' ,
	"user_id" VARCHAR(255) NOT NULL,
	FOREIGN KEY("course_id") REFERENCES "course" ("id") ON UPDATE RESTRICT ON DELETE RESTRICT,
	FOREIGN KEY("article_id") REFERENCES "article" ("id") ON UPDATE RESTRICT ON DELETE RESTRICT
);
CREATE UNIQUE INDEX "uniq_key" ON "user_study_info" ("user_id", "article_id");
CREATE INDEX "user_study_info_FK" ON "user_study_info" ("course_id");
CREATE INDEX "user_study_info_FK_1" ON "user_study_info" ("article_id");
CREATE INDEX "user_study_info_user_id_IDX" ON "user_study_info" ("user_id", "last_study_at");
CREATE TABLE IF NOT EXISTS "ws_connect_info" (
	"id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"user_id" VARCHAR(255) NOT NULL  ,
	"start_at" BIGINT NOT NULL  ,
	"end_at" BIGINT NOT NULL DEFAULT '0'
);
CREATE INDEX "ws_connect_info_user_id_start_at_index" ON "ws_connect_info" ("user_id", "start_at");
