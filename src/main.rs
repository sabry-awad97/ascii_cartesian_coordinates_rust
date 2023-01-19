pub struct Grid {
    cells: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
}

#[derive(Debug, PartialEq)]
enum Alignment {
    Left,
    Center,
    Right,
}

impl Grid {
    fn new(num_rows: usize, num_cols: usize, empty_cell_char: char) -> Self {
        let cells = vec![vec![empty_cell_char; num_cols]; num_rows];
        Self {
            cells,
            num_rows,
            num_cols,
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, character: char) {
        self.cells[row][col] = character;
    }

    fn draw(
        &self,
        cell_width: usize,
        border_char: char,
        row_separator_char: char,
        col_separator_char: char,
        x_axis_separator_char: char,
        alignment: Alignment,
    ) {
        let frame_top_bottom = (0..self.num_cols)
            .map(|_| border_char.to_string().repeat(cell_width))
            .collect::<Vec<_>>()
            .join(&border_char.to_string());

        let row_template = format!(
            "{row_num}{row_separator}{row_values}{row_values_separator}",
            row_num = "{row_num}",
            row_separator = row_separator_char,
            row_values = "{row_values}",
            row_values_separator = row_separator_char
        );

        let row_num = row_template.replace(
            "{row_num}",
            format!("{:width$}", "", width = cell_width).as_str(),
        );

        let frame_top = row_num.replace("{row_values}", &frame_top_bottom);

        println!("{}", frame_top);

        for (row_num, row) in self.cells.iter().rev().enumerate() {
            let row_num_str = format!("{:width$}", self.num_rows - row_num - 1, width = cell_width);
            let row_values: Vec<String> = row
                .iter()
                .map(|cell| {
                    let aligned_string: String = match alignment {
                        Alignment::Left => format!("{:<width$}", cell, width = cell_width),
                        Alignment::Center => format!("{:^width$}", cell, width = cell_width),
                        Alignment::Right => format!("{:>width$}", cell, width = cell_width),
                    };

                    aligned_string
                })
                .collect();

            println!(
                "{}",
                row_template
                    .replace(
                        "{row_num}",
                        format!("{:<width$}", row_num_str, width = cell_width).as_str()
                    )
                    .replace(
                        "{row_values}",
                        &row_values.join(&col_separator_char.to_string())
                    )
            );
        }
        println!("{}", frame_top);

        let col_nums: Vec<String> = (0..self.num_cols)
            .map(|col| {
                let aligned_string: String = match alignment {
                    Alignment::Left => format!("{:<width$}", col, width = cell_width),
                    Alignment::Center => format!("{:^width$}", col, width = cell_width),
                    Alignment::Right => format!("{:>width$}", col, width = cell_width),
                };

                aligned_string
            })
            .collect();
        let x_axis = row_num.replace(
            "{row_values}",
            &col_nums.join(&x_axis_separator_char.to_string()),
        );
        println!("{}", x_axis);
    }
}

fn main() {
    let mut grid = Grid::new(15, 15, ' ');
    grid.set_cell(0, 0, 'X');
    grid.set_cell(1, 1, 'O');
    grid.set_cell(2, 2, 'X');
    grid.draw(2, '+', '|', ' ', ' ', Alignment::Right);
}
