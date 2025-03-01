use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_missing_api_key() {
    // CLI argümanı veriliyor ancak DEEPL_API_KEY ayarlanmıyor
    let mut cmd = Command::cargo_bin("translate-cli").unwrap();
    let assert = cmd
        .args(&["Hello"])
        .env_remove("DEEPL_API_KEY")
        // Hedef dil girişi için "EN\n" veriyoruz.
        .write_stdin("EN\n")
        .assert();
    // Program DEEPL_API_KEY eksikliğinde exit kodu 1 döndürmeli
    assert.failure().code(1);
}

#[test]
fn test_no_text_provided() {
    // CLI argümanı yok ve interaktif modda boş metin giriliyor
    let mut cmd = Command::cargo_bin("translate-cli").unwrap();
    let assert = cmd
        .write_stdin("\n") // boş metin için
        .assert();
    // Hiç metin girilmediğinden exit kodu 1 ile sonlanmalı
    assert.failure().code(1).stderr(contains("No text provided."));
}
