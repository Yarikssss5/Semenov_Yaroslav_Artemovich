using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Shapes;
using Microsoft.EntityFrameworkCore;

namespace ddb
{
    public partial class ModifiWindow : Window
    {
        public Dictionary<string, object>? Current_person { get; set; } 
        public string? Current_tittle { get; set; }
        private List<string> fields = new List<string>() { };
        public AppDbContext? AppDbContext { get; set; }
        public ModifiWindow()
        {
            InitializeComponent();
            if (Current_tittle != null) EditFormTittle.Content = Current_tittle.ToString();
            else EditFormTittle.Content = " ";
            EditFormTittle.UpdateLayout();
        }

        public void print_person() {
            if (Current_person != null)
            {
                string buf = "";
                foreach (string key in Current_person.Keys.ToList()) buf += " " + Current_person[key];
                MessageBox.Show(buf);
            }
        }

        private void ModifiWindow_Loaded(object sender, RoutedEventArgs e)
        {
            try {
                if (Current_person != null)
                {
                    foreach (string key in Current_person.Keys.ToList())
                    {
                        
                        fields.Add(key.ToString());
                        // Добавляем Описание поля :
                        Label label = new Label()
                        {
                            Name = (key + "label").ToString(),
                            Content = key.ToString(),
                            FontFamily = new FontFamily("Times News Roman"),
                            Height = 24,
                            Width = 100,
                            FontSize = 14,
                            HorizontalAlignment = HorizontalAlignment.Center,
                            VerticalAlignment = VerticalAlignment.Top,
                            Padding = new Thickness(0),
                        };
                        if (key == "Id") label.Visibility = Visibility.Collapsed;
                        MainFrame_access.Children.Add(label);
                        TextBox textbox = new TextBox()
                        {
                            Name = key.ToString(),
                            Text = Current_person[key].ToString(),
                            FontFamily = new FontFamily("Times New Roman"), Height = 24,
                            Width = 100, FontSize = 14,
                            HorizontalAlignment = HorizontalAlignment.Center,
                            VerticalAlignment = VerticalAlignment.Top,
                            Padding = new Thickness(0)
                        };
                        textbox.TextChanged += (sender, e) =>
                        {
                            var unwrapped = sender as TextBox;
                            if (unwrapped != null)
                                {
                                    if (key != "Id") Current_person[key] = unwrapped.Text.ToString();
                                    else Current_person[key] = Current_person[key];
                                }
                            };
                        if (key == "Id") textbox.Visibility = Visibility.Collapsed;
                        MainFrame_access.Children.Add(textbox);
                    }
                    MainFrame_access.UpdateLayout();
                    ChangeWindowAccess.UpdateLayout();
                }
            }
            catch (Exception err) {
                MessageBox.Show(err.ToString());
            }
        }

        private void SaveToDB_CLick(object sender, RoutedEventArgs e)
        {
            Person person = new Person();
            object? elem = null;
            dynamic? unwrapped = null;
            if (Current_person != null)
            {
                foreach (string key in fields) person.SetByKey(key, Current_person[key]);
                MessageBox.Show($" {person.Name} {person.Surname}  - Person ID {person.Id}");
                if (person.Name != null & person.Surname != null)
                {
                        if (AppDbContext != null)
                        {
                        
                            var existingRecord = AppDbContext.Persons.Find(person.Id);
                            if (existingRecord != null)
                            {
                                existingRecord.Name = person.Name;
                                existingRecord.Surname = person.Surname;
                                AppDbContext.Entry(existingRecord).State = EntityState.Modified;
                                AppDbContext.SaveChanges();
                            }
                            else MessageBox.Show(person.Name);
                        }
                    }
                else MessageBox.Show($" {person.Name} {person.Name}  {person.Surname}");
            }
        }
    }
}
