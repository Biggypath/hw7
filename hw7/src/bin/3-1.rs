
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut f1: f64 = args[1].parse().unwrap();
    let f2: f64 = args[2].parse().unwrap();
    let frequen: f64 = args[3].parse().unwrap();

    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
<html>
    <head>
        <title>Fahr to Celcius</title>
        <style> table, th, td {
            border: 1px solid #000000;
            text-align: center;
            width: 50%;
            border-collapse: collapse; 
            }
        </style>
        <h1>Fahr to Cel</h1>
    </head>
    <body>
        <table>
            <thead>
                <tr>
                    <th>Fahr</th>
                    <th>Celcius</th>
                </tr>
            </thead>
            <tbody>");

    if f1 < f2 {
        while f1 <= f2 {
            let c:f64 =  (5.0 / 9.0 ) * (f1 - 32.0);
            let c : String= format!("{:.1}", c).parse().unwrap();
            table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", f1, c));

            f1 += frequen;
    
        } 
    } else if f1 > f2 {
        while f1 >= f2 {
            let c1 = (5.0 / 9.0 ) * (f1 - 32.0);
            let c1: String = format!("{:.1}", c1).parse().unwrap();
            table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", f1, c1));

            f1 -= frequen; 
        }
    } 

    table.push_str("</tbody></table></body></html>");

    println!("{}", table)
    
}  
