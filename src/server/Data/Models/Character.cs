namespace Funicular.Server.Data.Models;

using System.Text.Json;

using Funicular.Server.Graph.Types;

using StronglyTypedIds;

[StronglyTypedId]
public partial struct CharacterId { }

public record Character(CharacterId Id, string Name, [property: GraphQLType(typeof(JsonType))] JsonDocument Json);