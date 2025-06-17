using System.Text;

namespace work_54_55 {
    internal class Program {
        private static void numbers_txt()
{

    const string numbers_filename = "numbers.txt";

    // Удаляем старый файл и создаём новый
    if (File.Exists(numbers_filename))
        File.Delete(numbers_filename);

    using (StreamWriter sw = new StreamWriter(numbers_filename))
    {
        for (int i = 1; i < 257; i++)
        {
            if (i != 256)
                sw.Write($"{i},");
            else
                sw.Write($"{i}");
        }
    }
}
        private static void cars_txt() {
            List<string> cars_dev = new List<string>(){
                "Toyota", "Ford", "Chevrolet", "Honda", "Nissan", "BMW",
                "Mercedes-Benz", "Audi", "Volkswagen",  "Hyundai", "Kia"
            };
            const string cars = "cars.txt";
            if (!File.Exists(cars)) File.Create(cars).Close();
            else{
                File.Delete(cars);
                File.Create(cars).Close();
            }
            using(StreamWriter sw = new StreamWriter(cars)) {
                foreach(string car in cars_dev) {
                    sw.WriteLine(car);
                }
            }
        }
        private static void findLongerLine() {
            Console.WriteLine("Введите путь до файла : ");
            string my_file = Console.ReadLine() ?? "";
            if (!File.Exists(my_file)) File.Create(my_file).Close();
            StringBuilder sb = new StringBuilder();
            int high_line = 0;
            using(StreamReader sr = new StreamReader(my_file)) {
                string? line = sr.ReadLine();
                while(line != null) {
                    if (line.Length > high_line) { 
                        high_line = line.Length;
                        sb.Clear();
                        sb.Append(line);
                    }
                    line = sr.ReadLine();
                }
            }
            Console.WriteLine($"Самая длинная строка в файле {my_file} : {sb} , длинна : {high_line}");
        }


        private static void random_numbers_txt() {
            const string randmon_txt = "random.txt";
            if(!File.Exists(randmon_txt)) File.Create(randmon_txt).Close();
            else { File.Delete(randmon_txt); File.Create(randmon_txt).Close(); }
            Console.WriteLine("Введите размер массива : ");
            if (int.TryParse(Console.ReadLine() ?? "", out int num)) {
                List<int> values = new List<int>();
                for (int i = 0; i < num; i++) values.Add(new Random().Next(1, 1000));
                StringBuilder sb = new StringBuilder();
                StringBuilder sb_2 = new StringBuilder();
                for (int i = 0; i < num; i++) {
                    if (i % 2 == 0) sb.Append(" " + values[i].ToString());
                    else sb_2.Append(" " + values[i].ToString());
                }
                Console.WriteLine($"Четные : {sb.ToString()}");
                Console.WriteLine($"Нечетные : {sb_2.ToString()}");
                using(StreamWriter sw = new StreamWriter(randmon_txt)) {
                    sw.WriteLine($"Четные : {sb.ToString()}");
                    sw.WriteLine($"Нечетные : {sb_2.ToString()}");
                }
            }
        }
        private static void general_task() {
            const string out_log = "log.txt";
            if (!File.Exists(out_log)) File.Create(out_log).Close();
            else { File.Delete(out_log); File.Create(out_log).Close(); }
            Console.WriteLine("Введите полный путь до файла лога : ");
            string user_file = Console.ReadLine() ?? "";
            if (user_file != "") {
                if (File.Exists(user_file)) {
                    Console.WriteLine("Введите слово для поиска : ");
                    string search_word = Console.ReadLine() ?? "";
                    List<string> lines = [];
                    int selected_lines = 0;
                    using(StreamReader sr = new StreamReader(user_file)) {
                        string? line = sr.ReadLine();
                        while(line != null) {
                            if (line.Contains(search_word)) {lines.Add(line); selected_lines++;}
                            line = sr.ReadLine();
                        }
                    }
                    Console.WriteLine($"Было найденно : {selected_lines} строк");
                    using(StreamWriter sw = new StreamWriter(out_log)) {
                        foreach(string i in lines) sw.WriteLine(i);
                    }
                }
            }
        }
        private static void Main() {
            // cars_txt();
            numbers_txt();
            // findLongerLine();
            // random_numbers_txt();
            // general_task();
        }
    }
}
