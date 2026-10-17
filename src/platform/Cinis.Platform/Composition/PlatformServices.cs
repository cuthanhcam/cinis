using Cinis.Access.Features.AuthorizeAccount;
using Cinis.Access.Infrastructure.Persistence;
using Cinis.Access.Ports;
using Cinis.ExecutionClient;
using Cinis.Platform.Adapters;
using Cinis.Trading.Features.CancelOrder;
using Cinis.Trading.Ports;

namespace Cinis.Platform.Composition;

internal static class PlatformServices
{
    public static IServiceCollection AddPlatformModules(this IServiceCollection services)
    {
        // No implicit demo privilege, HTTP authentication or execution connection.
        services.AddSingleton<IAccountGrantStore>(new InMemoryAccountGrantStore([]));
        services.AddSingleton<AccountAccessService>();
        services.AddSingleton<IAccountAuthorizer, AccessAuthorizationAdapter>();
        services.AddSingleton<IExecutionGateway, DisabledExecutionGateway>();
        services.AddTransient<CancelOrderHandler>();
        return services;
    }
}
