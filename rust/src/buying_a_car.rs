pub fn nb_months(old_: i32, new_: i32, saving_: i32, loss_percent_: f64) -> (i32, i32) {
    let decrease_per_month: f64 = 0.5f64;
    let mut loss_percent: f64 = loss_percent_ as f64;
    let mut old: f64 = old_ as f64;
    let mut new: f64 = new_ as f64;
    let mut saving: f64 = 0.0f64;
    let mut loop_index: i32 = 0;

    loop {
        if (saving + old) >= new {
            let result_ = (saving + old) - new;
            return (loop_index, result_.round() as i32);
        }        

        saving += saving_ as f64;
        loop_index += 1;
        
        if loop_index % 2 == 0 {
            loss_percent += decrease_per_month;
        }

        old -= (old * loss_percent) / 100.0f64;
        new -= (new * loss_percent) / 100.0f64;
    }
}       

// Let us begin with an example:

// A man has a rather old car being worth $2000. He saw a secondhand car being worth $8000. He wants to keep his old car until he can buy the secondhand one.

// He thinks he can save $1000 each month but the prices of his old car and of the new one decrease of 1.5 percent per month. Furthermore this percent of loss increases of 0.5 percent at the end of every two months. Our man finds it difficult to make all these calculations.

// Can you help him?

// How many months will it take him to save up enough money to buy the car he wants, and how much money will he have left over?

// Parameters and return of function:

// parameter (positive int or float, guaranteed) startPriceOld (Old car price)
// parameter (positive int or float, guaranteed) startPriceNew (New car price)
// parameter (positive int or float, guaranteed) savingperMonth 
// parameter (positive float or int, guaranteed) percentLossByMonth

// nbMonths(2000, 8000, 1000, 1.5) should return [6, 766] or (6, 766)
