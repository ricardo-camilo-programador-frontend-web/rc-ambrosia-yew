use crate::data::ambrosia;
use yew::prelude::*;

pub struct Regions;

impl Component for Regions {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let region_tags = ambrosia::REGIONS
            .iter()
            .map(|(flag, name, detail)| {
                html! {
                    <span class="region-tag" title={*detail}>
                        { format!("{} {}", flag, name) }
                    </span>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter" id="regions">
                <div class="chapter-wide">
                    <p class="chapter-subtitle reveal">{ "Where It Grows" }</p>
                    <h2 class="chapter-title reveal">{ "Global Presence" }</h2>
                    <div class="world-map reveal">
                        <svg viewBox="0 0 800 400" style="width: 100%; height: auto;" role="img" aria-label="World map showing Ambrosia apple growing regions">
                            // Simplified world map silhouette
                            <ellipse cx="120" cy="140" rx="60" ry="80" fill="var(--ambrosia-mist)" />
                            <ellipse cx="180" cy="260" rx="30" ry="60" fill="var(--ambrosia-mist)" />
                            <ellipse cx="280" cy="130" rx="80" ry="60" fill="var(--ambrosia-mist)" />
                            <ellipse cx="400" cy="180" rx="50" ry="50" fill="var(--ambrosia-mist)" />
                            <ellipse cx="550" cy="160" rx="100" ry="70" fill="var(--ambrosia-mist)" />
                            <ellipse cx="680" cy="280" rx="50" ry="40" fill="var(--ambrosia-mist)" />

                            // Region dots
                            <circle class="region-dot" cx="100" cy="120" r="8"><title>{"British Columbia, Canada"}</title></circle>
                            <circle class="region-dot" cx="120" cy="140" r="6"><title>{"Ontario, Canada"}</title></circle>
                            <circle class="region-dot" cx="130" cy="260" r="6"><title>{"Washington State, USA"}</title></circle>
                            <circle class="region-dot" cx="140" cy="240" r="5"><title>{"New York, USA"}</title></circle>
                            <circle class="region-dot" cx="200" cy="300" r="6"><title>{"Chile"}</title></circle>
                            <circle class="region-dot" cx="680" cy="280" r="6"><title>{"New Zealand"}</title></circle>
                            <circle class="region-dot" cx="400" cy="140" r="5"><title>{"Netherlands"}</title></circle>
                            <circle class="region-dot" cx="420" cy="170" r="5"><title>{"Italy"}</title></circle>
                        </svg>
                    </div>
                    <div class="region-list reveal">
                        { region_tags }
                    </div>
                </div>
            </section>
        }
    }
}
