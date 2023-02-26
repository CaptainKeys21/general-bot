use mongodb::Client;

pub struct Mongodb {
    pub client: Client,
}

impl Mongodb {
    pub async fn new() -> Mongodb {
        let new_client = Client::with_uri_str("mongodb://GENERAL_BOT:general%40bot@logger-db:27017").await.expect("Erro ao conectar na base de dados");

        Mongodb {
            client: new_client
        }
    }
}
