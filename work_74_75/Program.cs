using System;
using System.Collections.Generic;
using System.Xml;

namespace WeatherApp {
    public class WeatherServiceXML {
        public string GetWeatherData() {
            return @"
                <weather>
                    <city>London</city>
                    <temperature>12</temperature>
                    <humidity>70</humidity>
                    <condition>Cloudy</condition>
                </weather>";
        }
    }
    public class WeatherSystemJSON {
        public void DisplayWeather(string jsonData) {
            Console.WriteLine("Полученные данные о погоде:");
            Console.WriteLine(jsonData);
        }
    }
    public interface IDataAdapter { string Convert(); }
    public class XmlToJsonAdapter : IDataAdapter {
        private readonly string _xmlData;
        public XmlToJsonAdapter(string xmlData) {  _xmlData = xmlData; }
        public string Convert() {
            var doc = new XmlDocument();
            doc.LoadXml(_xmlData);
            var result = new Dictionary<string, string>();
            foreach (XmlNode node in doc.DocumentElement.ChildNodes)  result[node.Name] = node.InnerText;
            return SimpleJson.Serialize(result);
        }
    }
    public static class SimpleJson {
        public static string Serialize(Dictionary<string, string> data) {
            var entries = new List<string>();
            foreach (var kvp in data) entries.Add($"\"{kvp.Key}\": \"{kvp.Value}\"");
            return "{" + string.Join(", ", entries) + "}";
        }
    }
    internal class Program {
        static void Main(string[] args)  {
            var xmlService = new WeatherServiceXML();
            string xmlData = xmlService.GetWeatherData();
            IDataAdapter adapter = new XmlToJsonAdapter(xmlData);
            string jsonData = adapter.Convert();
            var weatherSystem = new WeatherSystemJSON();
            weatherSystem.DisplayWeather(jsonData);
        }
    }
}