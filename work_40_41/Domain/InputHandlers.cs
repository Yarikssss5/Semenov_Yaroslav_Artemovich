namespace work_40_41.Domain {
    public partial class Domain {
        private static Dictionary<string, object?> input_handler_one(string key) {
            Dictionary<string, object?> result = new Dictionary<string, object?>(){};
            string searchValue = "";
            while (true)
            {
                Console.WriteLine("Введите путь до файла включая имя файла и его расшерение : ");
                string filepath = Console.ReadLine() ?? "";
                if (filepath == "") { Console.WriteLine("Путь к файлу пустой !"); continue; }
                else { result["filepath"] = filepath; break; }
            } while (true) {
                if (!int.TryParse(key, out int num)) break;
                else {
                    result[key] = searchValue;
                    Console.WriteLine($"Введите выбранное {paramsNames[num]} студента : ");
                    searchValue = Console.ReadLine() ?? "";
                    if (searchValue == "") { Console.WriteLine("Введённое значение пустое !"); }
                    else { result[paramsNames[num]] = searchValue; break; }
                    result.Add("param", num);
                }
            }
            return result;
        }
        private static Dictionary<string, Delegate> input_handlers = new Dictionary<string, Delegate>(){
            {"1", input_handler_one},
        };
    }
}