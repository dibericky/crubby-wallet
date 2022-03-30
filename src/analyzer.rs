use crate::fetch_api::{MonthlyRow};

fn get_positive_negative (rows: &Vec<MonthlyRow>) -> (Vec<&MonthlyRow>, Vec<&MonthlyRow>) {
    let mut positives : Vec<&MonthlyRow> = vec![];
    let mut negatives : Vec<&MonthlyRow> = vec![];

    for row in rows {
        if row.monthly_value > 0.0 {
            positives.push(row)
        } else {
            negatives.push(row)
        }
    }
    return (positives, negatives)
}

#[derive(Debug)]
pub struct Overview {
    salary: f32,
    gain: f32,
    saved_percentage: f32,
    loss: f32
}

impl Overview {
    pub fn calculate(rows: &Vec<MonthlyRow>) -> Overview {
        let (pos, neg) = get_positive_negative(rows);
        let sum_pos = pos.iter()
            .map(|item| item.monthly_value)
            .reduce(|acc, item| acc +  item).or(Some(0.0)).unwrap();
        let sum_loss = neg.iter()
            .map(|item| item.monthly_value)
            .reduce(|acc, item| acc +  item).or(Some(0.0)).unwrap();
        let sum_loss = -1.0 * sum_loss;
        Overview {
            salary: sum_pos,
            loss: sum_loss,
            gain: sum_pos - sum_loss,
            saved_percentage: (1.0 - sum_loss/sum_pos)*100.0
        }
    }
}