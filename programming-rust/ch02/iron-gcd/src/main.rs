extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;
use iron_gcd::gcd;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Listening on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html; Charset=Utf8));
    resp.set_mut(
        r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
  <input type="text" name="n">
  <input type="text" name="n">
  <button type="submit">Compute GCD</button>
</form>
"#,
    );
    Ok(resp)
}

fn post_gcd(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(resp);
        }
        Ok(map) => map,
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("Form data has no 'n' parameter\n"));
            return Ok(resp);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                resp.set_mut(status::BadRequest);
                resp.set_mut(format!("Error parsing number: {:?}\n", unparsed));
                return Ok(resp);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html; Charset=Utf8));
    resp.set_mut(format!("GCD of {:?} is <b>{}</b>\n", numbers, d));
    Ok(resp)
}
