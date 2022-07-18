#[macro_use]
extern crate log;

use std::env;

use actix_cors::Cors;
use actix_web::{App, http, HttpResponse, HttpServer, middleware, web};
use anyhow::Result;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
	dotenv().ok();
	env_logger::init();
	
	let host = env::var("HOST").expect("HOST is not set in .env file");
	let port = env::var("PORT")
			.expect("PORT is not set in .env file")
			.parse::<u16>()
			.expect("PORT should be a u16");
	
	let server = HttpServer::new(move || {
		let cors = Cors::default()
				.allowed_origin("https://localhost:5001")
				.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
				.allowed_header(http::header::CONTENT_TYPE)
				.max_age(3600);
		
		App::new()
				.wrap(cors)
				.wrap(middleware::Logger::default())
				.route("/Ping", web::get().to(|| HttpResponse::Ok()))
	}).bind((host, port))?;
	
	info!("Starting server");
	server.run().await?;
	
	Ok(())
}