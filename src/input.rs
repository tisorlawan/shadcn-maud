use maud::{html, Markup};

struct InputProps<'a> {
    id: &'a str,
    placeholder: Option<&'a str>,
    input_type: &'a str,
    disabled: bool,
    required: bool,
}

fn input(props: InputProps) -> Markup {
    let base_classes = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

    html! {
        input
            id=(props.id)
            type=(props.input_type)
            class=(base_classes)
            placeholder=(props.placeholder.unwrap_or(""))
            disabled[props.disabled]
            required[props.required] {}
    }
}

// Example usage in a form
pub fn form_with_input() -> Markup {
    html! {
        form class="space-y-4 w-full max-w-sm" {
            div {
                label for="email" class="block text-sm font-medium text-gray-700" {
                    "Email"
                }
                div class="mt-1" {
                    (input(InputProps {
                        id: "email",
                        placeholder: Some("Enter your email"),
                        input_type: "email",
                        disabled: false,
                        required: true,
                    }))
                }
            }
            div {
                label for="password" class="block text-sm font-medium text-gray-700" {
                    "Password"
                }
                div class="mt-1" {
                    (input(InputProps {
                        id: "password",
                        placeholder: Some("Enter your password"),
                        input_type: "password",
                        disabled: false,
                        required: true,
                    }))
                }
            }
            div {
                label for="disabled-input" class="block text-sm font-medium text-gray-700" {
                    "Disabled Input"
                }
                div class="mt-1" {
                    (input(InputProps {
                        id: "disabled-input",
                        placeholder: Some("This input is disabled"),
                        input_type: "text",
                        disabled: true,
                        required: false,
                    }))
                }
            }
        }
    }
}
