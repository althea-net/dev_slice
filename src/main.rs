use plotters::prelude::*;

const YEARS: u64 = 20;
const REDELEGATIONS_PER_YEAR: u64 = 1;
const STAKING_RATE: f64 = 0.7;

struct Period {
    total_tokens: f64,
    inflation_rate: f64,
    dev_slice_rate: f64,
    total_tokens_created: f64,
    total_tokens_created_dev_slice: f64,
    total_tokens_created_from_delegated_dev_slice: f64,
}

fn redelegate(i: u64, last_period: Period) -> Period {
    let effective_inflation: f64 = (last_period.inflation_rate
        - ((i as f64 * 0.01) / REDELEGATIONS_PER_YEAR as f64))
        / REDELEGATIONS_PER_YEAR as f64;

    let staking_rewards = last_period.total_tokens * STAKING_RATE * effective_inflation;

    let staking_rewards_delegated_dev_slice = (last_period
        .total_tokens_created_from_delegated_dev_slice
        + last_period.total_tokens_created_dev_slice)
        * STAKING_RATE
        * effective_inflation;

    println!("{:?}", staking_rewards);

    Period {
        total_tokens: last_period.total_tokens + staking_rewards,
        inflation_rate: last_period.inflation_rate,
        dev_slice_rate: last_period.dev_slice_rate,
        total_tokens_created: last_period.total_tokens_created + staking_rewards,
        total_tokens_created_dev_slice: last_period.total_tokens_created_dev_slice
            + (staking_rewards * last_period.dev_slice_rate),
        total_tokens_created_from_delegated_dev_slice: last_period
            .total_tokens_created_from_delegated_dev_slice
            + staking_rewards_delegated_dev_slice,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut period = Period {
        total_tokens: 10_000_000.0,
        inflation_rate: 0.15,
        dev_slice_rate: 0.2,
        total_tokens_created: 0.0,
        total_tokens_created_dev_slice: 0.0,
        total_tokens_created_from_delegated_dev_slice: 0.0,
    };
    let redelegations = REDELEGATIONS_PER_YEAR * YEARS;
    let mut total_tokens_series: Vec<(f32, f32)> = vec![];
    let mut total_tokens_created_series: Vec<(f32, f32)> = vec![];
    let mut total_tokens_created_dev_slice_series: Vec<(f32, f32)> = vec![];
    let mut total_tokens_created_from_delegated_dev_slice_series: Vec<(f32, f32)> = vec![];

    for i in 0..redelegations {
        period = redelegate(i, period);
        total_tokens_series.push((i as f32, period.total_tokens as f32));
        total_tokens_created_series.push((i as f32, period.total_tokens_created as f32));
        total_tokens_created_dev_slice_series
            .push((i as f32, period.total_tokens_created_dev_slice as f32));
        total_tokens_created_from_delegated_dev_slice_series.push((
            i as f32,
            period.total_tokens_created_from_delegated_dev_slice as f32,
        ));
    }

    draw_chart(
        total_tokens_series,
        total_tokens_created_series,
        total_tokens_created_dev_slice_series,
        total_tokens_created_from_delegated_dev_slice_series,
    )
}

fn draw_chart(
    total_tokens_series: Vec<(f32, f32)>,
    total_tokens_created_series: Vec<(f32, f32)>,
    total_tokens_created_dev_slice_series: Vec<(f32, f32)>,
    total_tokens_created_from_delegated_dev_slice_series: Vec<(f32, f32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("chart.svg", (640, 480)).into_drawing_area();
    root.fill(&White)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        // .caption("This is our first plot", ("monospace", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(100)
        .y_label_area_size(100)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_ranged(
            0f32..YEARS as f32 * REDELEGATIONS_PER_YEAR as f32,
            0f32..100_000_000f32,
        )?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(YEARS as usize)
        .y_labels(15)
        // We can also change the format of the label text
        .x_label_formatter(&|x| format!("{:.1}", x / REDELEGATIONS_PER_YEAR as f32))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(total_tokens_series, &Red))?;
    chart.draw_series(LineSeries::new(total_tokens_created_series, &Blue))?;
    chart.draw_series(LineSeries::new(
        total_tokens_created_dev_slice_series,
        &Blue,
    ))?;
    chart.draw_series(LineSeries::new(
        total_tokens_created_from_delegated_dev_slice_series,
        &Green,
    ))?;

    // Similarly, we can draw point series
    // chart.draw_series(PointSeries::of_element(
    //     vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
    //     5,
    //     &Red,
    //     &|c, s, st| {
    //         return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //         + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
    //         + Text::new(format!("{:?}", c), (10, 0), ("Arial", 10).into_font());
    //     },
    // ))?;
    Ok(())
}