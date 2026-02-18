use crate::bbn_date_range;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(name = "month", help = "YYYY-MM")]
    pub month: String,
    #[arg(long = "week-date", help = "Prints the date range as week date")]
    pub week_date: bool,
}

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        date_range(self.month, self.week_date)
    }
}

fn date_range(month: String, week_date: bool) -> anyhow::Result<()> {
    Ok(bbn_date_range(month, week_date)?)
}
