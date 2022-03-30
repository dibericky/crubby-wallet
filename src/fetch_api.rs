use reqwest;
use serde::Deserialize;

#[derive(Debug)]
pub struct MonthlyRow {
    pub name: String,
    pub monthly_value: f32,
    pub payment: String,
    pub label: String,
    pub total_value: f32,
    pub duration_montly: f32,
    pub id: i32,
    pub note: String
}

#[derive(Deserialize, Debug)]
struct ResponseItem {
    nome: String,
    valoreAlMese: f32,
    pagamento: String,
    etichetta: String,
    valoreTotale: f32,
    durataRispettoAMese: f32,
    nota: Option<String>,
    id: i32
}

#[derive(Deserialize, Debug)]
struct Response {
    db: Vec<ResponseItem>
}

pub async fn fetch (api_url: &str) -> Result<Vec<MonthlyRow>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(api_url)
        .send()
        .await?;
    let res = res.json::<Response>().await?;
    let res = res.db.iter().map(move |item| {
        let default_str = String::from("");
        let nota = item.nota.clone().or_else(|| Some(default_str)).unwrap();
        MonthlyRow {
            name: item.nome.to_owned(),
            monthly_value: item.valoreAlMese,
            payment: item.pagamento.to_owned(),
            label: item.etichetta.to_owned(),
            total_value: item.valoreTotale,
            duration_montly: item.durataRispettoAMese,
            note: nota,
            id: item.id
        }
    }).collect();
    Ok(res)
}