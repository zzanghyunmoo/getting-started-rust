# HelloWorld C# 프로젝트

이 프로젝트는 C# 프로그래밍 언어를 사용하여 콘솔에 "Hello, World!"를 출력하는 간단한 콘솔 애플리케이션입니다. C# 학습의 가장 기본적인 첫 단계입니다.

## 프로젝트 목적

- C# 콘솔 애플리케이션의 기본 구조 이해
- 프로그램의 시작점인 `Main` 메서드의 역할 파악
- `Console.WriteLine`을 사용한 기본 출력 학습

## 코드 (`Program.cs`)

```csharp
namespace HelloWorld;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, World!");
    }
}
```

## 실행 방법

이 프로젝트를 실행하려면 컴퓨터에 .NET SDK가 설치되어 있어야 합니다.

1.  터미널 또는 명령 프롬프트를 엽니다.
2.  이 프로젝트 폴더(`HelloWorld`)로 이동합니다.
3.  다음 명령어를 실행합니다.

    ```bash
    dotnet run
    ```

콘솔에 `Hello, World!`가 출력되는 것을 확인할 수 있습니다.