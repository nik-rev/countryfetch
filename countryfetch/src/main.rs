mod generated;

fn main() {
    for country in generated::Country::ALL_COUNTRIES {
        let flag = country.flag();

        println!("{flag}");
    }
}
