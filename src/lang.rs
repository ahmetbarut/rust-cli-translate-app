use sys_locale;

pub fn get_locale() -> String {
    if let Some(locale) = sys_locale::get_locale() {
        // locale string'i örn. "tr_TR", "en_US" gibi
        locale.split('.').next().unwrap_or("en").to_string()
    } else {
        "en".to_string()
    }
}

pub fn translate(key: &str) -> String {
    let locale = get_locale();
    match locale.as_str() {
        "tr_TR" | "tr" => match key {
            "greeting" => "Merhaba".to_string(),
            "farewell" => "Hoşça kal".to_string(),
            "copy_prompt" => "Çeviriyi kopyalamak ister misiniz? [E/H]: ".to_string(),
            "enter_target_lang" => "Hedef dil kodunu girin (varsayılan: EN): ".to_string(),
            "enter_translate_text" => "Çeviri için metin girin: ".to_string(),
            "translate_result" => "Çeviri Sonucu".to_string(),
            "result_copy_success" => "Çeviri kopyalandı.".to_string(),
            "please_enter_your_text" => "Lütfen metninizi girin:".to_string(),
            "translate_not_found" => "Çeviri bulunamadı.".to_string(),
            "missing_api_key" => "DEEPL API anahtarı bulunamadı. Lütfen .env dosyasını kontrol ediniz.".to_string(),
            _ => key.to_string(),
        },
        _ => match key {
            "greeting" => "Hello".to_string(),
            "farewell" => "Goodbye".to_string(),
            "please_enter_your_text" => "Please enter your text:".to_string(),
            "enter_translate_text" => "Enter text to translate: ".to_string(),
            "enter_target_lang" => "Enter target language (default: EN): ".to_string(),
            "translate_result" => "Translation Result".to_string(),
            "result_copy_success" => "Translation copied.".to_string(),
            "translate_not_found" => "Translation not found.".to_string(),
            "copy_prompt" => "Would you like to copy the translation? [Y/N]: ".to_string(),
            "missing_api_key" => "DEEPL API key not found. Please check your .env file.".to_string(),
            _ => key.to_string(),
        },
    }
}
