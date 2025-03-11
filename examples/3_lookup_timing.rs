use anyhow::Context as _;
use liberty_db::{timing::TimingType, DefaultCtx, Library};

const TEMPLATE: &str = include_str!("../dev/tech/tsmc22/tcbn22ullbwp30p140tt0p8v25c.lib");
fn main() -> anyhow::Result<()> {
  simple_logger::SimpleLogger::new().init().unwrap();
  let library = Library::<DefaultCtx>::parse_lib(TEMPLATE)?;
  let cell_dff = library.cell.get("DFCNQD1BWP30P140").context("Failed to get cell")?;
  let pin_d = cell_dff.pin.get("D".into()).context("Failed to get pin D")?;
  let timing = pin_d
    .timing
    .get(
      "CP".into(),
      None,
      Some(&TimingType::SETUP_RISING),
      Some(&cell_dff.parse_logic_booleanexpr("CDN")?),
    )
    .context("Failed to get timing")?;
  let setup_table =
    timing.rise_constraint.as_ref().context("Failed to get setup_table")?;
  // table lookup
  dbg!(setup_table.lookup(&0.1, &0.3));
  // table_template
  #[cfg(feature = "table_template")]
  {
    use liberty_db::Group;
    let table_template = setup_table
      .extra_ctx
      .table_template
      .as_ref()
      .context("Failed to get table_template")?;
    println!("table_template {}", table_template.display().to_string());
  }
  Ok(())
}
