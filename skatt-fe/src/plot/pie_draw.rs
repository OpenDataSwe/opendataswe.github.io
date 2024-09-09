use crate::app::BaseCalculationData;
use plotters::backend::DrawingBackend;
use plotters::element::Drawable;
use plotters::style::full_palette::{ORANGE, PURPLE};
use plotters::style::{BLUE, GREEN, RED, YELLOW};
use skatt_lib::calculate::calculate_single;
use skatt_lib::number::MaxNumberValue;

macro_rules! push_if_positive {
    ($values: expr, $colors: expr, $labels: expr, $v: expr, $c: expr, $l: expr) => {{
        if $v > 0.0 {
            $values.push($v);
            $colors.push($c);
            $labels.push($l);
        }
    }};
}

pub fn draw_pie_plot<DB: DrawingBackend>(
    backend: &mut DB,
    base: &BaseCalculationData,
    salary: MaxNumberValue,
) -> Result<(), String> {
    let tax = calculate_single(
        salary,
        base.birth_year,
        base.church_member,
        &base.municipal_tax_specification,
        &base.base_deductions,
    )
    .map_err(|e| format!("Failed to calculate pie: {e}"))?;
    let it = [].into_iter();
    // Todo: Smallvec maybe
    let (mut values, mut colors, mut labels) = (
        Vec::with_capacity(6),
        Vec::with_capacity(6),
        Vec::with_capacity(6),
    );
    push_if_positive!(
        values,
        colors,
        labels,
        f64::from(tax.employer_fee),
        RED,
        format!(
            "Arbetsgivaravgift {:.0} ({:.2}%)",
            tax.employer_fee / 12.,
            tax.employer_fee / tax.total_tax() * 100.
        )
    );
    push_if_positive!(
        values,
        colors,
        labels,
        f64::from(tax.church_fee),
        PURPLE,
        format!(
            "Kyrkoavgift {:.0} ({:.2}%)",
            tax.church_fee / 12.,
            tax.church_fee / tax.total_tax() * 100.
        )
    );
    push_if_positive!(
        values,
        colors,
        labels,
        f64::from(tax.burial_fee),
        BLUE,
        format!(
            "Begravningsavgift {:.0} ({:.2}%)",
            tax.burial_fee / 12.,
            tax.burial_fee / tax.total_tax() * 100.
        )
    );
    push_if_positive!(
        values,
        colors,
        labels,
        f64::from(tax.regional_tax),
        GREEN,
        format!(
            "Landstingsskatt {:.0} ({:.2}%)",
            tax.regional_tax / 12.,
            tax.regional_tax / tax.total_tax() * 100.
        )
    );
    push_if_positive!(
        values,
        colors,
        labels,
        f64::from(tax.municipal_tax),
        YELLOW,
        format!(
            "Kommunal skatt {:.0} ({:.2}%)",
            tax.municipal_tax / 12.,
            tax.municipal_tax / tax.total_tax() * 100.
        )
    );
    push_if_positive!(
        values,
        colors,
        labels,
        f64::from(tax.state_tax),
        ORANGE,
        format!(
            "Statlig inkomstskatt {:.0} ({:.2}%)",
            tax.state_tax / 12.,
            tax.state_tax / tax.total_tax() * 100.
        )
    );
    plotters::element::Pie::new(&(400, 300), &200., &values, &colors, &labels)
        .draw(it, backend, (0, 0))
        .map_err(|e| format!("Failed to create pie element: {e}"))
}
