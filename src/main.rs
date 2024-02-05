use scraper;
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time};

fn main() {
    let urls: Vec<String> = vec![
    // abstract
    String::from("https://wallhaven.cc/search?q=id%3A74&categories=100&purity=100&atleast=2560x1440&sorting=views&order=desc&ai_art_filter=1&page="),
    // nature
    "https://wallhaven.cc/search?q=id%3A37&categories=100&purity=100&atleast=2560x1440&sorting=views&order=desc&ai_art_filter=1&page=".to_string(),
];

    let mut file = OpenOptions::new()
        .write(true)
        .open("image_urls.txt")
        .expect("Unable to open file");

    for url in urls {
        for i in 1..=100 {
            let page = format!("{}{}", url, i);
            let response = reqwest::blocking::get(&page).unwrap().text().unwrap();
            //println!("page: {}", response);

            get_images(&response).iter().for_each(|img| {
                println!("img: {}", img);
                file.write_all(format!("{}\n", img).as_bytes())
                    .expect("Unable to write data");
            });

            let two_seconds = time::Duration::from_millis(2000);
            thread::sleep(two_seconds);
        }
    }
}

fn get_images(html: &str) -> Vec<String> {
    let document = scraper::Html::parse_document(&html);

    let thumbs_selector = scraper::Selector::parse("#thumbs > section:nth-child(1)").unwrap();
    let li_selector = scraper::Selector::parse("li").unwrap();
    let figure_selector = scraper::Selector::parse("figure").unwrap();
    let img_selector = scraper::Selector::parse("img").unwrap();
    let thumb_info_selector = scraper::Selector::parse("div").unwrap();

    let mut images = Vec::new();
    for element in document.select(&thumbs_selector) {
        for li in element.select(&li_selector) {
            //println!("src: {}\n", li.inner_html());
            for fig in li.select(&figure_selector) {
                //println!("fig: {}", fig.inner_html());
                let img_src = fig
                    .select(&img_selector)
                    .next()
                    .unwrap()
                    .value()
                    .attr("data-src")
                    .unwrap();

                let thumb_info = fig.select(&thumb_info_selector).next().unwrap();
                if thumb_info.inner_html().contains("png") {
                    images.push(get_full_src(img_src, "png"));
                } else {
                    images.push(get_full_src(img_src, "jpg"));
                }
            }
        }
    }
    images
}

fn get_full_src(src: &str, ext: &str) -> String {
    let parts = src.split("/").collect::<Vec<&str>>();
    let filename = parts[5].split(".").collect::<Vec<&str>>()[0];

    //println!("parts 5: {}", filename); println!("ext: {}", ext);
    let mut full_src = String::from("https://w.wallhaven.cc/full/");
    full_src.push_str(parts[4]);
    full_src.push_str("/wallhaven-");
    full_src.push_str(filename);
    full_src.push_str(".");
    full_src.push_str(ext);

    full_src
}
