using System;
using System.Text.Json;

using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Funicular.Server.Data.Migrations;

public partial class Init : Migration
{
    protected override void Up(MigrationBuilder migrationBuilder)
    {
        migrationBuilder.CreateTable(
            name: "Characters",
            columns: table =>
                new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false),
                    Name = table.Column<string>(type: "text", nullable: false),
                    Json = table.Column<JsonElement>(type: "jsonb", nullable: false)
                },
            constraints: table =>
            {
                table.PrimaryKey("PK_Characters", x => x.Id);
            }
        );
    }

    protected override void Down(MigrationBuilder migrationBuilder)
    {
        migrationBuilder.DropTable(name: "Characters");
    }
}