using System;

using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Funicular.Server.Data.Migrations;

public partial class WeatherForecasts : Migration
{
    protected override void Up(MigrationBuilder migrationBuilder)
    {
        migrationBuilder.CreateTable(
            name: "WeatherForecasts",
            columns: table =>
                new
                {
                    Date = table.Column<DateTime>(type: "date", nullable: false),
                    TemperatureC = table.Column<int>(type: "integer", nullable: false),
                    Summary = table.Column<string>(type: "text", nullable: true)
                },
            constraints: table =>
            {
                table.PrimaryKey("PK_WeatherForecasts", x => x.Date);
            }
        );
    }

    protected override void Down(MigrationBuilder migrationBuilder)
    {
        migrationBuilder.DropTable(name: "WeatherForecasts");
    }
}