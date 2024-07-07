use maud::{html, Markup};

#[derive(Default)]
pub struct FileUploader<'a> {
    id: Option<&'a str>,
    name: Option<&'a str>,
    class: Option<&'a str>,
    accept: Option<&'a str>,
    multiple: bool,
    disabled: bool,
    required: bool,
}

impl<'a> FileUploader<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    pub fn accept(mut self, accept: &'a str) -> Self {
        self.accept = Some(accept);
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn build(self) -> Markup {
        file_upload(self)
    }
}

fn file_upload(props: FileUploader) -> Markup {
    let base_classes = "cursor-pointer block w-full text-sm text-foreground file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-primary file:text-primary-foreground hover:file:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 dark:file:bg-primary dark:file:text-primary-foreground dark:hover:file:bg-primary/90";

    let mut classes = base_classes.to_string();
    if let Some(additional_class) = props.class {
        classes.push_str(&format!(" {}", additional_class));
    }

    html! {
        input
            class=(classes)
            type="file"
            id=[props.id]
            name=[props.name]
            accept=[props.accept]
            multiple[props.multiple]
            disabled[props.disabled]
            required[props.required] {}
    }
}
