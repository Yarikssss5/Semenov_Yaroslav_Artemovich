FROM yariksss5/dotnet-8_9_mine:latest AS publisher_build
WORKDIR /app
COPY . .
RUN dotnet publish -c Release -o out

# runtime
FROM yariksss5/dotnet-8_9_mine:latest
WORKDIR /app
COPY --from=publisher_build /app/out .
ENTRYPOINT ["dotnet", "publisher_app.dll"]
