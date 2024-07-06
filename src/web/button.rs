use maud::{html, Markup};

pub enum ButtonVariant {
    Default,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

pub struct Button<'a> {
    variant: ButtonVariant,
    size: ButtonSize,
    ty: Option<&'a str>,
    hx_get: Option<&'a str>,
    hx_post: Option<&'a str>,
    hx_swap: Option<&'a str>,
    disabled: bool,
    class: Option<&'a str>,
    aria_label: Option<&'a str>,
    title: Option<&'a str>,
    id: Option<&'a str>,
}

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            ty: None,
            hx_get: None,
            hx_post: None,
            hx_swap: None,
            disabled: false,
            class: None,
            aria_label: None,
            title: None,
            id: None,
        }
    }
}

impl<'a> Button<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn secondary() -> Self {
        Self::default().variant(ButtonVariant::Secondary)
    }

    pub fn outline() -> Self {
        Self::default().variant(ButtonVariant::Outline)
    }

    pub fn ghost() -> Self {
        Self::default().variant(ButtonVariant::Ghost)
    }

    pub fn destructive() -> Self {
        Self::default().variant(ButtonVariant::Destructive)
    }

    pub fn link() -> Self {
        Self::default().variant(ButtonVariant::Link)
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn ty(mut self, ty: &'a str) -> Self {
        self.ty = Some(ty);
        self
    }

    pub fn hx_get(mut self, hx_get: &'a str) -> Self {
        self.hx_get = Some(hx_get);
        self
    }

    pub fn hx_post(mut self, hx_post: &'a str) -> Self {
        self.hx_post = Some(hx_post);
        self
    }

    pub fn hx_swap(mut self, hx_swap: &'a str) -> Self {
        self.hx_post = Some(hx_swap);
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

    pub fn aria_label(mut self, aria_label: &'a str) -> Self {
        self.aria_label = Some(aria_label);
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn build(self, content: Markup) -> Markup {
        button(self, content)
    }
}

fn button(props: Button, content: Markup) -> Markup {
    let base_classes = "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

    let variant_classes = match props.variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90 dark:bg-primary dark:text-primary-foreground dark:hover:bg-primary/90",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80 dark:bg-secondary dark:text-secondary-foreground dark:hover:bg-secondary/80",
        ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground dark:border-input dark:bg-background dark:text-foreground dark:hover:bg-accent dark:hover:text-accent-foreground",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground dark:text-foreground dark:hover:bg-accent dark:hover:text-accent-foreground",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90 dark:bg-destructive dark:text-destructive-foreground dark:hover:bg-destructive/90",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline dark:text-primary",
    };

    let size_classes = match props.size {
        ButtonSize::Default => "h-10 px-4 py-2",
        ButtonSize::Sm => "h-9 rounded-md px-3",
        ButtonSize::Lg => "h-11 rounded-md px-8",
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
            hx-post=[props.hx_post]
            hx-swap=[props.hx_swap]
            disabled[props.disabled]
            aria-label=[props.aria_label]
            title=[props.title]
            id=[props.id]
            type=[props.ty]
        {
            (content)
        }
    }
}
