namespace Funicular.Server.Data.Models;

internal record DynamicField(string Name, string Type, bool Required)
{
    public DynamicField(string name, string type) : this(name, type, default) { }
}