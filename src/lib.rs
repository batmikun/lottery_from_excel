use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};
use rand::Rng;

pub fn run_lottery(path: String, skipped_lines: i32) {
    let info_from_excel = read_from_file(path, skipped_lines);

    let data = match info_from_excel {
        Ok(data) => data,
        Err(_) => vec![(2, String::from("There was an error"))],
    };

    let mut finish = false;

    loop {
        let random_number = obtain_random_number();

        for tuple in &data {
            if random_number == tuple.0 {
                println!("The Winner Is {}", tuple.1);
                finish = true;
                break;
            }
        }

        if finish == true {
            break;
        }
    }
}

fn read_from_file(path: String, mut skipped_lines: i32) -> Result<Vec<(i32, String)>, Error> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(Error::Msg("Cannot find sheet"))??;

    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    let mut people = vec![];

    while let Some(result) = iter.next() {
        if skipped_lines != 0 {
            skipped_lines -= 1;
            continue;
        }

        let (number, _, name): (i32, String, String) = result?;

        if name != "" {
            people.push((number, name));
        }
    }

    Ok(people)
}

fn obtain_random_number() -> i32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..500) as i32
}
