namespace work40_41.FileManager {
    public class FileManager {
        private string? filepath = null;
        public FileManager() {}
        public string[]? Read(string? _filepath) {
            FileReader fr = new FileReader(_filepath);
            string[]? readed_data = fr.ReadLines();
            return readed_data;
        }
        public void Write(string? _filepath, string[] _data) {
            FileWriter fw = new FileWriter(_filepath);
            fw.WriteLines(_filepath, _data);
        }
    }
}