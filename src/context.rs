use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

pub(crate) struct Context {
    pub mailer: SmtpTransport,
}

impl Context {
    pub fn new(
        username: String,
        password: String,
        server: String,
    ) -> Result<Self, crate::error::Error> {
        let creds = Credentials::new(username, password);

        Ok(Self {
            mailer: SmtpTransport::relay(server.as_str())?
                .credentials(creds)
                .build(),
        })
    }
}
