use tide::{Result, Request, Response};
use askama::Template;
use chrono::{Local, Datelike, Weekday, Month};

#[derive(Template)]
#[template(path = "index.html")]
struct FrontpageCtx {
    // Is it thursday
    thursday: bool,
    // Is it Saint Patrick's day
    patrick: bool,
}

#[async_std::main]
async fn main() -> tide::Result<()>{
    tide::log::with_level(tide::log::LevelFilter::Info);

    let mut app = tide::new();

    app.at("/").get(frontpage);

    app.at("/imgs").serve_dir("./imgs")?;

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn frontpage(mut _req: Request<()>) -> Result {
    let now = Local::now();

    let templ = FrontpageCtx {
        thursday: now.weekday() == Weekday::Thu,
        patrick: now.month() == Month::March.number_from_month() && now.day() == 17,
    };

    let mut resp = Response::new(200);
    resp.set_content_type("text/html");
    resp.set_body(templ.render()?);
    Ok(resp)
}