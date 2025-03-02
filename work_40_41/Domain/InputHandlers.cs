namespace work40_41.Domain {
    public partial class Domain {
        private static Dictionary<string, object?> input_handler_one(string key) {
            Dictionary<string, object?> result = new Dictionary<string, object?>(){};
            Console.WriteLine("Введите выбранное имя студента : ");
            string searchValue = Console.ReadLine() ?? "";
            result[key] = searchValue;
            return result;
        }
        private static Dictionary<string, object?> input_handler_two() {
            Dictionary<string, object?> result = new Dictionary<string, object?>(){};
            return result;
        }

        private static Dictionary<string, Delegate> input_handlers = new Dictionary<string, Delegate>(){
            {"1", input_handler_one},
            {"2", input_handler_two}
        };
    }
}