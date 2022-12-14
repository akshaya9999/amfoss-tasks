// fn read_num<T>() -> T
// where
//     T: std::str::FromStr,
//     <T as std::str::FromStr>::Err: std::fmt::Debug,
// {
//     std::io::stdin()
//         .lock()
//         .lines()
//         .next()
//         .unwrap()
//         .unwrap()
//         .parse::<T>()
//         .unwrap()
// }
use std::io;
use std::error::Error;
// use csv::Writer;

fn main() {

    let response = reqwest::blocking::get(
        "https://crypto.com/price",
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);


    
    

    let title_selector = scraper::Selector::parse("div.css-ttxvk0>p").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));


    let price_selector = scraper::Selector::parse("div.css-b1ilzc>p").unwrap();
    let price = document.select(&price_selector).map(|x| x.inner_html());
    price
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));


    let tfhrc_selector = scraper::Selector::parse("td.css-1b7j986>p").unwrap();
    let tfhrc = document.select(&tfhrc_selector).map(|x| x.inner_html());
    tfhrc
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
    
    
    let mut input = String::new();
   
   

    let tfhrv_selector = scraper::Selector::parse("td.css-1nh9lk8").unwrap();
    let tfhrv = document.select(&tfhrv_selector).map(|x| x.inner_html());
    tfhrv
        .zip(1..101)
        .for_each(|input| io::stdin().read_line(&mut input) );
    
   
    

    
    // let marketc_selector = scraper::Selector::parse("td.css-1nh9lk8").unwrap();
    // let marketc = document.select(&marketc_selector).map(|x| x.inner_html());
    // marketc
    //     .zip(1..101)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));
    
    let mut input: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", input);


    }
   

        

            


    



            
        
        


    
    
        
    

