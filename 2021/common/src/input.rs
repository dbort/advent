// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html

fn get_url_body(url: &str) -> String {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();

    let mut body = String::new();
    let async_block = async {
        body = reqwest::get(url).await.unwrap().text().await.unwrap();
    };
    runtime.block_on(async_block);
    body
}

pub fn load_file(name: &str) -> String {
    if name.starts_with("https://") {
        get_url_body(name)
    } else {
        std::fs::read_to_string(name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
