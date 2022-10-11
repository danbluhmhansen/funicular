namespace Funicular.Server.Validation;

using FluentValidation;

using Funicular.Server.ViewModels.Account;

public class RegisterModelValidator : AbstractValidator<RegisterViewModel>
{
    public RegisterModelValidator()
    {
        RuleFor(_ => _.Email).NotEmpty().EmailAddress();
        RuleFor(_ => _.Password).Length(6, 100).WithMessage("The Password must be at least 6 characters long.");
        RuleFor(_ => _.ConfirmPassword)
            .Must((model, confirmPassword) => model.Password == confirmPassword)
            .WithMessage("The password and confirmation password do not match.");
    }
}