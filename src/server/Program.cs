using FluentValidation;

using Funicular.Server.Data;
using Funicular.Server.Data.Models;
using Funicular.Server.Graph;
using Funicular.Server.Models.Account;
using Funicular.Server.Validation;

using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

using OpenIddict.Server;
using OpenIddict.Server.AspNetCore;

using Quartz;

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
        if (builder.Environment.IsDevelopment())
            options.AddDevelopmentEncryptionCertificate().AddDevelopmentSigningCertificate();
        options.UseAspNetCore();
    })
    .AddValidation(options =>
    {
        options.UseLocalServer();
        options.UseAspNetCore();
    });

services.AddGraphQLServer().AddQueryType<FunicularQuery>();

services.AddScoped<IValidator<Login>, LoginModelValidator>();
services.AddScoped<IValidator<Register>, RegisterModelValidator>();

services.Configure<OpenIddictServerOptions>(configuration.GetSection(nameof(OpenIddictServerOptions)));
services.Configure<OpenIddictServerAspNetCoreOptions>(
    configuration.GetSection(nameof(OpenIddictServerAspNetCoreOptions))
);

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

app.MapGraphQL();
app.MapRazorPages();
app.MapControllers();
app.MapFallbackToFile("index.html");

app.Run();