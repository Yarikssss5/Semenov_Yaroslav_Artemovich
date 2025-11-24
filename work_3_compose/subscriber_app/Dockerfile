FROM yariksss5/dotnet-8_9_mine:latest AS subscriber_build
WORKDIR /app
COPY . .
RUN dotnet publish -c Release -o out

# runtime
FROM yariksss5/dotnet-8_9_mine:latest
WORKDIR /app
COPY --from=subscriber_build /app/out .
ENTRYPOINT ["dotnet", "subscriber_app.dll"]
