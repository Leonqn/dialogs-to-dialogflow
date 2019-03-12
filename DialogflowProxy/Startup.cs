using System.IO;
using Google.Cloud.Dialogflow.V2;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.Logging;
using Newtonsoft.Json;
using Microsoft.AspNetCore.Http;
using System;
using Newtonsoft.Json.Serialization;

namespace DialogflowProxy
{
    public class DialogflowRequest
    {
        public string ProjectId { get; set; }
        public string SessionId { get; set; }
        public string Text { get; set; }
        public string LanguageCode { get; set; }
    }

    public class Startup
    {
        public void Configure(IApplicationBuilder app, ILogger<Startup> logger)
        {
            var client = SessionsClient.Create();

            var jsonSettings = new JsonSerializerSettings
            {
                ContractResolver = new DefaultContractResolver
                {
                    NamingStrategy = new CamelCaseNamingStrategy()
                },
            };
            app.Map("/api/dialogflow/detectIntent", builder =>
            {
                builder.Run(async ctx =>
                {
                    try
                    {
                        var body = await new StreamReader(ctx.Request.Body).ReadToEndAsync();
                        var request = JsonConvert.DeserializeObject<DialogflowRequest>(body);

                        var response = await client.DetectIntentAsync(
                            session: new SessionName(request.ProjectId, request.SessionId),
                            queryInput: new QueryInput()
                            {
                                Text = new TextInput()
                                {
                                    Text = request.Text,
                                    LanguageCode = request.LanguageCode
                                }
                            }
                        );
                        await ctx.Response.WriteAsync(JsonConvert.SerializeObject(response, jsonSettings));
                    }
                    catch (Exception e)
                    {
                        logger.LogError(e, "error occured");
                        ctx.Response.StatusCode = StatusCodes.Status500InternalServerError;
                        await ctx.Response.WriteAsync(e.Message);
                    }
                });
            });
        }
    }
}
