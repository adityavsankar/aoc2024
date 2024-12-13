use crate::DayResult;

struct Row {
    step: String,
    result: String,
    time_taken: String,
}

struct TableColumnWidths {
    step: usize,
    result: usize,
    time: usize,
}

impl TableColumnWidths {
    fn horizontal_line(&self, left: char, middle: char, right: char) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            left,
            "─".repeat(self.step + 2),
            middle,
            "─".repeat(self.result + 2),
            middle,
            "─".repeat(self.time + 2),
            right
        )
    }

    fn format_row(&self, row: (&str, &str, &str)) -> String {
        format!(
            "│ {:step_width$} │ {:result_width$} │ {:time_width$} │",
            row.0,
            row.1,
            row.2,
            step_width = self.step,
            result_width = self.result,
            time_width = self.time
        )
    }
}

fn create_rows(result: DayResult) -> Vec<Row> {
    let DayResult {
        parse_duration,
        part1,
        part2,
    } = result;

    vec![
        Row {
            step: String::from("Parsing"),
            result: String::from("-"),
            time_taken: format!("{parse_duration:?}"),
        },
        Row {
            step: String::from("Part 1"),
            result: format!("{}", part1.result),
            time_taken: format!("{:?}", part1.duration),
        },
        Row {
            step: String::from("Part 2"),
            result: format!("{}", part2.result),
            time_taken: format!("{:?}", part2.duration),
        },
        Row {
            step: String::from("Total"),
            result: String::from("-"),
            time_taken: format!("{:?}", parse_duration + part1.duration + part2.duration),
        },
    ]
}

fn calculate_dimensions(rows: &[Row], min_width: usize) -> TableColumnWidths {
    let step_width = rows
        .iter()
        .map(|r| r.step.len())
        .max()
        .unwrap_or(0)
        .max("Step".len())
        .max(min_width);

    let result_width = rows
        .iter()
        .map(|r| r.result.len())
        .max()
        .unwrap_or(0)
        .max("Result".len())
        .max(min_width);

    let time_width = rows
        .iter()
        .map(|r| r.time_taken.len())
        .max()
        .unwrap_or(0)
        .max("Time Taken".len())
        .max(min_width);

    TableColumnWidths {
        step: step_width,
        result: result_width,
        time: time_width,
    }
}

pub fn print_table(result: DayResult, min_width: usize) {
    let rows = create_rows(result);
    let dims = calculate_dimensions(&rows, min_width);

    println!("{}", dims.horizontal_line('┌', '┬', '┐'));
    println!("{}", dims.format_row(("Step", "Result", "Time Taken")));
    println!("{}", dims.horizontal_line('├', '┼', '┤'));

    for row in rows.iter().take(rows.len() - 1) {
        println!(
            "{}",
            dims.format_row((&row.step, &row.result, &row.time_taken))
        );
    }

    println!("{}", dims.horizontal_line('├', '┼', '┤'));
    let total_row = &rows[rows.len() - 1];
    println!(
        "{}",
        dims.format_row((&total_row.step, &total_row.result, &total_row.time_taken))
    );
    println!("{}", dims.horizontal_line('└', '┴', '┘'));
}
