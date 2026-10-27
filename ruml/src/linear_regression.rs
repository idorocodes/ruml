use std::fs::read;

use ndarray::Array;

#[derive(Debug)]
pub struct Data {
    pub hours: Vec<f32>,
    pub pay: Vec<f32>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            hours: Vec::new(),
            pay: Vec::new(),
        }
    }
}

pub fn read_csv() -> Data {
    let mut data = Data::new();
    let mut vec_hours: Vec<f32> = Vec::new();
    let mut vec_pay: Vec<f32> = Vec::new();

    let file_bytes = String::from_utf8(read("./dataset.csv").expect("Could not read bytes"))
        .expect("Could not parse bytes");
    for line in file_bytes.lines() {
        let mut parts = line.split(',');

        if let (Some(hours_raw), Some(pay_raw)) = (parts.next(), parts.next()) {
            let hours = hours_raw.trim();
            let pay = pay_raw.trim();

            if hours == "Hours" || pay == "Pay" {
                continue;
            }

            let hours: f32 = hours.parse().expect("Unable to parse");
            let pay: f32 = pay.parse().expect("Unable to parse");
            vec_hours.push(hours);
            vec_pay.push(pay);
        }
    }

    data.hours = vec_hours;
    data.pay = vec_pay;

    data
}

pub fn train_loop(w: f32, b: f32, learning_rate: f32, data: Data, iter: i32) -> (f32, f32) {
    let X = Array::from(data.hours.clone());
    let y = Array::from(data.pay.clone());
    let mut final_w = w;
    let mut final_b = b;

    for _i in 1..iter {
        let y_prediction = final_w * X.clone() + final_b;

        let error = y_prediction - y.clone();

        let dw = 2.0
            * (error.clone() * X.clone())
                .mean()
                .expect("Could not find mean");
        let db = 2.0 * (error).mean().expect("Could not find mean");

        final_w -= learning_rate * dw;
        final_b -= learning_rate * db
    }

    (final_w, final_b)
}
