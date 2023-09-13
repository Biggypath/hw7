
fn main() {
    let arg: Vec<_> = std::env::args().collect();
    let x:f64 = arg[1].parse().unwrap();
    let x_pow2 = x.powf(2.);
    let x_pow3 = x.powf(3.);
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>X to the power of ...</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse;
                table-layout: fixed;
                }
            </style>
            <h1>X to the power of ...</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>x</th>
                        <th>x^2</th>
                        <th>x^3</th>
                    </tr>
                </thead>
                <tbody>
    ");
    table.push_str(&format!("<tr><td>{}</td><td>{}</td><td>{}</td></tr>", x, x_pow2, x_pow3));
    table.push_str("</tbody></table></body></html>");
    println!("{}", table)
}