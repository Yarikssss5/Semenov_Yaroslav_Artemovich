using System.Text;
using System.Text.Json;
using RabbitMQ.Client;
using RabbitMQ.Client.Events;

class Lesson
{
    public int Count {get; set; }
    public string Office {get; set;}
    public string Teacher {get; set;}
    public string Subject {get; set;}
    public string Time {get; set;}
    public Lesson(int count, string office, string teacher, string subject, string time)
    {
        Count = count;
        Office = office;
        Teacher = teacher;
        Subject = subject;
        Time = time;
    }
}

class Consumer
{
    static void Main()
    {
        // Читаем параметры подключения из переменных окружения
        var host = Environment.GetEnvironmentVariable("RABBITMQ_HOST") ?? "localhost";
        var user = Environment.GetEnvironmentVariable("RABBITMQ_USER") ?? "guest";
        var pass = Environment.GetEnvironmentVariable("RABBITMQ_PASSWORD") ?? "guest";
        // Создаем подключение к RabbitMQ
        var factory = new ConnectionFactory()
        {
            HostName = host, // Адрес сервера RabbitMQ
            UserName = user,
            Password = pass
        };
        using (var connection = factory.CreateConnection())
        using (var channel = connection.CreateModel())
        {
            channel.QueueDeclare("lessons", false, false, false, null);
            var consumer = new EventingBasicConsumer(channel);
            consumer.Received += (model, ea) =>
            {
                var body = ea.Body.ToArray();
                var json = Encoding.UTF8.GetString(body);
                var lesson = JsonSerializer.Deserialize<Lesson>(json);
                if (lesson == null) { }
                else {
                    Console.WriteLine($"[x] Получено: {lesson.Subject} | {lesson.Office} | {lesson.Teacher} | {lesson.Time}");
                }
            };
            channel.BasicConsume("lessons", true, consumer);
            Console.WriteLine("Ожидаем пары...");
            while (true)
            {
                Thread.Sleep(100);
            }
        }
    }
} 
