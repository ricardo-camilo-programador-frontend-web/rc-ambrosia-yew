use crate::data::varieties::GITHUB_URL;
use crate::i18n::I18nContext;
use rust_i18n::t;
use yew::prelude::*;

#[allow(dead_code)]
pub struct Footer(yew::ContextHandle<I18nContext>);

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let handle = ctx
            .link()
            .context::<I18nContext>(ctx.link().callback(|_: I18nContext| ()))
            .map(|(_, h)| h)
            .expect("I18nContext must be provided");
        Self(handle)
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="footer dark-section">
                <div class="footer-inner">
                    <div class="footer-header reveal">
                        <h2 class="footer-title">{ t!("footer.title") }</h2>
                        <p class="footer-subtitle">{ t!("footer.subtitle") }</p>
                    </div>
                    <div class="footer-columns">
                        <nav class="footer-col">
                            <h3>{ t!("footer.nav") }</h3>
                            <ul>
                                <li><a href="#origin">{ t!("nav.origin") }</a></li>
                                <li><a href="#timeline">{ t!("nav.timeline") }</a></li>
                                <li><a href="#characteristics">{ t!("nav.characteristics") }</a></li>
                                <li><a href="#mythology">{ t!("nav.mythology") }</a></li>
                            </ul>
                        </nav>
                        <nav class="footer-col">
                            <h3>{ t!("footer.resources") }</h3>
                            <ul>
                                <li><a href="#nutrition">{ t!("nav.nutrition") }</a></li>
                                <li><a href="#regions">{ t!("regions.title") }</a></li>
                                <li><a href="#culinary">{ t!("culinary.title") }</a></li>
                                <li><a href="#series">{ t!("nav.series") }</a></li>
                            </ul>
                        </nav>
                        <nav class="footer-col">
                            <h3>{ t!("footer.project") }</h3>
                            <ul>
                                <li><a href={GITHUB_URL} target="_blank" rel="noopener">{ t!("footer.github") }</a></li>
                                <li><a href="https://click-on-the-malus-domestica-ide.netlify.app/apple-varieties" target="_blank" rel="noopener">{ t!("series.label") }</a></li>
                                <li>{ t!("footer.pwa") }</li>
                                <li>{ t!("footer.languages") }</li>
                            </ul>
                        </nav>
                    </div>
                    <div class="footer-bottom">
                        <p>{ t!("footer.copyright") }</p>
                        <div class="footer-social">
                            <a href={GITHUB_URL} target="_blank" rel="noopener">{ t!("footer.github") }</a>
                            <a href="https://www.linkedin.com/in/ricardo-camilo-programador-frontend-web-developer" target="_blank" rel="noopener">{ t!("footer.linkedin") }</a>
                            <a href="https://x.com/Ricardo50993066" target="_blank" rel="noopener">{ t!("footer.x") }</a>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
