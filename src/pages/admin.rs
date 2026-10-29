use resuma::prelude::*;

use crate::stats::grouped;

pub fn page(req: FlowRequest) -> View {
    set_page_title("Admin | PDFForge");
    set_page_description("Private PDFForge statistics.");
    set_page_robots("noindex, nofollow");
    set_page_lang("es");
    if !crate::site::request_is_admin() {
        return login_view(req.query_param("e").is_some());
    }
    dash_view()
}

fn login_view(failed: bool) -> View {
    let notice = if failed {
        view! {
            <p class="hint admin-error" role="alert">
                "Ese usuario o clave no coincide."
            </p>
        }
    } else {
        view! { <span hidden=""></span> }
    };
    view! {
        <main class="content-section admin-page">
            <p class="eyebrow">"PDFForge"</p>
            <h1>"Admin"</h1>
            <p class="hero-lead">
                "Los trabajos terminados de cada herramienta están en esta página."
            </p>
            {notice}
            <Form submit={crate::actions::admin_login} class="hero-form admin-login">
                <label>
                    "Usuario"
                    <input name="user" type="text" autocomplete="username" required=true />
                </label>
                <label>
                    "Clave"
                    <input name="password" type="password" autocomplete="current-password" required=true />
                </label>
                <button type="submit" class="btn btn-primary">"Entrar"</button>
            </Form>
        </main>
    }
}

fn dash_view() -> View {
    let activity = crate::stats::snapshot();
    view! {
        <main class="content-section admin-page">
            <div class="admin-head">
                <div>
                    <p class="eyebrow">"PDFForge"</p>
                    <h1>"Estadísticas"</h1>
                </div>
                <Form submit={crate::actions::admin_logout}>
                    <input type="hidden" name="ok" value="1" />
                    <button type="submit" class="btn">"Cerrar sesión"</button>
                </Form>
            </div>
            <div class="admin-grid">
                {stat_card("Comprimir", grouped(activity.compress))}
                {stat_card("Unir", grouped(activity.merge))}
                {stat_card("Dividir", grouped(activity.split))}
                {stat_card("JPG a PDF", grouped(activity.from_jpg))}
                {stat_card("PDF a JPG", grouped(activity.to_jpg))}
                {stat_card("Texto", grouped(activity.to_text))}
            </div>
            <p class="hint">
                "Cada número es un archivo que terminó de procesarse. No se publica."
            </p>
        </main>
    }
}

fn stat_card(label: &'static str, value: String) -> View {
    view! {
        <article class="admin-stat">
            <p class="admin-stat-value">{value}</p>
            <p class="hint">{label}</p>
        </article>
    }
}
