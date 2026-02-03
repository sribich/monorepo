/*
  Warnings:

  - You are about to drop the `Library` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the column `library_id` on the `Media` table. All the data in the column will be lost.
  - You are about to drop the column `volume` on the `Media` table. All the data in the column will be lost.

*/
-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "Library";
PRAGMA foreign_keys=on;

-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Media" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "kind" TEXT NOT NULL,
    "image_resource_id" BLOB,
    CONSTRAINT "Media_image_resource_id_fkey" FOREIGN KEY ("image_resource_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);
INSERT INTO "new_Media" ("id", "kind", "title") SELECT "id", "kind", "title" FROM "Media";
DROP TABLE "Media";
ALTER TABLE "new_Media" RENAME TO "Media";
CREATE UNIQUE INDEX "library_title_uniq" ON "Media"("title");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
