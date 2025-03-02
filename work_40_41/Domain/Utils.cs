namespace work_40_41.Domain {
    public partial class Domain
    {
        private static List<Dictionary<string, object?>>? parseReadedLines(List<string> _data)
        {
            try
            {
                List<Dictionary<string, object?>> parsedData = [];
                foreach (string line in _data)
                {
                    string[] filedsData = line.Split(sep);
                    Dictionary<string, object?> serialisedData = new Dictionary<string, object?>();
                    for (int i = 0; i < filedsData.Length; i++) serialisedData.Add(paramsNames[i], filedsData[i]);
                    parsedData.Add(serialisedData);
                }
                return parsedData;
            } catch (Exception e)
            {
                Console.WriteLine(e.ToString());
                return null;
            }
        }

        private static bool validate_dict(Dictionary<string, object?> _params)
        {
            if (_params.ContainsKey("param"))
            {
                if (_params["param"] != null)
                {
                    bool boolRule = false;
                    boolRule = boolRule + (_params["param"] ?? "").ToString() != "";
                    return boolRule;
                }
                else return false;
            }
            return false;
        }

        private string inputTaskNumber()
        {
            string taskNum = "";
            while(taskNum == "")
            {
                try
                {
                    Console.WriteLine("Введите номер задачи:");
                    taskNum = Console.ReadLine() ?? "";
                    if (taskNum == "")
                    {
                        Console.WriteLine("Вы ввели пустую строку или номер задачи отсутствуйщий в перечне");
                        continue;
                    }
                    else
                    {
                        if (!solutions.ContainsKey(taskNum)) { 
                            Console.WriteLine("Для выбранной вами задачи пока что нет решения.");
                            break;
                        }
                    }
                } catch (Exception e) { Console.WriteLine(e.ToString()); continue; }
            } 
            return taskNum;
        }
    }
}
