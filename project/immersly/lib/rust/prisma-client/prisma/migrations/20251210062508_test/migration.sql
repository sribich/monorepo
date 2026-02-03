-- CreateTable
CREATE TABLE "setting" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "kind" TEXT NOT NULL,
    "value" TEXT NOT NULL,
    "constraints" TEXT
);

-- CreateIndex
CREATE UNIQUE INDEX "setting_name_unique" ON "setting"("name");
