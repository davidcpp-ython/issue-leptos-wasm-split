#![recursion_limit = "256"]

use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes, A}, path, Lazy};

pub mod pages;
pub mod lazy_routes;

use pages::HomePage;
use lazy_routes::{
    LazyAdmin,
    LazyDocuments,
    LazyProfile,
    LazyAnalytics,
    LazySettings,
    LazyReports,
    LazyDashboard,
    LazyLogin,
    LazySignup,
    LazyUserManagementAdministrationDashboard,
    LazyEnterpriseResourcePlanningSystemIntegrationModule,
    LazyCustomerRelationshipManagementAnalyticsDashboard,
    LazyFinancialReportingComplianceDocumentationViewer,
    LazyInternationalBusinessOperationsManagementConsole,
    LazyAdvancedConfigurationSettingsAdministrationPanel,
    LazyComprehensiveDataAnalyticsVisualizationDashboard,
    LazyEnterpriseSecurityAuditComplianceReportingSystem,
    LazyGlobalSupplyChainManagementOptimizationPlatform,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>" | "
                <A href="/admin">"Admin"</A>" | "
                <A href="/documents">"Documents"</A>" | "
                <A href="/profile">"Profile"</A>" | "
                <A href="/analytics">"Analytics"</A>" | "
                <A href="/settings">"Settings"</A>" | "
                <A href="/reports">"Reports"</A>" | "
                <A href="/dashboard">"Dashboard"</A>" | "
                <A href="/login">"Login"</A>" | "
                <A href="/signup">"Signup"</A>
            </nav>
            <main>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/admin") view={Lazy::<LazyAdmin>::new()} />
                    <Route path=path!("/documents") view={Lazy::<LazyDocuments>::new()} />
                    <Route path=path!("/profile") view={Lazy::<LazyProfile>::new()} />
                    <Route path=path!("/analytics") view={Lazy::<LazyAnalytics>::new()} />
                    <Route path=path!("/settings") view={Lazy::<LazySettings>::new()} />
                    <Route path=path!("/reports") view={Lazy::<LazyReports>::new()} />
                    <Route path=path!("/dashboard") view={Lazy::<LazyDashboard>::new()} />
                    <Route path=path!("/login") view={Lazy::<LazyLogin>::new()} />
                    <Route path=path!("/signup") view={Lazy::<LazySignup>::new()} />
                    <Route path=path!("/user-management") view={Lazy::<LazyUserManagementAdministrationDashboard>::new()} />
                    <Route path=path!("/erp-integration") view={Lazy::<LazyEnterpriseResourcePlanningSystemIntegrationModule>::new()} />
                    <Route path=path!("/crm-analytics") view={Lazy::<LazyCustomerRelationshipManagementAnalyticsDashboard>::new()} />
                    <Route path=path!("/compliance-docs") view={Lazy::<LazyFinancialReportingComplianceDocumentationViewer>::new()} />
                    <Route path=path!("/business-ops") view={Lazy::<LazyInternationalBusinessOperationsManagementConsole>::new()} />
                    <Route path=path!("/advanced-config") view={Lazy::<LazyAdvancedConfigurationSettingsAdministrationPanel>::new()} />
                    <Route path=path!("/data-viz") view={Lazy::<LazyComprehensiveDataAnalyticsVisualizationDashboard>::new()} />
                    <Route path=path!("/security-audit") view={Lazy::<LazyEnterpriseSecurityAuditComplianceReportingSystem>::new()} />
                    <Route path=path!("/supply-chain") view={Lazy::<LazyGlobalSupplyChainManagementOptimizationPlatform>::new()} />
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_lazy(App);
}