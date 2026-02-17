/*
  Warnings:

  - You are about to drop the `Media` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Progress` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `TimestampedBook` table. If the table is not empty, all the data it contains will be lost.

*/
-- AlterTable
ALTER TABLE "Resource" ADD COLUMN "size" BIGINT;
ALTER TABLE "Resource" ADD COLUMN "type" TEXT;

-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "Media";
PRAGMA foreign_keys=on;

-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "Progress";
PRAGMA foreign_keys=on;

-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "TimestampedBook";
PRAGMA foreign_keys=on;
