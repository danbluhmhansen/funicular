namespace Funicular.Client.Pages;

using System.Net.Http.Json;
using System.Text.Json;
using System.Threading.Tasks;

using Microsoft.AspNetCore.Components;

public partial class Characters
{
    [Inject]
    public HttpClient? HttpClient { get; set; }

    private IEnumerable<Character>? characters;

    protected override async Task OnInitializedAsync()
    {
        await base.OnInitializedAsync();

        if (HttpClient is null)
            return;

        var response = await HttpClient.PostAsJsonAsync(
            "https://localhost:7000/graphql",
            new GraphQuery { Query = "query{characters{id name}}", }
        );
        var graph = await response.Content.ReadFromJsonAsync<Graph>();
        characters = graph?.Data.Characters;
    }
}

public class GraphQuery
{
    public string Query { get; set; }
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

public class Graph
{
    public GraphData Data { get; set; }
}
