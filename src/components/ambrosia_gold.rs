use yew::prelude::*;

pub struct AmbrosiaGold;

impl Component for AmbrosiaGold {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="chapter dark-section gold-section" id="ambrosia-gold">
                <div class="chapter-grid-two">
                    <div class="gold-text-col reveal-left">
                        <p class="section-label section-label-light">
                            <span class="section-number">{"10"}</span>
                            <span class="section-divider"></span>
                            <span>{"PREMIUM TIER"}</span>
                        </p>
                        <h2 class="gold-title">{ "Ambrosia Gold" }</h2>
                        <p class="gold-intro">
                            { "As a 'club' variety, Ambrosia was patented and quality-controlled. Only authorized growers could produce it." }
                        </p>
                        <div class="gold-cards">
                            <article class="premium-card reveal">
                                <h3 class="premium-heading">{ "CLUB VARIETY" }</h3>
                                <p>{ "Patented, quality-controlled, marketed collectively" }</p>
                            </article>
                            <article class="premium-card reveal">
                                <h3 class="premium-heading">{ "PATENT STORY" }</h3>
                                <p>{ "Canada 2015, USA 2017, Chile 2021. Others until 2034" }</p>
                            </article>
                            <article class="premium-card reveal">
                                <h3 class="premium-heading">{ "MARKET RISE" }</h3>
                                <p>{ "One of Canada's most-produced varieties as of 2024" }</p>
                            </article>
                        </div>
                    </div>
                    <div class="gold-visual-col reveal-scale">
                        <div class="gold-crate-placeholder">
                            <span>{ "\u{1F4E6}" }</span>
                            <p>{ "Ambrosia Gold" }</p>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
