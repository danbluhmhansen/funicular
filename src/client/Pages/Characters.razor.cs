namespace Funicular.Client.Pages;

using System.Net.Http.Json;
using System.Threading.Tasks;

using Microsoft.AspNetCore.Components;

public partial class Characters : ComponentBase
{
    [Parameter]
    [SupplyParameterFromQuery]
    public int PageIndex { get; set; }

    [Parameter]
    [SupplyParameterFromQuery]
    public int PageSize { get; set; }

    [Inject]
    public HttpClient? HttpClient { get; set; }

    [Inject]
    public NavigationManager? Navigation { get; set; }

    private int? count;
    private int? pageCount;
    private IEnumerable<Character>? characters;

    protected override async Task OnParametersSetAsync()
    {
        if (HttpClient is null)
            return;

        if (PageSize is 0)
            PageSize = 10;

        var response = await HttpClient.PostAsJsonAsync(
            "/graphql",
            new GraphQuery
            {
                Query =
                    @"
query CharactersQuery($skip: Int, $top: Int, $orderby: [Orderby]) {
    characters(count: true, skip: $skip, top: $top, orderby: $orderby) {
        id
        name
    }
}",
                Variables = new Dictionary<string, object> { { "skip", PageIndex * PageSize }, { "top", PageSize }, },
            }
        );

        var graph = await response.Content.ReadFromJsonAsync<Graph>();
        count = graph?.Extensions.Count;
        pageCount = count / PageSize;
        characters = graph?.Data.Characters;
    }

    private IEnumerable<int> Pages() =>
        pageCount.HasValue ? Enumerable.Range(1, pageCount.Value + 1) : Enumerable.Empty<int>();

    private void PreviousPage()
    {
        if (Navigation is not null && PageIndex is not 0)
            Navigation.NavigateTo(Navigation.GetUriWithQueryParameter(nameof(PageIndex), PageIndex - 1));
    }

    private void NextPage()
    {
        if (Navigation is not null && PageIndex < pageCount)
            Navigation.NavigateTo(Navigation.GetUriWithQueryParameter(nameof(PageIndex), PageIndex + 1));
    }
}

public class Foo
{
    public string Test { get; set; }
}

public class GraphQuery
{
    public string Query { get; set; }
    public IDictionary<string, object> Variables { get; set; }
}

public class Character
{
    public Guid Id { get; set; }
    public string Name { get; set; }
}

public class GraphData
{
    public IEnumerable<Character> Characters { get; set; }
}

public class GraphExtensions
{
    public int Count { get; set; }
}

public class Graph
{
    public GraphData Data { get; set; }
    public GraphExtensions Extensions { get; set; }
}
