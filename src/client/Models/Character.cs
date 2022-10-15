namespace Funicular.Client.Models;

using System.Text.Json;
using System.Text.Json.Serialization;

public class Character
{
    public Guid Id { get; set; }
    public string? Name { get; set; }

    [JsonExtensionData]
    public IDictionary<string, JsonElement>? Data { get; set; }
}
