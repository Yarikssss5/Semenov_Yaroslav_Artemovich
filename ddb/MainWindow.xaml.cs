using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata;
using System.Linq;
using Microsoft.EntityFrameworkCore.Metadata.Internal;
using Newtonsoft.Json;

namespace ddb;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
/// 

public class Person
{
    public int Id { get; set; }
    public string? Name { get; set; }
    public string? Surname { get; set; }

    public Dictionary<string, object> ToDictirnary(){
        Dictionary<string, object> out_data = new Dictionary<string, object>() { };
        out_data.Add(key: "Id", value: Id);
        out_data.Add(key: "Name", value: Name != null ? Name : "0".ToString());
        out_data.Add(key: "Surname", value: Surname != null ? Surname : "0".ToString());
        return out_data;
    }

    public void SetByKey(string key, object? value)
    {
        if (value != null)
        {
            switch (key)
            {
                case "Id":
                    if (int.TryParse(value.ToString(), out int id))
                        Id = id;
                    break;
                case "Name":
                    Name = value.ToString();
                    break;
                case "Surname":
                    Surname = value.ToString();
                    break;
            }
        }
    }
};



public partial class MainWindow : Window
{
    public AppDbContext? DBContext { get; set; }
    public MainWindow()
    {
        InitializeComponent();
    }

    private async void MainWindow_Loaded(object sender, RoutedEventArgs e)
    {
        DBContext = new AppDbContext();
        List<Person> peoples = await DBContext.Persons.ToListAsync();
        var entityType = DBContext.Model.GetEntityTypes().First(t => t.GetTableName() == "Persons");
        var columnNames = entityType.GetProperties().Select(p => p.Name);
        foreach (var columnName in columnNames)
        {
            //if (columnName != "Id")
            //{
            //    UserViewAccess.Columns.Add(new DataGridTextColumn() { Header = columnName, Binding = new Binding(columnName) });
            //}
            UserViewAccess.Columns.Add(new DataGridTextColumn() { Header = columnName, Binding = new Binding(columnName) });
        }
        UserViewAccess.ItemsSource = peoples;
    }

    private void UserViewAccess_DoubleClick(object sender, MouseButtonEventArgs e)
    {
        Person? selectedItem = UserViewAccess.SelectedItem as Person;
        if (selectedItem != null)
        {
            // Делаем что-то с выбранной строкой :
            Dictionary<string, object> validated = selectedItem.ToDictirnary();
            string buf = "";

            foreach (var item in validated) buf += " " + item.ToString();

            MessageBox.Show(buf);

            ModifiWindow modifiWindow = new ModifiWindow() { Current_person = validated, Current_tittle = "Person", AppDbContext = DBContext };
            modifiWindow.Show();
        }
    }
}