-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Book" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT,
    "path" TEXT NOT NULL,
    "rendered_path" TEXT NOT NULL,
    "rendered_audio_path" TEXT,
    "image_resource_id" BLOB,
    "audio_resource_id" BLOB,
    CONSTRAINT "Book_image_resource_id_fkey" FOREIGN KEY ("image_resource_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "Book_audio_resource_id_fkey" FOREIGN KEY ("audio_resource_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);
INSERT INTO "new_Book" ("audio_resource_id", "id", "path", "rendered_audio_path", "rendered_path", "title") SELECT "audio_resource_id", "id", "path", "rendered_audio_path", "rendered_path", "title" FROM "Book";
DROP TABLE "Book";
ALTER TABLE "new_Book" RENAME TO "Book";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
