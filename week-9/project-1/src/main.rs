use std::io::Write;

fn main() {
    
    let in1 = ["33 Export","Desperados","Golberg","Gulder","Heineken","Star"];
    let in2 = ["Legend","Turbo King","Williams"];
    let in3 = ["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    let mut file = std::fs::File::create("highquality-drinks.txt").expect("create failed");
    file.write_all("Lager:".as_bytes()).expect("write failed");
    file.write_all(format!("in1").as_bytes()).expect("write failed");
    file.write_all("Stout:".as_bytes()).expect("write failed");
    file.write_all(format!("in2").as_bytes()).expect("write failed");
    file.write_all("Non-Alcoholic:".as_bytes()).expect("write failed");
    file.write_all(format!("in3").as_bytes()).expect("write failed");

    println!("\nData written to file");


}
