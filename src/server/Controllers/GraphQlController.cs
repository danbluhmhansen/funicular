namespace Funicular.Server.Controllers;

using Funicular.Server.Data;

using GraphQL;
using GraphQL.Server.Transports.AspNetCore;
using GraphQL.Transport;
using GraphQL.Types;
using GraphQL.Validation;

using Microsoft.AspNetCore.Mvc;

using MoreLinq;

[Route("[controller]")]
public class GraphQlController : ControllerBase
{
    private readonly IDocumentExecuter<ISchema> executer;
    private readonly IGraphQLTextSerializer serializer;

    public GraphQlController(IDocumentExecuter<ISchema> executer, IGraphQLTextSerializer serializer)
    {
        this.executer = executer;
        this.serializer = serializer;
    }

    [HttpGet]
    public Task<IActionResult> GraphQLGetAsync(string query, string? operationName) =>
        HttpContext.WebSockets.IsWebSocketRequest
            ? Task.FromResult<IActionResult>(BadRequest())
            : ExecuteGraphQLRequestAsync(BuildRequest(query, operationName));

    [HttpPost]
    public async Task<IActionResult> GraphQLPostAsync()
    {
        if (HttpContext.Request.HasFormContentType)
        {
            var form = await HttpContext.Request.ReadFormAsync(HttpContext.RequestAborted);
            return await ExecuteGraphQLRequestAsync(
                BuildRequest(
                    form["query"].ToString(),
                    form["operationName"].ToString(),
                    form["variables"].ToString(),
                    form["extensions"].ToString()
                )
            );
        }
        else if (HttpContext.Request.HasJsonContentType())
        {
            var request = await serializer.ReadAsync<GraphQLRequest>(
                HttpContext.Request.Body,
                HttpContext.RequestAborted
            );
            return await ExecuteGraphQLRequestAsync(request);
        }
        return BadRequest();
    }

    private GraphQLRequest BuildRequest(
        string query,
        string? operationName,
        string? variables = null,
        string? extensions = null
    ) =>
        new()
        {
            Query = query == "" ? null : query,
            OperationName = operationName == "" ? null : operationName,
            Variables = serializer.Deserialize<Inputs>(variables == "" ? null : variables),
            Extensions = serializer.Deserialize<Inputs>(extensions == "" ? null : extensions),
        };

    private async Task<IActionResult> ExecuteGraphQLRequestAsync(GraphQLRequest? request)
    {
        try
        {
            IValidationRule rule = HttpMethods.IsGet(HttpContext.Request.Method)
                ? new HttpGetValidationRule()
                : new HttpPostValidationRule();
            ExecutionOptions opts =
                new()
                {
                    Query = request?.Query,
                    OperationName = request?.OperationName,
                    Variables = request?.Variables,
                    Extensions = request?.Extensions,
                    CancellationToken = HttpContext.RequestAborted,
                    RequestServices = HttpContext.RequestServices,
                    User = HttpContext.User,
                    ValidationRules = DocumentValidator.CoreRules.Append(rule),
                    CachedDocumentValidationRules = new[] { rule }
                };
            var result = await executer.ExecuteAsync(opts);

            var db = HttpContext.RequestServices.GetRequiredService<FunicularDbContext>();
            if (db.ChangeTracker.HasChanges())
                await db.SaveChangesAsync(HttpContext.RequestAborted);

            return new ExecutionResultActionResult(result);
        }
        catch
        {
            return BadRequest();
        }
    }
}