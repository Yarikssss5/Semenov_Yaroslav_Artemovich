namespace work_40_41.FileManager {
    class FileWriter {
        private string filepath;
        public FileWriter(string? _filepath) {
            string check = _filepath ?? throw new Exception("Путь к файлу пустой !");
            if (File.Exists(_filepath)) {
                filepath = _filepath;
            } else filepath = "";
        }
        public string? WriteLines(string? _filepath, string[] _data) {
            try {
                filepath = _filepath ?? "";
                if (filepath != "") {
                    if(!File.Exists(filepath)) File.Create(filepath);
                    using(StreamWriter sw = new StreamWriter(filepath)) {
                        foreach(string line in _data) {
                            sw.WriteLine(line);
                        }
                    }
                    return null;
                } else {
                    string msg = "Путь к файлу пустой !";
                    Console.WriteLine(msg);
                    return msg;
                }
            } catch (Exception e) {
                Console.WriteLine(e);
                return e.ToString();
            }
        }
    }
}