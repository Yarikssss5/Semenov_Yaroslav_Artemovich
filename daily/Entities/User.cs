namespace daily.Entities {
    public class User(int _id = 0, string _name = "")
    {
        private readonly int id = _id;
        private string name = _name;

        public int Id() => id;

        public string Name() => name;
        public void Name(string _name) { name = _name; }
    }
}