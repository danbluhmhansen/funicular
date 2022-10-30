namespace Funicular.Server.Data.Models;

using StronglyTypedIds;

[StronglyTypedId]
public partial struct CharacterId { }

public record Character(CharacterId Id, string Name, IDictionary<string, int> Ints);