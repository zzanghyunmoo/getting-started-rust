# SimpleOOP

이 저장소는 C#의 기본 개념(프로그램 시작점, 주석, 클래스, 메서드)을 간단한 예제로 설명하기 위해 만들어졌습니다. 학생들이 `SimpleOOP/Program.cs` 파일의 주석과 코드를 따라하며 개념을 이해할 수 있도록 구성되어 있습니다.

## 학습 목표

- `Main`이 프로그램의 시작점이라는 것을 이해한다.
- 주석(comment)의 역할과 사용법을 배운다.
- 클래스(class)가 무엇이며 어떻게 인스턴스를 만드는지 이해한다.
- 메서드(method)가 무엇인지, 코드 재사용에 어떻게 도움이 되는지 배운다.

## 핵심 개념 요약

1. `Main`
   - C# 프로그램은 `Main` 메서드에서 실행이 시작됩니다.
   - 매개변수 `string[] args`는 명령줄 인수를 받습니다.

2. 주석 (Comments)
   - `//`로 시작하는 한 줄 주석과 `/* ... */`의 여러 줄 주석이 있습니다.
   - 주석은 코드 실행에 영향을 주지 않고, 코드의 의도를 설명하는 데 사용됩니다.

3. 클래스 (Class)
   - 클래스는 데이터(필드)와 동작(메서드)을 묶는 설계도입니다.
   - 예: `GiftBox` 클래스는 `Letter`(문자열)와 `Money`(정수)라는 필드를 가집니다.
   - 클래스로부터 객체(instance)를 생성하면 실제 데이터를 담는 인스턴스가 만들어집니다.

4. 메서드 (Method)
   - 메서드는 특정 작업을 수행하는 코드 블록입니다.
   - 예: `GiftBoxMaker` 메서드는 새 `GiftBox` 객체를 생성하여 반환합니다.
   - 메서드를 사용하면 중복 코드를 줄이고, 코드를 구조화할 수 있습니다.

## 예제(핵심 발췌)

다음 코드는 `SimpleOOP/Program.cs`에서 발췌한 예제입니다.

클래스 정의
```csharp
class GiftBox
{
    public string Letter;
    public int Money;
}
```

클래스 인스턴스 생성 및 사용
```csharp
GiftBox box = new GiftBox();
box.Letter = "Congratulations!";
box.Money = 100000;
Console.WriteLine($"Letter: {box.Letter}, Money: {box.Money}");
```

메서드 정의
```csharp
static GiftBox GiftBoxMaker(string letter, int money)
{
    GiftBox box = new GiftBox();
    box.Letter = letter;
    box.Money = money;
    return box;
}
```

메서드 사용
```csharp
static void Main(string[] args)
{
    GiftBox myBox = GiftBoxMaker("Happy Birthday!", 50000);
    Console.WriteLine($"Letter: {myBox.Letter}, Money: {myBox.Money}");
}
```
        

## 실행 방법

1. Visual Studio 2026에서 솔루션을 연다.
2. `SimpleOOP` 프로젝트를 시작 프로젝트로 설정한 뒤 실행(__F5__)한다.

## 연습 과제

- `GiftBox`에 새 필드(`Sender`)를 추가하고 출력문을 수정해 보세요.
- `GiftBoxMaker`를 오버로드하여 기본값을 사용하도록 만들어 보세요.
- 주석을 읽고 각 코드 블록이 무엇을 하는지 직접 설명해 보세요.

---

필요하면 README를 더 확장해 단계별 실습, 퀴즈, 참고 자료(공식 문서 링크 등)를 추가해 드리겠습니다.