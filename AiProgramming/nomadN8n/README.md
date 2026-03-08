# n8n 마스터클래스: 워크플로우 자동화 정복하기

이 저장소는 [Nomad Coders](https://nomadcoders.co/)의 n8n 마스터클래스 강의를 듣고 학습한 내용을 정리하는 공간입니다. n8n을 사용하여 다양한 워크플로우를 자동화하고 생산성을 높이는 방법을 기록합니다.

---

## 학습 목차

### 1. n8n 소개 및 시작하기
- **n8n 이란?**: n8n의 개념과 장점 소개
- **설치 및 설정**: 로컬 환경에 n8n 설정하기 (Docker, npm)
- **UI 둘러보기**: n8n 에디터의 주요 기능 및 인터페이스 소개

### 2. 핵심 개념 (Core Concepts)
- **Workflows**: 워크플로우의 개념과 생명주기
- **Nodes**: 노드의 종류 (Trigger, Regular) 와 역할
- **Connections (Credentials)**: 외부 서비스 연동을 위한 인증 정보 관리
- **Executions**: 워크플로우 실행 및 로그 확인 방법
- **Data Flow & Expressions**: 노드 간 데이터 전달 방식 및 JavaScript 표현식 활용법

### 3. 주요 노드 및 통합 (Nodes & Integrations)
- **기본 노드**:
    - `Start`: 워크플로우의 시작점
    - `Manual Trigger`: 수동으로 워크플로우 실행
    - `Cron`: 정해진 시간에 자동으로 실행
    - `Set`: 데이터 재구성 및 새로운 값 설정
    - `IF`: 조건에 따른 분기 처리
    - `Switch`: 여러 조건에 따른 분기 처리
    - `Merge`: 여러 데이터 스트림 병합
- **외부 서비스 연동**:
    - `HTTP Request`: REST API 호출
    - `Google Sheets`: 구글 시트 데이터 읽기 및 쓰기
    - `Discord / Slack`: 메시지 및 알림 전송
    - `OpenAI`: ChatGPT 등 AI 모델 연동

### 4. 워크플로우 예제 (Workflow Examples)
- **자동화 예제 1**: 매일 아침 오늘의 날씨 정보 슬랙으로 받기
- **자동화 예제 2**: 구글 시트에 새로운 행 추가 시, 데이터베이스에 자동 저장하기
- **자동화 예제 3**: 특정 키워드 관련 뉴스 기사 수집 후, Discord 채널에 공유하기

### 5. 고급 주제 (Advanced Topics)
- **Error Handling**: 워크플로우 실행 중 발생하는 오류 처리 방법
- **Sub-Workflows (Call Workflow)**: 다른 워크플로우 호출 및 재사용
- **Code Node**: JavaScript를 사용하여 복잡한 로직 구현
- **Custom Credentials**: 지원하지 않는 API를 위한 직접 인증 구현
- **Self-Hosting Best Practices**: 자체 서버 운영 시 고려사항 및 최적화

---

> 이 문서는 강의를 들으며 지속적으로 업데이트될 예정입니다.