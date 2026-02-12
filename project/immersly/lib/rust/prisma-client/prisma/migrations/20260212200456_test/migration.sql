/*
  Warnings:

  - You are about to drop the column `kind` on the `Resource` table. All the data in the column will be lost.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Resource" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "state" TEXT NOT NULL,
    "hash" TEXT NOT NULL,
    "path" TEXT NOT NULL,
    "managed" BOOLEAN NOT NULL,
    "mime_type" TEXT NOT NULL,
    "last_access" DATETIME,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO "new_Resource" ("created_at", "hash", "id", "last_access", "managed", "mime_type", "path", "state") SELECT "created_at", "hash", "id", "last_access", "managed", "mime_type", "path", "state" FROM "Resource";
DROP TABLE "Resource";
ALTER TABLE "new_Resource" RENAME TO "Resource";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
