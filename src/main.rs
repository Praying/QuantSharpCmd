use clap::{App, Arg};

fn main() {
    let matches = App::new("qscmd")
        .version("0.1")
        .author("Praying <snowfallvilla@163.com>")
        .about("QuantSharp Command")
        .subcommand(
            App::new("save")
                .about("save market data to database")
                .version("0.1")
                .author("Praying <snowfallvilla@163.com>")
                .arg(
                    Arg::new("exchange")
                        .short('e')
                        .value_name("binance")
                        .takes_value(true)
                        .required(true)
                        .about("exchange, such as binance, okex and so on"),
                )
                .arg(
                    Arg::new("symbol")
                        .short('s')
                        .value_name("symbol")
                        .takes_value(true)
                        .required(true)
                        .about("symbol, such as ETH-USDT,BTC-USDT"),
                )
                .arg(
                    Arg::new("frequency")
                        .short('f')
                        .value_name("frequency")
                        .takes_value(true)
                        .required(true)
                        .about("frequency, 1min,5min,10min,1h,1day"),
                )
                .arg(
                    Arg::new("begin")
                        .value_name("begin")
                        .takes_value(true)
                        .required(true)
                        .about("begin time, such as 2011-10-01"),
                )
                .arg(
                    Arg::new("end")
                        .value_name("end")
                        .takes_value(true)
                        .required(true)
                        .about("end time, such as 2011-10-01"),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("save") {
        if let Some(exchange) = matches.value_of("exchange") {
            println!("exchange:{}", exchange);
        }
        if let Some(symbol) = matches.value_of("symbol") {
            println!("symbol:{}", symbol);
        }
        if let Some(frequency) = matches.value_of("frequency") {
            println!("frequency = {}", frequency)
        }
        if let Some(begin) = matches.value_of("begin") {
            println!("begin = {}", begin)
        }
        if let Some(end) = matches.value_of("end") {
            println!("end = {}", end)
        }
    }
}
