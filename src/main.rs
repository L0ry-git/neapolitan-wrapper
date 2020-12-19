#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let args = args.iter().skip(1);

    let fixed = args.filter(|e| !e.is_empty())
        .map(|e| e.to_string()).collect::<Vec<String>>().join("+");

    let url_base = String::from("http://www.giggino.com/traduzione-italiano-napoletano.asp?t=") + fixed.as_str();
    if fixed.is_empty() {
        eprintln!("Empty data!");
        return Ok(())
    }
	
    let body = reqwest::get(url_base.as_str())
        .await?
        .text()
        .await?;
    let start_idx = body.find("Ecco la frase che Giggino ha tradotto per te:").unwrap();
    let end_idx = body.find("</span></td>").unwrap();
    let result = body[start_idx..end_idx].to_string();
    let result = &result[result.find("<td><span class=\"Stile3\">").unwrap() + 25..]
        .trim()
        .replace("  ", " ")
        .replace("�", "");

    println!("result = {}", result);
    Ok(())
}
