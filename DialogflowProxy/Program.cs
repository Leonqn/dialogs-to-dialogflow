using System;
using Google.Apis.Auth.OAuth2;
using Google.Cloud.Dialogflow.V2;
using Grpc.Core;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Logging;

namespace DialogflowProxy
{

    class Program
    {
        static void Main(string[] args)
        {

            var port =
                args.Length == 1
                    ? int.Parse(args[0])
                    : 80;
            new WebHostBuilder()
                .UseKestrel(opt =>
                {
                    opt.Limits.MaxRequestBodySize = null;
                })
                .ConfigureLogging((_, config) =>
                {
                    config.AddConsole();
                    config.SetMinimumLevel(LogLevel.Information);
                })
                .UseUrls($"http://*:{port}/")
                .UseStartup<Startup>()
                .Build()
                .Run();


        }
    }
}
