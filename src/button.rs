use maud::{html, Markup};

#[derive(Debug, Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

pub struct Button<'a> {
    variant: ButtonVariant,
    size: ButtonSize,
    hx_get: Option<&'a str>,
    disabled: bool,
    class: Option<&'a str>,
}

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Default,
            hx_get: None,
            disabled: false,
            class: None,
        }
    }
}

impl<'a> Button<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn outline() -> Self {
        Self::default().variant(ButtonVariant::Outline)
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn hx_get(mut self, hx_get: &'a str) -> Self {
        self.hx_get = Some(hx_get);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    pub fn build(self, content: Markup) -> Markup {
        button(self, content)
    }
}

fn c(input: &'static str) -> &'static str {
    input
}

fn button(props: Button, content: Markup) -> Markup {
    let base_classes = "inline-flex items-center justify-center rounded-sm text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

    let variant_classes = match props.variant {
        ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Outline => {
            c("border border-input text-foreground bg-background hover:bg-accent hover:text-accent-foreground")
        }
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Destructive => {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90"
        }
    };

    let size_classes = match props.size {
        ButtonSize::Default => "h-10 px-4 py-2",
        ButtonSize::Sm => "h-9 rounded-sm px-3",
        ButtonSize::Lg => "h-11 rounded-sm px-8",
        ButtonSize::Icon => "h-10 w-10",
    };

    let mut classes = format!("{} {} {}", base_classes, variant_classes, size_classes);
    if let Some(additional_class) = props.class {
        classes.push_str(&format!(" {}", additional_class));
    }

    html! {
        button
            class=(classes)
            hx-get=[props.hx_get]
            disabled[props.disabled]
        {
            (content)
        }
    }
}
