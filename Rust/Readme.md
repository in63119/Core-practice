# Rust를 공부해보자!

## 1. Rust 개발 셋팅

### 1-1. 설치

Rust 사이트(https://www.rust-lang.org/tools/install)에서 CLI로 설치가 가능하다.(MacOS)

- 명령어 : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 1-2. Rust의 텍스트 편집기?

Rust를 사용할 때 특정한 텍스트 편집기를 필수적으로 사용해야 하는 것은 아니다. 다만 Rust 개발자들은 `VS Code`와 `IntelliJ IDEA`를 사용한다고 한다. 그 외로는 `Sublime Text`, `Vim/Neovim`(이건 터미널에서 급하기 사용할 때 쓰지 않을까??..), `Emacs`를 사용한다고 한다.

- 나의 선택?? 당연히 `VS Code`.
- (Optional) Rust를 설치 후 `VS Code`에 rust Extension을 설치 해주었다.

### 1-3. 프로젝트 셋팅(a.k.a 패키지 매니저)

Node 환경에서는 프로젝트 첫 셋팅을 할 때, `npm(or yarn) init -y`를 이용한다. 그러면 `package.json(or yarn.json)`이 생기면서 이곳에 사용하고 싶은 모듈을 설치하여 의존성, 메타데이터, 기타 설정을 정의할 수 있다.
<br />
Rust 역시 `Cargo`(이름 멋있다..)라는 공식 패키지 매니저와 빌드 시스템이 있다. 검색 해본 결과 npm과 비슷한 역할을 하는 것 같다.

- Cargo.toml과 Cargo.lock: Cargo.toml 파일은 Node.js의 package.json과 유사하며, 프로젝트의 의존성, 메타데이터, 기타 설정을 정의한다. Cargo.lock 파일은 프로젝트의 의존성 트리와 각 의존성의 정확한 버전을 기록하여 일관된 빌드를 보장한다.

- 의존성 관리: Cargo는 crates.io라는 중앙 집중식 패키지 레지스트리에서 패키지(크레이트라고 함)를 가져온다. Cargo.toml 파일에 정의된 의존성에 따라 필요한 크레이트를 자동으로 다운로드하고, 빌드한다.

- 프로젝트 빌드: Cargo는 Rust 프로젝트를 쉽게 빌드할 수 있는 명령어를 제공한다. cargo build 명령을 사용하여 프로젝트를 컴파일할 수 있다.

- 테스트 및 문서: cargo test 명령으로 프로젝트의 테스트를 실행할 수 있고, cargo doc 명령으로 코드 문서를 생성할 수 있다.

- 패키지 배포: 개발한 크레이트를 crates.io에 배포할 수 있어, 다른 사람들이 사용할 수 있게 된다.

<br />
<br />

rust 프로젝트를 생성을 하려면, `cargo new my_project`.
<br />

현재 내가 있는 디렉토리에 프로젝트를 초기화 하려면 `cargo init`. (나는 그냥 초기화만 했다.)

<br />

**`cargo init` 후에 생기는 것들**

- src 폴더: 여타 프레임워크의 src 폴더와 다른 것이 없다.(여기서 작업)
- target 폴더: Node의 경우. 내가 사용하고 싶은 모듈을 설치하면 package.json에 의존성이 저장되면서, `node_modules`에 설치된다. 일반적으로 `node_modules`는 깃 레포에 굳이 올리지 않는다. rust도 같은 것이 있을까 검색해보니 역시나 이 `target` 폴더가 그 역할을 한다고 한다. `.gitignore`로 커밋 제외 시켜주었다.
- `Cargo.toml & Cargo.lock` : `package.json(or yarn.json) & package.lock(or yarn.lock)`이라고 보면 된다.

<br />

### 1-4. 크레이트 추가(a.k.a 라이브러리)

NodeJS에서는 라이브러리를 추가하려면 `npm(or yarn) install(or add) {라이브러리 명} @1.0.0(버전 명시 안하면 최신버전)` 같은 형식으로 명령을 해야한다.
<br />

명령을 하게되면 `package.json(or yarn.json)`에 명시되면서 `node_modules`에 대한 라이브러리가 추가된다.
<br />
<br />

Rust에서는 외부 라이브러리를 '크레이트'라고 해서 화물칸에 싣는 큰 상자를 의미하는 이름을 사용한다고 한다.
<br />

Rust 프로젝트에서 외부 라이브러리(크레이트)를 추가하는 과정은 NodeJS의 `npm(or yarn) install(or add) {라이브러리 명} @1.0.0(버전 명시 안하면 최신버전)`과는 다르게 진행된다. 명령어를 사용해 설치하지 않고, `Cargo.toml`의 파일을 수정하는 방식으로 진행한다.
<br />

예를 들어, `sha2`라는 크레이트를 사용하기 위해서

- `Cargo.toml` 수정

  ```
   # 기존 Cargo.toml
   ...

   [dependencies]

   # 변경된 Cargo.toml
   ...

   [dependencies]
   sha2 = "*" # 버전을 명시하고 싶으면, sha2 = "0.9.8"

  ```

- `$ cargo build` 프로젝트 빌드
  터미널에서 `cargo build` 명령어를 통해 `Cargo.toml`에 지정된 의존성을 자동으로 다운로드하고, 프로젝트를 빌드한다.
- 라이브러리 사용
  이제 프로젝트 내의 Rust 파일에서 use 키워드를 사용해 크레이트의 기능을 가져와 사용할 수 있다.
  <br />
  (사용 예시는 다음 포스트로)

<br />
<br />

## 2. 문법

### 2-1. main 함수

`cargo init`을 실행시키면 `src`폴더에 `main.rs` 파일이 생기면서 안에는 main 함수가 있다.
<br />

`main`함수에는 "Hello, world!"를 화면에 출력하는 명령이 포함되어 있다.

```rust
fn main() {
    println!("Hello, world!");
}
```

한번 rust를 실행시켜보고 싶어서 `cargo run`을 해봤는데, 터미널에 "Hello world!" 가 출력되었다.
<br />

여기서 궁금한 것!
<br />

**분명히 함수 `main`을 실행시키는 코드가 없는데, 왜 함수 실행이 된걸까?**
<br />

: Rust 프로그램에서 `main`함수는 특별한 역할을 한다. Rust 언어의 규칙에 따라, **모든 실행 가능한 Rust 프로그램은 `main`함수로 시작한다.** 따라서 이 함수는 프로그램의 진입점(entry point)으로 작동하며, 프로그램이 실행될 때 가장 먼저 호출되는 것이다.
<br />

즉, Rust에서는 다른 언어들처럼 별도로 프로그램의 시작점을 지정할 필요가 없다. `main` 함수가 그 역할을 자동으로 수행하기 떄문이다.

<br />

### 2-2. 크레이트(외부 라이브러리) 가져오기

지난 번 크레이트를 추가하는 공부를 하면서 `sha2`라는 크레이트를 설치했다.

> [1-4. 크레이트 추가(a.k.a 라이브러리)](#1-4-크레이트-추가aka-라이브러리)

이번에는 이 `sha2`크레이트를 내 코드에 불러오는 것을 해보자.
<br />
크레이트를 불러오는 코드는 다음과 같다.

```rust
use sha2::{Sha256, Digest}; // sha2 불러오기

fn main() {
    println!("Hello, world!"); // sha2를 사용하는 부분은 다음 포스트에서...
}
```

Rust에서 모듈, 크레이트, 함수, 타입 등을 현재의 스코프로 가져올 때는 `use` 키워드를 사용한다.
<br />

위의 코드에서는 `sha2` 크레이트의 `Sha256`, `Digest`를 가져온다는 내용이다.
<br />

여기서 `::` 기호는 'namespace 연산자' 또는 '경로 지정자'라는 이름으로, Rust에서 모듈이나 크레이트의 '내부'로 들어가 해당 스코프 내의 특정 요소들에 접근할 수 있게 해준다.

> 궁금한 점!! **`=`과 `::`은 어떤 차이가 있을까??**
>
> - '=' 연산자
>   '='는 대입 연산자로 사용된다. 이 연산자는 오른쪽 피연산자의 값을 왼쪽 피연산자(보통 변수)에 할당하는 데 사용된다.
> - '::' 연산자
>   '::'는 '경로 지정자' 또는 'namespace 연산자'로, 모듈, 크레이트, 타입 또는 특정 스코프 내의 요소에 접근할 때 사용된다.
>
> 즉, **'='는 값이나 표현식을 변수에 할당할 때 사용되고, '::'는 모듈이나 타입의 멤버에 접근할 때 사용된다.**

<br />

<br />
익숙한 Javascript 코드로 변경해보면 다음과 같다.

```js
/*
    rust 코드
    use sha2::{Sha256, Digest};
*/

// ES6(ES2015) 이후의 JavaScript에서 import 문은 모듈에서 특정 기능을 가져오는 코드
import { Sha256, Digest } from "sha2";

// CommonJS 모듈 시스템(주로 Node.js에서 사용)에서 require 함수는 모듈 전체를 가져오거나 모듈의 특정 부분을 추출하는 데 사용되는 코드
const { Sha256, Digest } = require("sha2");
```

<br />

### 2-3. 변수 선언하기

Rust는 JS와 비슷하게 변수를 선언하려면 `let`과 `const`를 사용 할 수 있다.
<br />

나는 이제 `sha2` 크레이트를 사용하기 위해 `main.rs`를 다음과 같이 코드를 짰다.

> 코드 해석은 다음 포스트로...
> <br />

```rust
use sha2::{Sha256, Digest};

fn main() {
    // 해싱할 문자열
    let data = "Hello, world!";

    // SHA-256 객체 생성
    let mut hasher = Sha256::new();

    // 데이터 업데이트
    hasher.update(data);

    // 해시 계산 완료
    let result = hasher.finalize();

    // 결과 출력
    println!("SHA-256 hash of '{}' is {:?}", data, result);
}
```

<br />

`main 함수`의 처음 줄을 보면, data라는 변수에 "Hello, world!" 문자열을 담았다.
<br />

```rust
...

let data = "Hello, world!";

...
```

<br />

이것만 보면 JS와 다른 것이 없다. 먼저, Rust는 JS와 같이 `let`, `const`를 사용한다. 하지만 Rust의 이 `let`, `const`는 JS와 다른 비밀이 있다.
<br />

**Rust vs JS의 `let`, `const`**

- `let`
  - JS: `let` 키워드로 선언된 변수는 항상 가변(mutable) 변수이다.
  - Rust:
    - `let` 키워드로 선언된 변수는 기본적으로 불변(immutable)이지만, 가변(mutable)이 가능하다.
    - `let` 키워드로 선언된 변수를 가변(mutable)으로 만들기
      ```rust
      let x = 5; // 불변 변수 선언
      let mut y = 5; // 가변 변수 선언
      y = 10; // 가능
      ```
- `const`

  - JS: `const` 키워드로 선언된 변수는 항상 불변(immutable) 변수이다.
    (배열(array)같은 참조 타입의 자료형의 내부 요소들은 변경 가능하다.)
  - Rust:

    - `const`는 컴파일 타임에 결정되는 상수를 선언하는 데 사용된다.
      > - 컴파일 타임: 프로그램이 실행되기 전, 소스 코드가 실행 가능한 기계 코드로 변환되는 과정을 의미한다. 이 과정에서 프로그램의 구조, 타입 체크, 변수 할당 등이 수행되는 것.
    - 상수는 항상 불변(immutable)이며, 타입을 명시적으로 지정해야 한다.

      ```rust
      // 타입 명시 예제 코드
      // u32는 솔리디티의 uint32라고 보면 된다.
        // u32: 부호 없는 32비트 정수 (0 ~ 4,294,967,295)
      // 여기서 숫자의 '_'는 가독성을 위해 사용된다.(세 자리 마다 쓰임)
      const MAX_POINTS: u32 = 100_000;
      ```

    - `const`는 전역 상수 또는 모듈 수준에서 주로 사용됩니다.
      - 전역 상수: JS로 치면 전역 스코프에서 선언되어 사용된다는 말이다.
      - 모듈 수준:
        - Rust에서 모듈 수준이란 특정 모듈 내에서 정의되고 사용되는 함수, 구조체, 상수 등을 의미한다.
        - 모듈은 Rust 코드를 구조화하고 관련된 기능을 그룹화하는 데 사용된다. 예를 들어, 관련된 함수들을 하나의 모듈로 묶어 관리할 수 있다.
        - 모듈 내에서 정의된 요소들은 기본적으로 해당 모듈 내부에서만 접근 가능합니다. 하지만 pub 키워드를 사용하여 다른 모듈에서도 접근할 수 있도록 만들 수 있다.(약간 솔리디티에서 접근 제한 관리 같은 너낌)
