namespace Funicular.Server.Validation;

using FluentValidation;

using Funicular.Server.Models.Account;

public class LoginModelValidator : AbstractValidator<Login>
{
    public LoginModelValidator()
    {
        RuleFor(_ => _.Email).NotEmpty().EmailAddress();
        RuleFor(_ => _.Password).NotEmpty();
    }
}