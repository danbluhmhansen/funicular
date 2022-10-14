using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;

using Funicular.Client;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddTransient<CustomAuthorizationMessageHandler>();

builder.Services
    .AddHttpClient("server", client => client.BaseAddress = new("https://localhost:7000"))
    .AddHttpMessageHandler<CustomAuthorizationMessageHandler>();

builder.Services.AddScoped(sp => sp.GetRequiredService<IHttpClientFactory>().CreateClient("server"));

builder.Services.AddOidcAuthentication(options =>
{
    options.AuthenticationPaths.RemoteRegisterPath = "https://localhost:7000/account/register";
    options.ProviderOptions.Authority = "https://localhost:7000";
    options.ProviderOptions.ClientId = "default";
    options.ProviderOptions.ResponseMode = "query";
    options.ProviderOptions.ResponseType = "code";
});

await builder.Build().RunAsync();
