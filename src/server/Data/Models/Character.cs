namespace Funicular.Server.Data.Models;

using System.Text.Json;

internal record Character(Guid Id, string Name, JsonElement? Json);