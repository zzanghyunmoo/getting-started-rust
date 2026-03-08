namespace SimpleOOP;

class GiftBox
{
    public string Letter;
    public int Money;
}
class Program
{
    // 1. main이란 프로그램의 시작점!
    static void Main(string[] args)
    {
        // 2. 클래스 생성
        GiftBox a = new GiftBox()
        {
            Letter = "Happy Birthday!",
            Money = 10000
        };

        // 3. 주석이란 개발자가 코드를 설명하기 위해 다는 글
        // C#에서는 // 로 시작하는 한 줄 주석과 /* */로 감싸는 여러 줄 주석이 있다.
        Console.WriteLine(a.Letter + " " + a.Money);

        // 2-1. b 변수에 a의 값 그러니까 GiftBox 객체의 주소가 b에 복사된다.
        // 따라서 a나 b 둘 중 하나의 값을 바꾸면 같은 객체를 가리키고 있기 때문에 서로 영향을 미친다.
        GiftBox b = a;
        a.Letter = "Merry Christmas!";
        a.Money = 5000;
        Console.WriteLine(b.Letter + " " + b.Money);

        // 2-2. c와 d는 각각 다른 객체를 가리킨다.
        GiftBox c = new GiftBox()
        {
            Letter = "Happy New Year!",
            Money = 1000
        };

        // 4-1. GiftBoxMaker 함수를 호출하여 새로운 GiftBox 객체를 생성
        GiftBox d = Program.GiftBoxMaker(letter: "", money: 12346);
    }
    // 4. 메소드란 특정 작업을 수행하는 코드의 집합
    // 메소드는 함수라고 부르기도 한다
    // 코드를 정리하고 반복을 단순화한다. 즉, 코드의 재사용성을 높이기 위해 사용된다.
    // 함수의 형태는 다음과 같다.
    // 반환형 함수이름(매개변수1, 매개변수2, ...) { 실행코드; return 반환값; }
    private static GiftBox GiftBoxMaker(String letter, int money)
    {
        return new GiftBox()
        {
            Letter = letter,
            Money = money
        };
    }
}
