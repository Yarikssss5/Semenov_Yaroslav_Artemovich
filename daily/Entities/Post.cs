namespace daily.Entities {
    public class Post(string _text = "", int _id = 0)
    {
        private string text = _text;
        private readonly int id = _id;
        public string Text() => text;
        public int Id() => id;
    }
}