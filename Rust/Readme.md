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
