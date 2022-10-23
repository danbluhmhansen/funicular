using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Funicular.Client;
using Funicular.Client.Graph;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });

builder.Services
    .AddFunicularClient()
    .ConfigureHttpClient(client => client.BaseAddress = new(builder.HostEnvironment.BaseAddress + "graphql"));

await builder.Build().RunAsync();
