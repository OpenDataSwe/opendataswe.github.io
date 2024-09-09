use crate::app::BaseCalculationData;
use crate::error::SimpleErrorDisplay;
use crate::info::Info;
use skatt_lib::calculate::calculate_single;
use skatt_lib::generated::readme::{
    ARBETSGIVARAVGIFT, BEGRAVNINGSAVGIFT, JOBBSKATTEAVDRAG, KOMMUNALSKATT, KOMMUNAL_SKATT,
    KVAR_EFTER_SKATT, KYRKOAVGIFT, LANDSTINGSSKATT_REGIONSSKATT, PUBLIC_SERVICE_AVGIFT,
    SKATTEREDUKTION_FORVARVSINKOMST, STATLIG_INKOMSTSKATT, TOTAL_PERSONLIG_SKATT,
    TOTAL_SKATTESUMMA,
};
use skatt_lib::number::MaxNumberValue;
use yew::{html, Component, Context, Html, Properties};

pub struct TaxTable;

#[derive(Properties, PartialEq, Clone)]
pub struct TaxTableProps {
    pub salary: MaxNumberValue,
    pub base_calculation_data: BaseCalculationData,
}

struct TableEntry {
    category: &'static str,
    percentage: String,
    absolute: String,
    help: &'static str,
}

impl TableEntry {
    pub fn new(category: &'static str, total: f32, gross: f32, help_html: &'static str) -> Self {
        let perc = if gross == 0.0 {
            0.0
        } else {
            (total / gross) * 100.
        };
        Self {
            category,
            percentage: format!("{perc:.2}"),
            absolute: format!("{:.2}", total / 12.0),
            help: help_html,
        }
    }
}

impl TaxTableProps {
    fn to_table_entries(&self) -> Result<[TableEntry; 13], String> {
        let tax = calculate_single(
            self.salary,
            self.base_calculation_data.birth_year,
            self.base_calculation_data.church_member,
            &self.base_calculation_data.municipal_tax_specification,
            &self.base_calculation_data.base_deductions,
        )
        .map_err(|e| format!("Failed to calculate tax for table: {e}"))?;
        Ok([
            TableEntry::new("Kyrkoavgift", tax.church_fee, tax.gross_yearly, KYRKOAVGIFT),
            TableEntry::new(
                "Begravningsavgift",
                tax.burial_fee,
                tax.gross_yearly,
                BEGRAVNINGSAVGIFT,
            ),
            TableEntry::new(
                "Landstingsskatt",
                tax.regional_tax,
                tax.gross_yearly,
                LANDSTINGSSKATT_REGIONSSKATT,
            ),
            TableEntry::new(
                "Kommunal skatt",
                tax.municipal_tax,
                tax.gross_yearly,
                KOMMUNALSKATT,
            ),
            TableEntry::new(
                "Totalt inom kommun",
                tax.total_municipal_tax(),
                tax.gross_yearly,
                KOMMUNAL_SKATT,
            ),
            TableEntry::new(
                "Statlig inkomstskatt",
                tax.state_tax,
                tax.gross_yearly,
                STATLIG_INKOMSTSKATT,
            ),
            TableEntry::new(
                "Public service-avgift",
                tax.public_service_fee,
                tax.gross_yearly,
                PUBLIC_SERVICE_AVGIFT,
            ),
            TableEntry::new(
                "Jobskatteavdrag",
                tax.work_tax_deduction,
                tax.gross_yearly,
                JOBBSKATTEAVDRAG,
            ),
            TableEntry::new(
                "Skattereduktion förvärvsinkomst",
                tax.income_tax_deduction,
                tax.gross_yearly,
                SKATTEREDUKTION_FORVARVSINKOMST,
            ),
            TableEntry::new(
                "Total personlig skatt",
                tax.personal_tax(),
                tax.gross_yearly,
                TOTAL_PERSONLIG_SKATT,
            ),
            TableEntry::new(
                "Arbetsgivaravgift",
                tax.employer_fee,
                tax.gross_yearly,
                ARBETSGIVARAVGIFT,
            ),
            TableEntry::new(
                "Total skattesumma",
                tax.total_tax(),
                tax.gross_yearly,
                TOTAL_SKATTESUMMA,
            ),
            TableEntry::new(
                "Kvar efter skatt",
                tax.gross_yearly - tax.personal_tax(),
                tax.gross_yearly,
                KVAR_EFTER_SKATT,
            ),
        ])
    }
}

impl Component for TaxTable {
    type Message = ();
    type Properties = TaxTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let table_entries = ctx.props().to_table_entries();
        match table_entries {
            Ok(table_entries) => {
                html! {
                    <table>
                        <tr>
                            <th>{"Kategori "}</th>
                            <th>{"Procent [%] "}</th>
                            <th>{"Absolut [SEK] "}</th>
                        </tr>
                        {
                            table_entries
                            .into_iter()
                            .map(|ts| {
                                html! {
                                    <tr>
                                        <td>{ts.category} <Info content={ts.help} /></td>
                                        <td>{ts.percentage}</td>
                                        <td>{ts.absolute}</td>
                                    </tr>
                                }
                            })
                            .collect::<Html>()
                        }
                    </table>
                }
            }
            Err(e) => {
                html! { <SimpleErrorDisplay message={Some(e)}/> }
            }
        }
    }
}
