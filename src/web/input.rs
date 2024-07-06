use maud::{html, Markup};

#[derive(Debug, Clone, Copy)]
pub enum InputSize {
    Default,
    Sm,
    Lg,
}

pub struct Input<'a> {
    size: InputSize,
    placeholder: Option<&'a str>,
    value: Option<&'a str>,
    ty: Option<&'a str>,
    id: Option<&'a str>,
    name: Option<&'a str>,
    class: Option<&'a str>,
    disabled: bool,
    required: bool,
}

impl<'a> Default for Input<'a> {
    fn default() -> Self {
        Self {
            size: InputSize::Default,
            placeholder: None,
            value: None,
            id: None,
            name: None,
            class: None,
            ty: None,
            disabled: false,
            required: false,
        }
    }
}

impl<'a> Input<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub fn ty(mut self, value: &'a str) -> Self {
        self.ty = Some(value);
        self
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn build(self) -> Markup {
        input(self)
    }
}

fn input(props: Input) -> Markup {
    let base_classes = "flex w-full rounded-md border border-input bg-background text-foreground px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

    let size_classes = match props.size {
        InputSize::Default => "h-10",
        InputSize::Sm => "h-9",
        InputSize::Lg => "h-11",
    };

    let mut classes = format!("{} {}", base_classes, size_classes);
    if let Some(additional_class) = props.class {
        classes.push_str(&format!(" {}", additional_class));
    }

    html! {
        input
            class=(classes)
            type=(props.ty.unwrap_or("text"))
            placeholder=[props.placeholder]
            value=[props.value]
            id=[props.id]
            name=[props.name]
            spellcheck="false"
            disabled[props.disabled]
            required[props.required] {}
    }
}
