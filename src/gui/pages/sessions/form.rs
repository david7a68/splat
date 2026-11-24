//! The New session form, and Session details for a session that exists (`Sessions::show_form`):
//! Number, Session title, Date and, for a new session, Template; Create and Cancel; then the
//! reason Create waits or the file it makes. Enter in a field creates; Escape cancels.
use crate::gui::{SplatRoot, kit, theme};
use crate::sessions::page::gpui_facade::FormField;
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, KeyBinding, ParentElement, Styled,
    Window, actions, div,
};
use theme::{FORM_WIDTH, L, S, Type, colors};

actions!(session_form, [CancelForm]);

pub(super) fn init(cx: &mut gpui::App) {
    cx.bind_keys([KeyBinding::new("escape", CancelForm, Some("SessionForm"))]);
}

fn set(field: FormField) -> impl Fn(&mut SplatRoot, &str, &mut Window, &mut Context<SplatRoot>) {
    move |root, words, _, cx| {
        root.app.sessions.gv_form_set(field, words);
        cx.notify();
    }
}

pub(super) fn render(
    root: &mut SplatRoot,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let Some(form) = root.app.sessions.gv_form() else {
        return div().into_any_element();
    };
    // Each field's id is the session it edits, so its typing history is that form's.
    let key = if form.editing {
        root.app
            .sessions
            .gv_selected()
            .map_or_else(String::new, |id| id.to_string())
    } else {
        "new".to_owned()
    };
    let id = |name: &str| gpui::ElementId::Name(format!("session-form-{name}-{key}").into());
    let entity = cx.entity().downgrade();
    let on_enter = move |_: &mut Window, cx: &mut gpui::App| {
        let _ = entity.update(cx, |root, cx| {
            root.app.sessions.gv_form_create();
            cx.notify();
        });
    };
    let heading = if form.editing {
        "Session details"
    } else {
        "New session"
    };
    let number = kit::TextField::new(id("number"), form.number.clone())
        .name("Number")
        .width(L * 5.0)
        .on_change(cx.listener(set(FormField::Number)))
        .on_enter(on_enter.clone());
    let title = kit::TextField::new(id("title"), form.title.clone())
        .name("Session title")
        .focus(form.focus_title)
        .on_change(cx.listener(set(FormField::Title)))
        .on_enter(on_enter.clone());
    let date = kit::TextField::new(id("date"), form.date.clone())
        .name("Date")
        .on_change(cx.listener(set(FormField::Date)))
        .on_enter(on_enter);
    let templates = root.app.sessions.gv_templates();
    let template = (!form.editing && !templates.is_empty()).then(|| {
        let names: Vec<String> = templates.iter().map(|t| t.name.clone()).collect();
        let chosen = names.iter().position(|name| *name == form.template);
        kit::Dropdown::new(
            "session-template",
            "Session template",
            names.clone(),
            chosen,
        )
        .on_select(cx.listener(move |root, at: &usize, _, cx| {
            if let Some(name) = names.get(*at) {
                root.app.sessions.gv_form_set(FormField::Template, name);
                cx.notify();
            }
        }))
    });
    let word = if form.editing {
        "Save details"
    } else {
        "Create session"
    };
    let create = if form.primary {
        kit::Button::primary("session-create", word)
    } else {
        kit::Button::secondary("session-create", word)
    }
    .blocked(form.reason)
    .on_click(cx.listener(|root, _, _, cx| {
        root.app.sessions.gv_form_create();
        cx.notify();
    }));
    let cancel =
        kit::Button::quiet("session-cancel", "Cancel").on_click(cx.listener(|root, _, _, cx| {
            root.app.sessions.gv_form_cancel();
            cx.notify();
        }));
    // The kept line: the reason Create waits, else the file it makes.
    let kept = form
        .reason
        .map(str::to_owned)
        .or(form.preview.clone())
        .map(|words| kit::text(Type::Small, c.muted, words));
    let error = form
        .error
        .clone()
        .map(|words| kit::Message::new(kit::Tone::Error, words));
    let fields = div()
        .flex()
        .flex_col()
        .w(FORM_WIDTH)
        .max_w_full()
        .child(kit::heading(
            kit::Heading::Group {
                title: heading,
                count: 0,
            },
            true,
        ))
        .child(kit::field("Number", None, None, number).pb(S))
        .child(
            kit::field(
                "Session title",
                Some("Optional. You can name it after you play.".into()),
                None,
                title,
            )
            .pb(S),
        )
        .child(kit::field("Date", Some("As YYYY-MM-DD.".into()), None, date).pb(S))
        .children(template.map(|template| kit::field("Template", None, None, template).pb(S)))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(create)
                .child(cancel),
        )
        .children(kept.map(|kept| kept.pt(theme::XS)))
        .children(error.map(|error| div().pt(S).child(error)));
    kit::scroll_column(
        "session-form",
        div()
            .key_context("SessionForm")
            .on_action(cx.listener(|root, _: &CancelForm, _, cx| {
                root.app.sessions.gv_form_cancel();
                cx.notify();
            }))
            .child(fields),
    )
    .into_any_element()
}
