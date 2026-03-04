use crate::assets::Icon;
use crate::config::Config;
use crate::data;
use crate::gallery::{Gallery, Refresh};
use crate::locale;
use crate::market_selector::{ChangeMarket, MarketSelector};
use crate::preview::{OpenPreview, Preview};
use crate::search_bar::SearchBar;
use crate::theme::Theme;
use crate::theme_toggle::ThemeToggle;
use crate::title_bar::TitleBar;
use crate::RUNTIME;
use ahash::AHashMap;
use chrono::Duration;
use gpui::prelude::*;
use gpui::{
    div, img, px, App, Context, DismissEvent, Entity, FocusHandle, Focusable, Render, Window,
};
use gpui_component::input::{InputEvent, InputState};
use gpui_component::scroll::ScrollableElement;
use std::sync::Arc;
use tracing::{debug, error, info};
use xpic::bing::Market;
use xpic::Image;

pub struct XpicApp {
    focus_handle: FocusHandle,

    market: Market,
    cache: AHashMap<Market, Vec<Arc<Image>>>,
    images: Vec<Arc<Image>>,
    filtered_images: Vec<Arc<Image>>,

    search_input: Entity<InputState>,
    search_query: String,

    preview: Option<Entity<Preview>>,
}

impl Focusable for XpicApp {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl XpicApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(t!("search-placeholder")));

        cx.subscribe(&search_input, |this, input, event, cx| {
            if matches!(event, InputEvent::Change) {
                let query = input.read(cx).value().trim().to_string();
                if this.search_query != query {
                    this.search_query = query;
                    this.filtered_images = this.search(&this.search_query);
                    cx.notify();
                }
            }
        })
        .detach();

        let market = cx.global::<Config>().market;
        let images = data::to_arc(data::embedded(market));
        Self::load(market, cx);

        window.on_window_should_close(cx, |window, cx| {
            let config = cx.global_mut::<Config>();

            config.window_bounds = Some(window.bounds());
            config.save();

            true
        });

        let focus_handle = cx.focus_handle();
        cx.focus_self(window);
        cx.on_focus_lost(window, |_, window, cx| {
            cx.focus_self(window);
        })
        .detach();

        XpicApp {
            focus_handle,
            market,
            cache: AHashMap::new(),
            filtered_images: images.clone(),
            images,
            search_input,
            search_query: String::new(),
            preview: None,
        }
    }

    fn on_change_market(
        &mut self,
        ChangeMarket(market_code): &ChangeMarket,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Ok(market) = market_code.parse::<Market>() else {
            return;
        };

        if self.market == market {
            return;
        }

        self.market = market;
        cx.global_mut::<Config>().market = self.market;
        locale::set_from_market(market);
        info!(market = market.code(), "market changed");

        if let Some(cached) = self.cache.get(&market) {
            self.images = cached.clone();

            if data::is_stale(&self.images, Duration::hours(24)) {
                Self::load(market, cx);
            }
        } else {
            self.images = data::to_arc(data::embedded(market));
            Self::load(market, cx);
        }

        self.filtered_images = self.images.clone();
        self.search_query = String::new();
        cx.notify();
    }

    fn on_refresh(&mut self, _: &Refresh, _: &mut Window, cx: &mut Context<Self>) {
        Self::load(self.market, cx);
    }

    /// Loads local data and fetch remote if stale.
    fn load(market: Market, cx: &mut Context<Self>) {
        debug!(market = market.code(), "loading data");

        let path = cx.global::<Config>().data_path(market);
        let handle = RUNTIME.handle().clone();

        cx.spawn(async move |this, cx| {
            let images = handle
                .spawn(async move {
                    let mut images: Vec<Image> = Vec::new();

                    if let Ok(local) = data::load(&path, market).await {
                        images = local;
                    }

                    let mut merged = false;

                    if data::is_stale(&images, Duration::days(7))
                        && let Ok(remote) = data::fetch_remote(market).await
                        && !remote.is_empty()
                    {
                        images = data::merge(&images, &remote);
                        merged = true;
                    }

                    if data::is_stale(&images, Duration::hours(24))
                        && let Ok(api) = data::fetch(market).await
                        && !api.is_empty()
                    {
                        images = data::merge(&images, &api);
                        merged = true;
                    }

                    if merged {
                        if let Err(err) = data::save(&path, &images).await {
                            error!("failed to save data: {err}");
                        }
                    }

                    data::into_arc(images)
                })
                .await?;

            if images.is_empty() {
                return Ok(());
            }

            this.update(cx, |this, cx| {
                info!(
                    count = images.len(),
                    market = market.code(),
                    "images loaded",
                );

                this.images = data::merge(&this.images, &images);
                this.cache.insert(market, this.images.clone());
                this.filtered_images = this.search(&this.search_query);
                cx.notify();
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    }

    fn search(&self, query: impl AsRef<str>) -> Vec<Arc<Image>> {
        let query = query.as_ref();
        if query.is_empty() {
            return self.images.clone();
        }

        let query = query.to_lowercase();

        self.images
            .iter()
            .filter(|img| {
                img.title.to_lowercase().contains(&query)
                    || img.copyright.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    }

    fn on_open_preview(
        &mut self,
        &OpenPreview(index): &OpenPreview,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if index >= self.filtered_images.len() {
            return;
        }

        let image = self.filtered_images[index].clone();
        let entity = cx.new(|cx| Preview::new(image, cx));
        entity.focus_handle(cx).focus(window, cx);

        cx.subscribe(&entity, |this, _, _: &DismissEvent, cx| {
            this.close_preview(cx);
        })
        .detach();

        self.preview = Some(entity);
        cx.notify();
    }

    fn close_preview(&mut self, cx: &mut Context<Self>) {
        self.preview = None;
        cx.notify();
    }

    fn render_title_bar(&self, cx: &App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        TitleBar::new()
            .child(
                div()
                    .id("title-bar-content")
                    .flex()
                    .flex_row()
                    .items_center()
                    .w_full()
                    .child(
                        div()
                            .flex_none()
                            .flex()
                            .items_center()
                            .justify_center()
                            .ml_2()
                            .size(px(18.0))
                            .child(img(Icon::AppIcon).size_full()),
                    )
                    .child(
                        div()
                            .ml_1p5()
                            .text_size(px(12.0))
                            .text_color(theme.foreground)
                            .child("Xpic"),
                    ),
            )
            .child(
                div()
                    .id("title-bar-actions")
                    .flex()
                    .flex_row()
                    .items_center()
                    .mr_1p5()
                    .h(theme.title_bar_height)
                    .child(MarketSelector::new(self.market))
                    .child(ThemeToggle),
            )
    }
}

impl Render for XpicApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("xpic-app")
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .relative()
            .on_action(cx.listener(Self::on_change_market))
            .on_action(cx.listener(Self::on_refresh))
            .on_action(cx.listener(Self::on_open_preview))
            .child(self.render_title_bar(cx))
            .child(div().flex_1().relative().overflow_hidden().child({
                let is_empty = self.filtered_images.is_empty();
                div()
                    .size_full()
                    .overflow_y_scrollbar()
                    .when(is_empty, |this| {
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .flex_1()
                                .size_full()
                                .text_color(theme.caption)
                                .text_sm()
                                .child(t!("no-wallpapers-found")),
                        )
                    })
                    .when(!is_empty, |el| {
                        el.child(Gallery::new(self.filtered_images.clone()))
                    })
            }))
            .child(SearchBar::new(self.search_input.clone()))
            .children(self.preview.clone())
    }
}
