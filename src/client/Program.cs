using Funicular.Client;
using Funicular.Client.Graph;

using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Microsoft.AspNetCore.Components.WebAssembly.Authentication;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

var services = builder.Services;

services.AddHttpClient("Server", client => client.BaseAddress = new(builder.HostEnvironment.BaseAddress));

// .AddHttpMessageHandler<BaseAddressAuthorizationMessageHandler>();
services.AddScoped(sp => sp.GetRequiredService<IHttpClientFactory>().CreateClient("Server"));

services
    .AddFunicularClient()
    .ConfigureHttpClient(client => client.BaseAddress = new(builder.HostEnvironment.BaseAddress + "graphql"));

services.AddOidcAuthentication(options =>
{
    options.AuthenticationPaths.RemoteRegisterPath = "https://localhost:7000/account/register";
    options.ProviderOptions.Authority = "https://localhost:7000";
    options.ProviderOptions.ClientId = "default";
    options.ProviderOptions.ResponseMode = "query";
    options.ProviderOptions.ResponseType = "code";
});

await builder.Build().RunAsync();
