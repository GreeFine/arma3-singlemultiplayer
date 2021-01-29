pub fn fetch(url: &String) -> String {
    println!("request url {}", url);
    let res = reqwest::blocking::get(url).unwrap();
    println!("Status: {}", &res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().unwrap();
    println!("Body:\n{}", body);
    body
}
