using FluentValidation;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Models.Account;
using Funicular.Server.Validation;

using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

using Quartz;

using static OpenIddict.Abstractions.OpenIddictConstants;

var builder = WebApplication.CreateBuilder(args);

var configuration = builder.Configuration;
var services = builder.Services;

services.AddControllersWithViews();
services.AddRazorPages();

services.AddDbContext<FunicularDbContext>(options =>
{
    options.UseNpgsql(configuration.GetConnectionString("Default"));
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
        options.UseLocalServer();
        options.UseAspNetCore();
    });

services.AddScoped<IValidator<Login>, LoginModelValidator>();
services.AddScoped<IValidator<Register>, RegisterModelValidator>();

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

app.MapRazorPages();
app.MapControllers();
app.MapFallbackToFile("index.html");

app.Run();