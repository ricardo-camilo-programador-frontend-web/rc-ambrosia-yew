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
        let region_items = ambrosia::REGIONS
            .iter()
            .map(|(flag, name, detail)| {
                html! {
                    <li class="region-item">
                        <span class="region-flag">{ flag }</span>
                        <div>
                            <strong class="region-name">{ name }</strong>
                            <p class="region-detail">{ detail }</p>
                        </div>
                    </li>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter light-section" id="regions">
                <div class="chapter-grid-map">
                    <p class="section-label">
                        <span class="section-number">{"08"}</span>
                        <span class="section-divider"></span>
                        <span>{"GLOBAL MAP"}</span>
                    </p>
                    <h2 class="chapter-title reveal">{ "Where It Grows" }</h2>
                    <div class="map-layout">
                        <div class="world-map reveal-scale">
                            <svg viewBox="0 0 800 400" class="map-svg" role="img" aria-label="World map showing Ambrosia apple growing regions">
                                <ellipse cx="120" cy="140" rx="60" ry="80" fill="var(--ambrosia-mist)" />
                                <ellipse cx="180" cy="260" rx="30" ry="60" fill="var(--ambrosia-mist)" />
                                <ellipse cx="280" cy="130" rx="80" ry="60" fill="var(--ambrosia-mist)" />
                                <ellipse cx="400" cy="180" rx="50" ry="50" fill="var(--ambrosia-mist)" />
                                <ellipse cx="550" cy="160" rx="100" ry="70" fill="var(--ambrosia-mist)" />
                                <ellipse cx="680" cy="280" rx="50" ry="40" fill="var(--ambrosia-mist)" />
                                <circle class="map-dot" cx="100" cy="120" r="7"><title>{"British Columbia"}</title></circle>
                                <circle class="map-dot" cx="120" cy="140" r="6"><title>{"Ontario"}</title></circle>
                                <circle class="map-dot" cx="130" cy="260" r="6"><title>{"Washington State"}</title></circle>
                                <circle class="map-dot" cx="200" cy="300" r="6"><title>{"Chile"}</title></circle>
                                <circle class="map-dot" cx="680" cy="280" r="6"><title>{"New Zealand"}</title></circle>
                                <circle class="map-dot" cx="400" cy="140" r="5"><title>{"Netherlands"}</title></circle>
                                <circle class="map-dot" cx="420" cy="170" r="5"><title>{"Italy"}</title></circle>
                            </svg>
                        </div>
                        <aside class="region-panel reveal-left">
                            <ul class="region-list">{ region_items }</ul>
                        </aside>
                    </div>
                </div>
            </section>
        }
    }
}
