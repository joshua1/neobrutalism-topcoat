// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::label::label;

/// One field of a form: the id its parts share and its validation error.
///
/// This stands in for Octane's `Form`, `FormField` and `useFormField`, which
/// share field state through react-hook-form's context. Topcoat forms are
/// server-first instead: a real `<form>` that the browser submits, which the
/// server validates and answers by rendering the page again with each
/// field's error. Nothing is shared implicitly, so build a `FormField` for
/// each field from those errors and pass it to the field's parts:
///
/// - [`form_label`] points its `for` at the control and sets `data-error`.
/// - [`FormField::control`] gives the control (an input, textarea, select,
///   checkbox, switch or radio group) its `id`, `aria-describedby` and
///   `aria-invalid`. It is Octane's `FormControl`.
/// - [`form_description`] and [`form_message`] get the ids that
///   `aria-describedby` names, and the message shows the error.
///
/// Give the control its `name` and current `value` yourself, and its native
/// constraints (`required`, `minlength`, `maxlength`, `pattern`,
/// `type="email"`): the browser checks those before it submits and shows
/// its own message. Validate again on the server, since requests can skip
/// the browser, and for the rules HTML cannot express. Put `novalidate` on
/// the `<form>` to always show the server's messages instead.
///
/// The recommended pattern is one page that renders the form on `GET` and
/// validates it on `POST`: on success it saves and redirects (so a reload
/// does not submit again), otherwise it renders the form again with the
/// submitted values, the errors and a `422` status.
///
/// ```ignore
/// use std::collections::HashMap;
///
/// use topcoat::{
///     Result,
///     context::Cx,
///     router::{StatusCode, content::Form, error::see_other, page},
///     view::{View, attributes, view},
/// };
///
/// // `Form<T>` deserializes with serde, so the app depends on it:
/// // serde = { version = "1", features = ["derive"] }
/// #[derive(Default, serde::Deserialize)]
/// #[serde(default)]
/// struct Profile {
///     username: String,
/// }
///
/// /// Field name to error message; empty when the profile is valid.
/// fn validate(profile: &Profile) -> HashMap<&'static str, &'static str> {
///     let mut errors = HashMap::new();
///     if profile.username.chars().count() < 2 {
///         errors.insert("username", "Username must be at least 2 characters.");
///     }
///     errors
/// }
///
/// #[page([GET, POST])]
/// async fn edit_profile(cx: &Cx, form: Option<Form<Profile>>) -> Result<impl View> {
///     let submitted = form.is_some();
///     let profile = form.map(|Form(profile)| profile).unwrap_or_default();
///     let errors = if submitted { validate(&profile) } else { HashMap::new() };
///     if submitted && errors.is_empty() {
///         // Save the profile, then send the browser on with a GET.
///         return Err(see_other("/").into());
///     }
///
///     let username = FormField::new("username", errors.get("username").copied());
///     Ok(view! {
///         if !errors.is_empty() {
///             (StatusCode::UNPROCESSABLE_ENTITY)
///         }
///         <form method="post" class="space-y-8">
///             form_item(
///                 form_label(field: username, "Username")
///                 input(attrs: attributes! {
///                     name="username"
///                     value=(profile.username.as_str())
///                     required=""
///                     minlength="2"
///                     (username.control(cx))
///                 })
///                 form_description(field: username, "This is your public display name.")
///                 form_message(field: username)
///             )
///             button(attrs: attributes! { type="submit" }, "Submit")
///         </form>
///     })
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormField<'a> {
    /// The base of the parts' element ids. Make it unique on the page.
    pub id: &'a str,
    /// The field's validation message, or `None` while it is valid.
    pub error: Option<&'a str>,
}

#[allow(dead_code)]
impl<'a> FormField<'a> {
    /// A field whose parts derive their ids from `id`, showing `error` if
    /// there is one.
    #[must_use]
    pub const fn new(id: &'a str, error: Option<&'a str>) -> Self {
        Self { id, error }
    }

    /// The control's id, which the [`form_label`] points at.
    #[must_use]
    pub fn item_id(&self) -> String {
        format!("{}-form-item", self.id)
    }

    /// The id of the field's [`form_description`].
    #[must_use]
    pub fn description_id(&self) -> String {
        format!("{}-form-item-description", self.id)
    }

    /// The id of the field's [`form_message`].
    #[must_use]
    pub fn message_id(&self) -> String {
        format!("{}-form-item-message", self.id)
    }

    /// The control's `aria-describedby`: the description, and the message
    /// too while there is an error.
    #[must_use]
    pub fn described_by(&self) -> String {
        match self.error {
            Some(_) => format!("{} {}", self.description_id(), self.message_id()),
            None => self.description_id(),
        }
    }

    /// The `aria-invalid` and `data-error` value: `"true"` while there is an
    /// error. These are enumerated attributes, where an empty value would
    /// mean `false`, so it is spelled out.
    #[must_use]
    pub const fn invalid(&self) -> &'static str {
        if self.error.is_some() {
            "true"
        } else {
            "false"
        }
    }

    /// Attributes for the field's control, Octane's `FormControl`: its `id`,
    /// `aria-describedby` and `aria-invalid`.
    ///
    /// Spread them last into the control's `attrs`, after its `name` and
    /// `value`:
    ///
    /// ```ignore
    /// input(attrs: attributes! { name="username" (username.control(cx)) })
    /// ```
    #[must_use]
    pub fn control(&self, cx: &Cx) -> Attributes {
        attributes! { cx =>
            id=(self.item_id())
            aria-describedby=(self.described_by())
            aria-invalid=(self.invalid())
        }
    }
}

/// A field's label, control, description and message, stacked in a grid.
///
/// `attrs` are forwarded to the `<div>`, with extra classes appended to its
/// classes. A checkbox, radio or switch field nests one `form_item` per
/// option, with a flex layout (`flex!`, since the item's `grid` would
/// otherwise win).
///
/// ```ignore
/// view! {
///     form_item(
///         form_label(field: username, "Username")
///         input(attrs: attributes! { name="username" placeholder="ekmas" (username.control(cx)) })
///         form_description(field: username, "This is your public display name.")
///         form_message(field: username)
///     )
/// }
/// ```
#[component]
pub async fn form_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="form-item" class=(class!("grid gap-2", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// A [`label`] for a field's control.
///
/// Its `for` points at the control given [`FormField::control`], and
/// `data-error` is `"true"` while the field has an error. A `for` in `attrs`
/// replaces the default one.
#[component]
pub async fn form_label(
    /// The field this label names.
    field: FormField<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        label(
            attrs: attributes! {
                data-slot="form-label"
                data-error=(field.invalid())
                for=(field.item_id())
                class=(class!("font-heading", attrs.remove("class")))
                (attrs)
            },
            (child)
        )
    })
}

/// Supporting text for a field, named by its control's `aria-describedby`.
#[component]
pub async fn form_description(
    /// The field this text describes.
    field: FormField<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="form-description"
            id=(field.description_id())
            class=(class!("text-sm font-base text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}

/// A field's validation message, named by its control's `aria-describedby`
/// while the field has an error.
///
/// It shows the field's `error`, or the children when there is none. With
/// neither, the paragraph is empty and `empty:hidden` removes it, where
/// Octane renders nothing.
#[component]
pub async fn form_message(
    /// The field whose error this shows.
    field: FormField<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="form-message"
            id=(field.message_id())
            class=(class!("text-sm font-base text-red-500 empty:hidden", attrs.remove("class")))
            (attrs)
        >
            if let Some(error) = field.error {
                (error)
            } else {
                (child)
            }
        </p>
    })
}
