use lettre::{
    message::{Attachment, Body, Mailbox, MultiPart, SinglePart},
    Message, Transport,
};

use crate::{make_message_body, Context, Error, Party};

pub(crate) mod template;

pub(crate) struct Mailer {
    context: Context,
    image_body: Body,
    attachments: Vec<SinglePart>,
    from: Mailbox,
}

impl Mailer {
    pub fn new(
        context: Context,
        image_path: Option<String>,
        attachments: Option<Vec<String>>,
        from: String,
    ) -> Result<Self, Error> {
        let image_body = image_path
            .map(std::fs::read)
            .transpose()?
            .map(Body::new)
            .ok_or(Error::Image)?;

        let attachments = attachments
            .map(|attachments| {
                attachments
                    .iter()
                    .map(|file| {
                        let body = Body::new(std::fs::read(file)?);
                        Ok(Attachment::new(file.clone()).body(body, "application/pdf".parse()?))
                    })
                    .collect::<Result<Vec<SinglePart>, Error>>()
            })
            .transpose()?
            .ok_or(Error::Image)?;

        Ok(Self {
            context,
            image_body,
            attachments,
            from: from.parse()?,
        })
    }

    pub fn send_email(&self, party: &Party) -> Result<(), Error> {
        // todo(steve): write custom deserializer or debug...
        // support comma separated list of mailboxes
        // for some reason these were not deserializing into a vector
        let mailboxes = party
            .email
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|email| {
                let string = format!("{} <{}>", party.name, email);
                string.parse().unwrap()
            })
            .collect::<Vec<Mailbox>>();

        let mut email = Message::builder()
            .from(self.from.clone())
            .reply_to(self.from.clone())
            .subject("Thanks for attending our wedding!!!");

        for mailbox in mailboxes {
            email = email.to(mailbox)
        }

        let mut parts = MultiPart::related()
            .singlepart(SinglePart::html(make_message_body(party)))
            .singlepart(
                Attachment::new_inline(String::from("honeymoon"))
                    .body(self.image_body.clone(), "image/jpeg".parse()?),
            );

        for attachment in self.attachments.iter() {
            parts = parts.singlepart(attachment.clone());
        }

        let email = email.multipart(parts)?;

        self.context.mailer.send(&email)?;

        Ok(())
    }
}
