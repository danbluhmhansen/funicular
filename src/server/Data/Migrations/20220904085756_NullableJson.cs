using System.Text.Json;

using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Funicular.Server.Data.Migrations;

public partial class NullableJson : Migration
{
    protected override void Up(MigrationBuilder migrationBuilder)
    {
        migrationBuilder.AlterColumn<JsonElement>(
            name: "Json",
            table: "Characters",
            type: "jsonb",
            nullable: true,
            oldClrType: typeof(JsonElement),
            oldType: "jsonb"
        );
    }

    protected override void Down(MigrationBuilder migrationBuilder)
    {
        migrationBuilder.AlterColumn<JsonElement>(
            name: "Json",
            table: "Characters",
            type: "jsonb",
            nullable: false,
            defaultValue: System.Text.Json.JsonDocument
                .Parse("", new System.Text.Json.JsonDocumentOptions())
                .RootElement,
            oldClrType: typeof(JsonElement),
            oldType: "jsonb",
            oldNullable: true
        );
    }
}