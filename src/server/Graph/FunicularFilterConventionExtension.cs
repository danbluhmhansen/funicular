namespace Funicular.Server.Graph;

using Funicular.Server.Data.Models;

using HotChocolate.Data.Filters;

public class FunicularFilterConventionExtensions : FilterConventionExtension
{
    protected override void Configure(IFilterConventionDescriptor descriptor)
    {
        base.Configure(descriptor);
        descriptor.BindRuntimeType<CharacterId, UuidOperationFilterInputType>();
    }
}