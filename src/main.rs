mod lang;

use dotenv::dotenv;
use std::env;
use std::io::{self, Write};
use colored::*; // eklenen: renk kütüphanesi kullanımı
use clipboard::{ClipboardContext, ClipboardProvider}; // eklenen

use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Ortam değişkenlerini .env dosyasından yükle

    // Kullanıcıdan çevrilecek metni al (CLI argümanı varsa kullan, yoksa interaktif sor)
    let args: Vec<String> = env::args().skip(1).collect();
    let mut input_text = String::new();
    if !args.is_empty() {
        input_text = args.join(" ");
    } else {
        // Görsel başlangıç ekranı
        let border = "***********************".blue();
        println!("{}", border);
        println!("{}", "*   Translate CLI     *".blue().bold());
        println!("{}", border);
        println!();
        println!("{}", lang::translate("please_enter_your_text").white().bold());
        print!("{}", lang::translate("enter_translate_text").green());
        io::stdout().flush()?;
        io::stdin().read_line(&mut input_text)?;
    }

    if input_text.trim().is_empty() {
        eprintln!("No text provided.");
        std::process::exit(1);
    }

    // Kullanıcıdan hedef dili opsiyonel al. Eğer boş ise bilgisayarın LANG ortam değişkenini kullan.
    let mut target_lang_input = String::new();
    print!("{}", lang::translate("enter_target_lang").green());
    io::stdout().flush()?;
    io::stdin().read_line(&mut target_lang_input)?;
    let target_lang = if target_lang_input.trim().is_empty() {
        let lang = env::var("LANG").unwrap_or_else(|_| "EN".to_string());
        let lang = lang.split('.').next().unwrap(); // örn: "tr_TR"
        lang.split('_').next().unwrap_or(lang).to_uppercase()
    } else {
        target_lang_input
            .trim()
            .split('_')
            .next()
            .unwrap_or(target_lang_input.trim())
            .to_uppercase()
    };
    
    // DeepL API anahtarını ortam değişkeninden al
    let api_key = match env::var("DEEPL_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("{}", lang::translate("missing_api_key"));
            std::process::exit(1);
        }
    };

    // DeepL API'ya HTTP POST isteği gönder
    let client = reqwest::Client::new();
    let res = client.post("https://api-free.deepl.com/v2/translate")
        .form(&[
            ("auth_key", api_key.as_str()),
            ("text", input_text.trim()),
            ("target_lang", &target_lang),
        ])
        .send()
        .await?;

    // API'dan gelen cevabı deserialize et
    #[derive(Deserialize)]
    struct Translation {
        text: String,
    }
    #[derive(Deserialize)]
    struct TranslationResponse {
        #[serde(default)]
        translations: Vec<Translation>,
    }
    let deepl_res: TranslationResponse = res.json().await?;
    
    if let Some(translation) = deepl_res.translations.first() {
        let content = translation.text.trim();
        let box_width = content.len() + 4;  // iki kenar boşluğu + padding
        
        // Yeni başlık
        println!("\n{}\n", lang::translate("translate_result").green().bold());
        // Üst çerçeve
        println!("┌{:-^width$}┐", "", width = box_width);
        // Ortalanmış metin satırı
        println!("│ {0:^width$} │", content, width = box_width - 2);
        // Alt çerçeve
        println!("└{:-^width$}┘", "", width = box_width);
        
        // Kopyalama özelliği
        println!("\n{}", lang::translate("copy_prompt"));
        let mut copy_input = String::new();
        io::stdout().flush()?;
        io::stdin().read_line(&mut copy_input)?;
        if copy_input.trim().eq_ignore_ascii_case("E") {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(content.to_string()).unwrap();
            println!("{}", lang::translate("result_copy_success").green());
        }
    } else {
        println!("\n{}", lang::translate("translate_not_found").red());
    }
    
    Ok(())
}
