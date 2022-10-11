namespace Funicular.Server.Validation;

using FluentValidation;

using Funicular.Server.ViewModels.Account;

public class LoginModelValidator : AbstractValidator<LoginViewModel>
{
    public LoginModelValidator()
    {
        RuleFor(_ => _.Email).NotEmpty().EmailAddress();
        RuleFor(_ => _.Password).NotEmpty();
    }
}