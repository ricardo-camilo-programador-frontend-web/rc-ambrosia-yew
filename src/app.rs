use yew::prelude::*;

use crate::components::{
    ambrosia_gold::AmbrosiaGold, characteristics::Characteristics, culinary::Culinary,
    footer::Footer, hero::Hero, mythology::Mythology, navbar::Navbar, nutrition::Nutrition,
    origin::Origin, parentage::Parentage, regions::Regions, series::Series, timeline::Timeline,
};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <a href="#article-content" class="skip-link">{ "Skip to content" }</a>
                <div class="reading-progress" id="reading-progress" style="width: 0%"></div>
                <Navbar />
                <main id="article-content">
                    <Hero />
                    <Origin />
                    <Timeline />
                    <Characteristics />
                    <Mythology />
                    <Parentage />
                    <Nutrition />
                    <Regions />
                    <Culinary />
                    <AmbrosiaGold />
                    <Series />
                </main>
                <Footer />
            </>
        }
    }
}
