use crate::data::varieties::GITHUB_URL;
use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="footer">
                <p style="font-family: var(--font-accent); font-size: var(--text-h3);">
                    { "Built with Rust + Yew + WebAssembly" }
                </p>
                <div class="footer-links">
                    <a href={GITHUB_URL} target="_blank" rel="noopener">{ "GitHub" }</a>
                    <a href="https://github.com/ricardo-camilo-programador-frontend-web/rc-ambrosia-yew" target="_blank" rel="noopener">{ "Repository" }</a>
                    <a href="https://click-on-the-malus-domestica-ide.netlify.app/apple-varieties" target="_blank" rel="noopener">{ "Series Hub" }</a>
                    <a href="https://www.linkedin.com/in/ricardo-camilo-programador-frontend-web-developer" target="_blank" rel="noopener">{ "LinkedIn" }</a>
                    <a href="https://x.com/Ricardo50993066" target="_blank" rel="noopener">{ "X" }</a>
                </div>
                <p class="footer-tech">
                    { "© 2026 Ricardo Camilo — Part of the Apple Varieties series" }
                </p>
                <p class="footer-tech" style="margin-top: 0.5rem; font-size: 0.75rem;">
                    { "Design: Agriculture & Farm Landing (Figma Community) · MIT License" }
                </p>
            </footer>
        }
    }
}
