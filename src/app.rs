use crate::temperature::read_temperature;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::{debug, error, info};

#[get("/")]
async fn get_temperature(data: Data<Config>) -> impl Responder {
    let temperature = read_temperature(&data.device_id);

    match temperature {
        Ok(temperature) => {
            info!("Reporting temperature: {}", temperature);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(temperature.to_string())
        }
        Err(error) => {
            error!("{}", error);
            HttpResponse::InternalServerError()
                .body("Failed to read temperature (see logs for details)")
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub device_id: String,
    pub interface: String,
    pub port: u16,
}

#[actix_web::main]
pub async fn run(config: Config) -> Result<(), std::io::Error> {
    let interface = config.interface.clone();
    let port = config.port;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .service(get_temperature)
    })
    .bind((interface.clone(), port))?;

    let server_base_url = format!("{}:{}", interface, port);
    debug!("Server starting on {}", server_base_url);
    let running_server = server.run();
    info!("Server listening on {}", server_base_url);
    running_server.await
}
