/*
  Warnings:

  - You are about to drop the column `media_id` on the `Book` table. All the data in the column will be lost.

*/
-- CreateTable
CREATE TABLE "BookProgress" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "timestamp" BIGINT NOT NULL,
    "book_id" BLOB NOT NULL,
    CONSTRAINT "BookProgress_book_id_fkey" FOREIGN KEY ("book_id") REFERENCES "Book" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT
);

-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Book" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT,
    "path" TEXT NOT NULL,
    "rendered_path" TEXT NOT NULL,
    "rendered_audio_path" TEXT,
    "audio_resource_id" BLOB,
    CONSTRAINT "Book_audio_resource_id_fkey" FOREIGN KEY ("audio_resource_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);
INSERT INTO "new_Book" ("audio_resource_id", "id", "path", "rendered_audio_path", "rendered_path", "title") SELECT "audio_resource_id", "id", "path", "rendered_audio_path", "rendered_path", "title" FROM "Book";
DROP TABLE "Book";
ALTER TABLE "new_Book" RENAME TO "Book";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;

-- CreateIndex
CREATE UNIQUE INDEX "BookProgress_book_id_key" ON "BookProgress"("book_id");
