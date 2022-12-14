namespace Funicular.Server.Attributes;

using Microsoft.AspNetCore.Mvc.Abstractions;
using Microsoft.AspNetCore.Mvc.ActionConstraints;

public sealed class FormValueRequiredAttribute : ActionMethodSelectorAttribute
{
    private readonly string name;

    public FormValueRequiredAttribute(string name)
    {
        this.name = name;
    }

    public override bool IsValidForRequest(RouteContext routeContext, ActionDescriptor action) =>
        !string.Equals(routeContext.HttpContext.Request.Method, "GET", StringComparison.OrdinalIgnoreCase)
        && !string.Equals(routeContext.HttpContext.Request.Method, "HEAD", StringComparison.OrdinalIgnoreCase)
        && !string.Equals(routeContext.HttpContext.Request.Method, "DELETE", StringComparison.OrdinalIgnoreCase)
        && !string.Equals(routeContext.HttpContext.Request.Method, "TRACE", StringComparison.OrdinalIgnoreCase)
        && !string.IsNullOrEmpty(routeContext.HttpContext.Request.ContentType)
        && routeContext.HttpContext.Request.ContentType.StartsWith(
            "application/x-www-form-urlencoded",
            StringComparison.OrdinalIgnoreCase
        )
        && !string.IsNullOrEmpty(routeContext.HttpContext.Request.Form[name]);
}