use askama::Template;

#[derive(Template)]
#[template(path = "confirmation/email.html")]
pub struct ConfirmationEmailHtmlTemplate<'a> {
    pub token: &'a str,
}

#[derive(Template)]
#[template(path = "confirmation/email.txt")]
pub struct ConfirmationEmailTxtTemplate<'a> {
    pub token: &'a str,
}

#[derive(Template)]
#[template(path = "confirmation/subject.txt")]
pub struct ConfirmationEmailSubject {}
