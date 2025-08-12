use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"Home Page"</h1>
            <p>"This is the home page - not lazy loaded"</p>
        </div>
    }
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <div>
            <h1>"Admin Dashboard"</h1>
            <p>"Heavy admin page with lots of components"</p>
            <p>"Admin content here..."</p>
            <p>"More admin content..."</p>
            <p>"Even more admin content..."</p>
        </div>
    }
}

#[component]
pub fn DocumentsPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Documents"</h1>
            <p>"Document management interface"</p>
            <p>"Document 1"</p>
            <p>"Document 2"</p>
            <p>"Document 3"</p>
        </div>
    }
}

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <div>
            <h1>"User Profile"</h1>
            <p>"Profile settings and information"</p>
            <p>"Name: John Doe"</p>
            <p>"Email: john@example.com"</p>
        </div>
    }
}

#[component]
pub fn AnalyticsPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Analytics Dashboard"</h1>
            <p>"Analytics and metrics"</p>
            <p>"Page Views: 10,234"</p>
            <p>"Unique Visitors: 3,456"</p>
        </div>
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Settings"</h1>
            <p>"Application settings"</p>
            <p>"General Settings"</p>
            <p>"Privacy Settings"</p>
        </div>
    }
}

#[component]
pub fn ReportsPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Reports"</h1>
            <p>"Reports and data exports"</p>
            <p>"Report 1"</p>
            <p>"Report 2"</p>
        </div>
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Main Dashboard"</h1>
            <p>"Dashboard overview"</p>
            <p>"Active Users: 1,234"</p>
            <p>"Revenue: $45,678"</p>
        </div>
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Login"</h1>
            <p>"Please sign in to continue"</p>
            <p>"Username field"</p>
            <p>"Password field"</p>
        </div>
    }
}

#[component]
pub fn SignupPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Sign Up"</h1>
            <p>"Create a new account"</p>
            <p>"Email field"</p>
            <p>"Password field"</p>
        </div>
    }
}