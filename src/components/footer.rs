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
            <footer class="footer dark-section">
                <div class="footer-inner">
                    <div class="footer-header reveal">
                        <h2 class="footer-title">{ "Built with Rust + Yew" }</h2>
                        <p class="footer-subtitle">{ "WebAssembly editorial experience" }</p>
                    </div>
                    <div class="footer-columns">
                        <nav class="footer-col">
                            <h3>{ "Navigation" }</h3>
                            <ul>
                                <li><a href="#origin">{ "Origin" }</a></li>
                                <li><a href="#timeline">{ "Timeline" }</a></li>
                                <li><a href="#characteristics">{ "The Apple" }</a></li>
                                <li><a href="#mythology">{ "Mythology" }</a></li>
                            </ul>
                        </nav>
                        <nav class="footer-col">
                            <h3>{ "Resources" }</h3>
                            <ul>
                                <li><a href="#nutrition">{ "Nutrition" }</a></li>
                                <li><a href="#regions">{ "World Map" }</a></li>
                                <li><a href="#culinary">{ "Culinary" }</a></li>
                                <li><a href="#series">{ "The Series" }</a></li>
                            </ul>
                        </nav>
                        <nav class="footer-col">
                            <h3>{ "Project" }</h3>
                            <ul>
                                <li><a href={GITHUB_URL} target="_blank" rel="noopener">{ "GitHub" }</a></li>
                                <li><a href="https://click-on-the-malus-domestica-ide.netlify.app/apple-varieties" target="_blank" rel="noopener">{ "Series Hub" }</a></li>
                                <li>{ "PWA" }</li>
                                <li>{ "20 Languages" }</li>
                            </ul>
                        </nav>
                    </div>
                    <div class="footer-bottom">
                        <p>{ "\u{00A9} 2026 Ricardo Camilo \u{2014} Part of the Apple Varieties series" }</p>
                        <div class="footer-social">
                            <a href={GITHUB_URL} target="_blank" rel="noopener">{ "GitHub" }</a>
                            <a href="https://www.linkedin.com/in/ricardo-camilo-programador-frontend-web-developer" target="_blank" rel="noopener">{ "LinkedIn" }</a>
                            <a href="https://x.com/Ricardo50993066" target="_blank" rel="noopener">{ "X" }</a>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
