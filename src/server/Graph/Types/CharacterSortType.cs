namespace Funicular.Server.Graph.Types;

using Funicular.Server.Data.Models;

using HotChocolate.Data.Sorting;

public class CharacterSortType : SortInputType<Character>
{
    protected override void Configure(ISortInputTypeDescriptor<Character> descriptor)
    {
        base.Configure(descriptor);
        descriptor.Ignore(_ => _.Json);
    }
}