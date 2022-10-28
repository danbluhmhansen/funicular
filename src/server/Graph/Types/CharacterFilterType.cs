namespace Funicular.Server.Graph.Types;

using Funicular.Server.Data.Models;

using HotChocolate.Data.Filters;

public class CharacterFilterType : FilterInputType<Character>
{
    protected override void Configure(IFilterInputTypeDescriptor<Character> descriptor)
    {
        base.Configure(descriptor);
        descriptor.Ignore(_ => _.Json);
    }
}