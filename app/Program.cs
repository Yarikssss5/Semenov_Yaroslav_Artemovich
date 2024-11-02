using System;
using System.Drawing;

namespace app {
    public enum SupportyeFunctions {

    }

    public class MyPoint {
        private int x;
        private int y;

        public int X() => this.x;
        public int Y() => this.y;

        public void X(int _x) { this.x = _x; }
        public void Y(int _y) {this.y = _y;} 
    }

    public struct Box<T> {
        private T value;

        public Box(T _value) { this.value = _value; }

        public T Value() => this.value;
        public T Value(T _value) { return this.value = _value; }
    }

    public class Program {
        public ref struct Node {
            private int next;
            private int current;
            private double d_buf;
            private MyPoint? mypoint;

            // Constructor :
            public Node(int _next, int _current, double _dbuf = 0, MyPoint? _point = null) {
                this.next = _next;
                this.current = _current;
                this.d_buf = _dbuf;
                this.mypoint = _point;
            }

            // Setters :
            public void Next(int _next) { this.next = _next; }
            public void Current(int _current) { this.current = _current; }
            public void Dbuf(int _dbuf) {this.d_buf = _dbuf;}
            public void Point(MyPoint? _point) {this.mypoint = _point;}

            // Getters :
            public int Next() => this.next; 
            public int Current() => this.current;
            public double Dbuf() => this.d_buf;
            public MyPoint? Point() => this.mypoint;
            
        }
        static void Main() {
            // Node a = new Node(12, 6);

            // Console.WriteLine(a.Next());
            // a.Next(13);
            // Console.WriteLine(a.Next());

            // Console.WriteLine("Hello !");

            Box<int> a = new Box<int>(12);
            Console.WriteLine(a.Value());
            a.Value(100);
            Console.WriteLine(a.Value());

        }
    }
}