using Funicular.Server.Commands;
using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph;
using Funicular.Server.Services;

using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

using OpenIddict.Server;
using OpenIddict.Server.AspNetCore;
using OpenIddict.Validation.AspNetCore;

using Quartz;

using StronglyTypedIds;

using static OpenIddict.Abstractions.OpenIddictConstants;

[assembly: StronglyTypedIdDefaults(
    converters: StronglyTypedIdConverter.TypeConverter
        | StronglyTypedIdConverter.SystemTextJson
        | StronglyTypedIdConverter.EfCoreValueConverter
)]

var builder = WebApplication.CreateBuilder(args);

var configuration = builder.Configuration;
var services = builder.Services;

services.AddControllersWithViews();

services.AddDbContext<FunicularDbContext>(options =>
{
    options.UseNpgsql(configuration.GetConnectionString("Default"));
    options.UseOpenIddict();
});

services.AddAuthentication(OpenIddictValidationAspNetCoreDefaults.AuthenticationScheme);

services
    .AddIdentity<FunicularUser, IdentityRole>()
    .AddEntityFrameworkStores<FunicularDbContext>()
    .AddDefaultTokenProviders();

services.AddQuartz(options =>
{
    options.UseMicrosoftDependencyInjectionJobFactory();
    options.UseSimpleTypeLoader();
    options.UseInMemoryStore();
});

services.AddQuartzHostedService(options => options.WaitForJobsToComplete = true);

services
    .AddOpenIddict()
    .AddCore(options =>
    {
        options.UseEntityFrameworkCore().UseDbContext<FunicularDbContext>();
        options.UseQuartz();
    })
    .AddServer(options =>
    {
        options
            .SetAuthorizationEndpointUris("/connect/authorize")
            .SetDeviceEndpointUris("/connect/device")
            .SetIntrospectionEndpointUris("/connect/introspect")
            .SetLogoutEndpointUris("/connect/logout")
            .SetTokenEndpointUris("/connect/token")
            .SetUserinfoEndpointUris("/connect/userinfo")
            .SetVerificationEndpointUris("/connect/verify");

        options.AllowAuthorizationCodeFlow().AllowDeviceCodeFlow().AllowPasswordFlow().AllowRefreshTokenFlow();

        options.RegisterScopes(Scopes.Email, Scopes.Profile, Scopes.Roles);

        if (builder.Environment.IsDevelopment())
            options.AddDevelopmentEncryptionCertificate().AddDevelopmentSigningCertificate();

        options.RequireProofKeyForCodeExchange();

        options
            .UseAspNetCore()
            .EnableStatusCodePagesIntegration()
            .EnableAuthorizationEndpointPassthrough()
            .EnableLogoutEndpointPassthrough()
            .EnableTokenEndpointPassthrough()
            .EnableUserinfoEndpointPassthrough()
            .EnableVerificationEndpointPassthrough();

        options.UseAspNetCore();
    })
    .AddValidation(options =>
    {
        options.UseLocalServer();
        options.UseAspNetCore();
    });

services.AddScoped(typeof(IExecutable<>), typeof(FunicularExecutable<>));

services
    .AddGraphQLServer()
    .RegisterService<IExecutable<Character>>()
    .RegisterService<IExecutable<WeatherForecast>>()
    .RegisterService<AddEntity>()
    .RegisterService<UpdateEntity>()
    .RegisterService<RemoveEntity>()
    .AddQueryType<FunicularQuery>()
    .AddMutationType<FunicularMutation>()
    .AddQueryableCursorPagingProvider(defaultProvider: true)
    .AddProjections()
    .AddFiltering()
    .AddSorting()
    .BindRuntimeType<CharacterId, UuidType>()
    .AddTypeConverter<CharacterId, Guid>(_ => _.Value)
    .AddTypeConverter<Guid, CharacterId>(_ => new(_));

services.AddScoped<AddEntity>();
services.AddScoped<UpdateEntity>();
services.AddScoped<RemoveEntity>();

services.Configure<OpenIddictServerOptions>(configuration.GetSection(nameof(OpenIddictServerOptions)));
services.Configure<OpenIddictServerAspNetCoreOptions>(
    configuration.GetSection(nameof(OpenIddictServerAspNetCoreOptions))
);

if (builder.Environment.IsDevelopment())
    services.AddHostedService<DataSeedWorker>();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseWebAssemblyDebugging();
}
else
{
    app.UseExceptionHandler("/Error");
    app.UseHsts();
}

app.UseHttpsRedirection();

app.UseBlazorFrameworkFiles();
app.UseStaticFiles();

app.UseRouting();

app.UseAuthentication();
app.UseAuthorization();

app.MapGraphQL().RequireAuthorization();
app.MapDefaultControllerRoute();
app.MapFallbackToFile("index.html");

app.Run();