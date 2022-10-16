using FluentValidation;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph;
using Funicular.Server.Graph.Models;
using Funicular.Server.Services;
using Funicular.Server.Validation;
using Funicular.Server.ViewModels.Account;

using GraphQL;

using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

using Quartz;

using static OpenIddict.Abstractions.OpenIddictConstants;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
var services = builder.Services;

services.AddControllersWithViews();

services.AddDbContext<FunicularDbContext>(options =>
{
    options.UseNpgsql(builder.Configuration.GetConnectionString("Default"));
    options.UseOpenIddict();
});

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
    })
    .AddValidation(options =>
    {
        // options.AddAudiences("resource_server");
        options.UseLocalServer();
        options.UseAspNetCore();
    });

services.AddCors(
    options => options.AddDefaultPolicy(policy => policy.AllowAnyHeader().AllowAnyMethod().AllowAnyOrigin())
);

services.AddScoped<IValidator<LoginViewModel>, LoginModelValidator>();
services.AddScoped<IValidator<RegisterViewModel>, RegisterModelValidator>();

services.AddScoped<OrderByGraphType>();
services.AddScoped<DynamicFieldType>();
services.AddScoped<CharacterType>();
services.AddScoped<FunicularQuery>();
services.AddScoped<FunicularMutation>();

services.AddGraphQL(
    options => options.AddSystemTextJson().AddSchema<FunicularSchema>(GraphQL.DI.ServiceLifetime.Scoped)
);

if (builder.Environment.IsDevelopment())
    services.AddHostedService<DataSeedWorker>();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseGraphQLPlayground();
}

app.UseStaticFiles();

app.UseHttpsRedirection();

app.UseRouting();

app.UseCors();

app.UseAuthentication();
app.UseAuthorization();

app.UseWhen(
    context => context.Request.Path.Equals("/graphql"),
    b =>
        b.Use(
            async (context, next) =>
            {
                var characterType = context.RequestServices.GetRequiredService<CharacterType>();
                var query = context.RequestServices.GetRequiredService<FunicularQuery>();
                var mutation = context.RequestServices.GetRequiredService<FunicularMutation>();
                var db = context.RequestServices.GetRequiredService<FunicularDbContext>();
                await foreach (
                    var field in db.CharacterFields
                        .AsNoTracking()
                        .AsAsyncEnumerable()
                        .WithCancellation(context.RequestAborted)
                )
                {
                    characterType.DynamicField(field);
                    query.AddDynamicFields(field);
                    mutation.AddDynamicFields(field);
                }
                query.InitializeCharacters();
                mutation.InitializeSaveCharacters();
                await next();
            }
        )
);

app.MapControllerRoute("default", "{controller=Home}/{action=Index}/{id?}");

app.Run();