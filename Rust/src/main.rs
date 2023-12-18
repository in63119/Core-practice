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
