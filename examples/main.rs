use anyhow::Result;
use pagemaker::chart::plot_type::Time;
use pagemaker::chart::{plot_type, Dataset, Pair, XYPlot};
use pagemaker::{palette, ColorMap, Columns, Field, Page, Row, Style, Table, Text, TextObject};

fn main() -> Result<()> {
    let page = Page::default().set_title("Demo");

    let text = Text::new("Hello World!")
        .set_style(Style::Color, palette::RED)
        .set_style(Style::FontSize, 36)
        .clone();
    let page = page.append_text(text).unwrap();

    let header = Row::new()
        .add_field(Field::new("Name"))
        .add_field(Field::new("Age"))
        .add_field(Field::new("Height"));
    let table1 = Table::new(header).set_title("Demo Table");

    let colormap = ColorMap::new(vec![
        (0.0, palette::GREEN.to_string()),
        (0.5, palette::YELLOW.to_string()),
        (1.0, palette::RED.to_string()),
    ])
    .unwrap();
    let table1 = table1
        .add_row(
            Row::new()
                .add_field(Field::new("A"))
                .add_field(
                    Field::new("20")
                        .set_style(Style::BackgroundColor, palette::YELLOW)
                        .set_style(Style::Color, palette::CYAN)
                        .clone(),
                )
                .add_field(Field::new("1.80"))
                .clone(),
        )
        .set_style(Style::Color, palette::GREEN)
        .clone()
        .add_row(
            Row::new()
                .add_field(Field::new("B"))
                .add_field(Field::new("19").set_style(Style::Color, palette::RED))
                .add_field(Field::new("-1.70"))
                .set_style(Style::BackgroundColor, palette::BLUE)
                .clone(),
        )
        .apply_to_column(2, Style::Color, &|content| {
            Some(colormap.get_color(content)?.into())
        });

    let table2 = Table::new(
        Row::new()
            .add_field(Field::new("Id"))
            .add_field(Field::new("Time"))
            .add_field(Field::new("Value")),
    )
    .set_title("Demo Table 2")
    .add_row(
        Row::new()
            .add_field(Field::new("1"))
            .add_field(Field::new("2021-01-01 00:00:00"))
            .add_field(Field::new("1.0")),
    )
    .add_row(
        Row::new()
            .add_field(Field::new("2"))
            .add_field(Field::new("2021-01-01 00:00:01"))
            .add_field(Field::new("2.0"))
            .set_style(Style::Color, palette::GREEN)
            .clone(),
    );

    let columns = Columns::new().add_column(table1)?.add_column(table2)?;

    let data1 = Dataset::from_vec(vec![
        Pair::new(1609322800000, 0.1),
        Pair::new(1609372800000, 0.1),
        Pair::new(1609459200000, 0.2),
        Pair::new(1609545600000, 0.3),
        Pair::new(1609632000000, 0.4),
        Pair::new(1609718400000, 0.5),
        Pair::new(1609804800000, 0.6),
        Pair::new(1609891200000, 0.7),
    ])
    .set_label("data1");
    let data2 = Dataset::from_vec(vec![
        Pair::new(1609509200000, 0.1),
        Pair::new(1609595600000, 0.2),
        Pair::new(1609682000000, 0.3),
        Pair::new(1609768400000, 0.4),
        Pair::new(1609854800000, 0.5),
    ])
    .set_label("data2");

    let chart = XYPlot::<plot_type::Line, Time<i64>>::default()
        .set_title("Demo Chart")
        .set_x_label("x")
        .set_y_label("y")
        .add_data(data1)
        .add_data(data2);
    page.append_text(columns)?
        .append_chart(chart)?
        .save_to_html(std::path::PathBuf::from("examples/demo.html"))?;

    Ok(())
}
