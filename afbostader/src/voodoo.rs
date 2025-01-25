use fantoccini::Locator;
use secrecy::ExposeSecret;

use crate::Credentials;

pub struct VoodooState {}

pub struct Voodoo {
    c: fantoccini::Client,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Fantoccini(#[from] fantoccini::error::CmdError),
}

impl Voodoo {
    pub fn new(fantoccini: fantoccini::Client) -> Self {
        Self { c: fantoccini }
    }

    pub async fn login(&mut self, credentials: &Credentials) -> Result<(), Error> {
        self.c.goto("https://www.afbostader.se/dina/sidor/").await?;

        self.c
            .find(Locator::Id("UserName"))
            .await?
            .send_keys(&credentials.email)
            .await?;
        self.c
            .find(Locator::Id("Password"))
            .await?
            .send_keys(&credentials.password.expose_secret())
            .await?;
        self.c
            .find(Locator::Id("ctl29_LoginControl_LoginButton"))
            .await?
            .click()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio_macros::test]
    async fn test_login() {
        let mut voodoo = Voodoo::new(
            fantoccini::ClientBuilder::native()
                .connect("http://localhost:4444")
                .await
                .unwrap(),
        );
        voodoo
            .login(&Credentials {
                email: "ake@amcoff.net".into(),
                password: "$q$JQNSEi!K74Ebm".to_string().into(),
            })
            .await
            .unwrap();
    }
}
