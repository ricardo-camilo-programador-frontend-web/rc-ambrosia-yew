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
            <section class="chapter" id="ambrosia-gold">
                <div class="chapter-inner">
                    <p class="chapter-subtitle reveal">{ "The Premium Tier" }</p>
                    <h2 class="chapter-title reveal">{ "Ambrosia Gold" }</h2>
                    <p class="reveal">
                        { "Beyond the original variety, Ambrosia Gold represents the premium branding of this remarkable apple. As a 'club' variety, Ambrosia was historically patented and quality-controlled — only authorized growers could produce it, ensuring consistent excellence." }
                    </p>
                    <p class="reveal" style="margin-top: 1rem;">
                        { "The patent expired in Canada in 2015 and in the United States in 2017, yet Ambrosia's reputation for quality endures. It remains active in other countries until as late as 2034. Today, Ambrosia stands alongside Honeycrisp and Gala as one of the varieties reshaping the apple industry — rising at the expense of the once-dominant Red Delicious." }
                    </p>
                    <p class="reveal" style="margin-top: 1rem; font-weight: 600; color: var(--ambrosia-red);">
                        { "As of 2024, it is one of the most-produced apple varieties in Canada." }
                    </p>
                </div>
            </section>
        }
    }
}
