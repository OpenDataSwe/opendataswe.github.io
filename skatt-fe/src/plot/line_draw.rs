use crate::app::BaseCalculationData;
use plotters::backend::DrawingBackend;
use plotters::chart::ChartBuilder;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::PathElement;
use plotters::series::{AreaSeries, LineSeries};
use plotters::style::full_palette::PURPLE;
use plotters::style::{IntoFont, RGBAColor, BLUE, RED};
use skatt_lib::calculate::{calculate_incline, run_calculation_range, CalculationRange};

#[derive(Copy, Clone)]
pub struct PlotOpts {
    pub personal: bool,
    pub total: bool,
    pub left: bool,
}

pub fn draw_percentage_plots<DB: DrawingBackend>(
    drawing_area: &DrawingArea<DB, Shift>,
    calculation_range: &CalculationRange,
    data: &BaseCalculationData,
    opts: PlotOpts,
) -> Result<(), String> {
    let data = run_calculation_range(
        calculation_range,
        data.birth_year,
        data.church_member,
        &data.municipal_tax_specification,
        &data.base_deductions,
    )
    .map_err(|e| format!("Failed to run calculation: {e}"))?;
    let mut chart = ChartBuilder::on(drawing_area)
        .caption(
            "Skatt (%) över krona tjänad",
            ("sans-serif", 14).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .margin_right(30)
        .y_label_area_size(50)
        .build_cartesian_2d(calculation_range.f32_range(), 0f32..100f32)
        .map_err(|e| format!("Failed to create chart: {e}"))?;

    chart
        .configure_mesh()
        .draw()
        .map_err(|e| format!("Failed to draw chart: {e}"))?;

    let mut pts = Vec::with_capacity(data.taxes.len());
    let mut pts_tot = Vec::with_capacity(data.taxes.len());
    let mut money_left = Vec::with_capacity(data.taxes.len());
    if opts.total || opts.personal || opts.left {
        for tax in &data.taxes {
            if opts.personal {
                pts.push((tax.gross_monthly(), tax.personal_tax_percentage() * 100.));
            }
            if opts.total {
                pts_tot.push((tax.gross_monthly(), tax.total_tax_percentage() * 100.));
            }
            if opts.left {
                money_left.push((
                    tax.gross_monthly(),
                    (1. - tax.personal_tax_percentage()) * 100.,
                ));
            }
        }
    }

    let mut personal_col = RGBAColor::from(RED);
    personal_col.3 = 0.5;
    let mut total_col = RGBAColor::from(PURPLE);
    total_col.3 = 0.5;
    let mut left_col = RGBAColor::from(BLUE);
    left_col.3 = 0.5;
    chart
        .draw_series(AreaSeries::new(pts, 0.0, personal_col))
        .map_err(|e| format!("Failed to draw personal series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], personal_col));
    chart
        .draw_series(AreaSeries::new(pts_tot, 0.0, total_col))
        .map_err(|e| format!("Failed to draw total series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], total_col));
    chart
        .draw_series(AreaSeries::new(money_left, 0.0, left_col))
        .map_err(|e| format!("Failed to draw money left series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], left_col));
    Ok(())
}

#[derive(Copy, Clone)]
pub struct InclinePlotOpts {
    pub personal: bool,
    pub left: bool,
}

pub fn draw_percentage_incline<DB: DrawingBackend>(
    drawing_area: &DrawingArea<DB, Shift>,
    calculation_range: &CalculationRange,
    data: &BaseCalculationData,
    opts: InclinePlotOpts,
) -> Result<(), String> {
    let data = run_calculation_range(
        calculation_range,
        data.birth_year,
        data.church_member,
        &data.municipal_tax_specification,
        &data.base_deductions,
    )
    .map_err(|e| format!("Failed to run calculation for percentage incline: {e}"))?;
    let mut personal = None;
    let mut money_left = None;
    let mut incline_personal = Vec::with_capacity(data.taxes.len());
    let mut incline_left = Vec::with_capacity(data.taxes.len());
    let mut max_incline = 0.0f32;
    let mut min_incline = 0.0f32;
    if opts.personal || opts.left {
        for tax in &data.taxes {
            if opts.personal {
                let y2 = tax.personal_tax_percentage() * 100.;
                if let Some((x1, y1)) = personal {
                    let incline = calculate_incline(y1, y2, data.step);
                    if incline > max_incline {
                        max_incline = incline;
                    }
                    if incline < min_incline {
                        min_incline = incline;
                    }
                    incline_personal.push((x1, incline));
                }
                personal = Some((tax.gross_monthly(), y2));
            }
            if opts.left {
                let y2 = (1. - tax.personal_tax_percentage()) * 100.;
                if let Some((x1, y1)) = money_left {
                    let incline = calculate_incline(y1, y2, data.step);
                    if incline > max_incline {
                        max_incline = incline;
                    }
                    if incline < min_incline {
                        min_incline = incline;
                    }
                    incline_left.push((x1, incline));
                }
                money_left = Some((tax.gross_monthly(), y2));
            }
        }
    }

    let mut chart = ChartBuilder::on(drawing_area)
        .caption(
            "Skatt ökning (%) per krona tjänad över krona tjänad",
            ("sans-serif", 14).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .margin_right(30)
        .y_label_area_size(50)
        .build_cartesian_2d(calculation_range.f32_range(), min_incline..max_incline)
        .map_err(|e| format!("Failed to create percentage incline chart: {e}"))?;

    chart
        .configure_mesh()
        .draw()
        .map_err(|e| format!("Failed to draw percentage incline chart: {e}"))?;

    let mut personal_col = RGBAColor::from(RED);
    personal_col.3 = 0.5;
    let mut left_col = RGBAColor::from(BLUE);
    left_col.3 = 0.5;
    chart
        .draw_series(LineSeries::new(incline_personal, personal_col))
        .map_err(|e| format!("Failed to create percentage incline personal series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], personal_col));
    chart
        .draw_series(LineSeries::new(incline_left, left_col))
        .map_err(|e| format!("Failed to create percentage incline money left series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], left_col));
    Ok(())
}

pub fn draw_absolute_plots<DB: DrawingBackend>(
    drawing_area: &DrawingArea<DB, Shift>,
    calculation_range: &CalculationRange,
    data: &BaseCalculationData,
    opts: PlotOpts,
) -> Result<(), String> {
    let data = run_calculation_range(
        calculation_range,
        data.birth_year,
        data.church_member,
        &data.municipal_tax_specification,
        &data.base_deductions,
    )
    .map_err(|e| format!("Failed to run absolute calculation: {e}"))?;
    // Assuming sorted and tax is strictly increasing (which it is at the moment at least)
    let max = data
        .taxes
        .last()
        .map(skatt_lib::calculate::GrossTaxCalculation::total_tax_monthly);
    let mut chart = ChartBuilder::on(drawing_area)
        .caption("Skatt SEK", ("sans-serif", 14).into_font())
        .margin(5)
        .x_label_area_size(30)
        .margin_right(30)
        .y_label_area_size(80)
        .build_cartesian_2d(
            calculation_range.f32_range(),
            0f32..max.unwrap_or(100_000f32),
        )
        .map_err(|e| format!("Failed to create absolute calculation chart: {e}"))?;

    chart
        .configure_mesh()
        .draw()
        .map_err(|e| format!("Failed to draw absolute calculation chart: {e}"))?;

    let mut pts = Vec::with_capacity(data.taxes.len());
    let mut pts_tot = Vec::with_capacity(data.taxes.len());
    let mut money_left = Vec::with_capacity(data.taxes.len());
    if opts.total || opts.personal || opts.left {
        for tax in &data.taxes {
            if opts.personal {
                pts.push((tax.gross_monthly(), tax.personal_tax_monthly()));
            }
            if opts.total {
                pts_tot.push((tax.gross_monthly(), tax.total_tax_monthly()));
            }
            if opts.left {
                money_left.push((
                    tax.gross_monthly(),
                    tax.gross_monthly() - tax.personal_tax_monthly(),
                ));
            }
        }
    }

    let mut personal_col = RGBAColor::from(RED);
    personal_col.3 = 0.5;
    let mut total_col = RGBAColor::from(PURPLE);
    total_col.3 = 0.5;
    let mut left_col = RGBAColor::from(BLUE);
    left_col.3 = 0.5;
    chart
        .draw_series(AreaSeries::new(pts, 0.0, personal_col))
        .map_err(|e| format!("Failed to draw absolute calculation personal series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], personal_col));
    chart
        .draw_series(AreaSeries::new(pts_tot, 0.0, total_col))
        .map_err(|e| format!("Failed to draw absolute calculation total series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], total_col));
    chart
        .draw_series(AreaSeries::new(money_left, 0.0, left_col))
        .map_err(|e| format!("Failed to draw absolute calculation money left series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], left_col));
    Ok(())
}

pub fn draw_absolute_incline<DB: DrawingBackend>(
    drawing_area: &DrawingArea<DB, Shift>,
    calculation_range: &CalculationRange,
    data: &BaseCalculationData,
    opts: InclinePlotOpts,
) -> Result<(), String> {
    let data = run_calculation_range(
        calculation_range,
        data.birth_year,
        data.church_member,
        &data.municipal_tax_specification,
        &data.base_deductions,
    )
    .map_err(|e| format!("Failed to run absolute incline calculation: {e}"))?;
    let mut personal = None;
    let mut money_left = None;
    let mut incline_personal = Vec::with_capacity(data.taxes.len());
    let mut incline_left = Vec::with_capacity(data.taxes.len());
    let mut max_incline = 0.0f32;
    let mut min_incline = 0.0f32;
    if opts.personal || opts.left {
        for tax in &data.taxes {
            if opts.personal {
                let y2 = tax.personal_tax_monthly();
                if let Some((x1, y1)) = personal {
                    let incline = calculate_incline(y1, y2, data.step);
                    if incline > max_incline {
                        max_incline = incline;
                    }
                    if incline < min_incline {
                        min_incline = incline;
                    }
                    incline_personal.push((x1, incline));
                }
                personal = Some((tax.gross_monthly(), y2));
            }
            if opts.left {
                let y2 = tax.gross_monthly() - tax.personal_tax_monthly();
                if let Some((x1, y1)) = money_left {
                    let incline = calculate_incline(y1, y2, data.step);
                    if incline > max_incline {
                        max_incline = incline;
                    }
                    if incline < min_incline {
                        min_incline = incline;
                    }
                    incline_left.push((x1, incline));
                }
                money_left = Some((tax.gross_monthly(), y2));
            }
        }
    }

    let mut chart = ChartBuilder::on(drawing_area)
        .caption(
            "Skatt ökning krona per krona tjänad över krona tjänad (marginalskatt)",
            ("sans-serif", 14).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .margin_right(30)
        .y_label_area_size(50)
        .build_cartesian_2d(calculation_range.f32_range(), min_incline..max_incline)
        .map_err(|e| format!("Failed to create absolute incline chart: {e}"))?;

    chart
        .configure_mesh()
        .draw()
        .map_err(|e| format!("Failed to draw absolute incline chart: {e}"))?;

    let mut personal_col = RGBAColor::from(RED);
    personal_col.3 = 0.5;
    let mut left_col = RGBAColor::from(BLUE);
    left_col.3 = 0.5;
    chart
        .draw_series(LineSeries::new(incline_personal, personal_col))
        .map_err(|e| format!("Failed to draw personal absolute incline series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], personal_col));
    chart
        .draw_series(LineSeries::new(incline_left, left_col))
        .map_err(|e| format!("Failed to draw money left absolute incline series: {e}"))?
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], left_col));
    Ok(())
}
