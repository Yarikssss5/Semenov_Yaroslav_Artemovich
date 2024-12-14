using System.Configuration;
using System.Data;
using System.Windows;
using Microsoft.EntityFrameworkCore;

namespace ddb;

/// <summary>
/// Interaction logic for App.xaml
/// </summary>
/// 

public class AppDbContext : DbContext
{
    public DbSet<Person> Persons { get; set; }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        optionsBuilder.UseNpgsql("server=localhost;database=db_app;uid=postgres;pwd=1234;");
    }
};



public partial class App : Application
{
}

