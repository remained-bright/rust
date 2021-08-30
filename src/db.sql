PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "fav" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"create_time"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"update_time"	INTEGER NOT NULL DEFAULT 0,
	"next_time"	INTEGER NOT NULL DEFAULT 0,
	"refresh_freq"	INTEGER NOT NULL DEFAULT 60,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "site_pk" (
	"id"	INTEGER NOT NULL,
	"val"	BLOB NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "site_ipv4" (
	"id"	INTEGER NOT NULL,
	"site_id"	INTEGER NOT NULL,
	"fail"	INTEGER NOT NULL,
	"rank"	INTEGER NOT NULL,
	"ip"	INTEGER NOT NULL,
	"port"	INT2 NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('fav',0);
INSERT INTO sqlite_sequence VALUES('site_pk',0);
INSERT INTO sqlite_sequence VALUES('site_ipv4',0);
CREATE INDEX "fav.next_time" ON "fav" (
	"next_time"	ASC
);
CREATE INDEX "fav.create_time" ON "fav" (
	"create_time"	DESC
);
CREATE INDEX "fav.update_time" ON "fav" (
	"update_time"	DESC
);
CREATE UNIQUE INDEX "site_ipv4.ip_id.ip.port" ON "site_ipv4" (
	"site_id",
	"ip",
	"port"
);
CREATE INDEX "site_ipv4.pk_id.rank" ON "site_ipv4" (
	"site_id"	ASC,
	"rank"	ASC
);
COMMIT;
PRAGMA journal_mode = WAL;
PRAGMA locking_mode = EXCLUSIVE;
PRAGMA temp_store = 2;
