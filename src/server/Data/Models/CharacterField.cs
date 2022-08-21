namespace Funicular.Server.Data.Models;

internal record CharacterField(string Name, string Type, bool Required)
{
    public CharacterField(string name, string type) : this(name, type, default) { }
}
