// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0060
// CVE: N/A
// CWE: N/A
// Crate: orbtk
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/orbtk
// Extraction Method: repo_search
// GitHub Repo: redox-os/orbtk
// Commit: 2793671
// File: orbtk_widgets/src/tab_widget.rs


use crate::{api::prelude::*, prelude::*, proc_macros::*, themes::theme_orbtk::*};
 
 // --- KEYS --
 const TAB_HEADER_CONTAINER: &str = "tab_header_container";
 const HEADER_CONTAINER: &str = "header_container";
 const HEADER_BAR: &str = "header_bar";
const BODY_CONTAINER: &str = "body_container";
 // --- KEYS --
 
/**
The `TabHeaderState` is used to store some callbacks that will be applied during template function.
Once the template function is called, they are no more used.
*/
 #[derive(Default, AsAny)]
 pub struct TabHeaderState {
    //Callback called when user click on the header of the tab (normally used to switch to the clicked tab). Only used during initialization
     on_header_mouse_down_callback: Option<Box<dyn 'static + Fn(&mut StatesContext, Mouse) -> bool>>,
    //Callback called when user click on the close button near the tab header (normally used to close the tab). Only used during initialization
     on_close_click_callback: Option<Box<dyn 'static + Fn(&mut StatesContext, Point) -> bool>>,
 
     header_bar: Entity,
         }
 
         let selected = *ctx.widget().get::<bool>("selected");
        let vis = *ctx
             .get_widget(self.header_bar)
             .get::<Visibility>("visibility");
        if selected && vis != Visibility::Visible {
             ctx.get_widget(self.header_bar)
                 .set("visibility", Visibility::Visible);
        } else if !selected && vis == Visibility::Visible {
             ctx.get_widget(self.header_bar)
                 .set("visibility", Visibility::Collapsed);
         }
 );
 
 impl TabHeader {
    ///Set the callback that is called when user click on the header (generally used to switch tab)
     pub fn on_header_mouse_down<T: 'static + Fn(&mut StatesContext, Mouse) -> bool>(
         mut self,
         callback: T,
         self
     }
 
    ///Set the callback that is called when user click on the close button near the header (generally used to close the tab)
     pub fn on_close_click<T: 'static + Fn(&mut StatesContext, Point) -> bool>(
         mut self,
         callback: T,
             mouse_behavior = mouse_behavior.on_mouse_down(callback);
         }
 
        self.name("TabHeader")
             .style("tab_header")
            .selected(false)
            .min_width(64)
            .height(36)
             .background(colors::LYNCH_COLOR)
             .border_radius(4)
             .border_width(0)
             .border_brush("transparent")
            .padding((16, 0, 16, 0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("Unnamed Tab")
             .font_size(orbtk_fonts::FONT_SIZE_12)
             .font("Roboto-Regular")
             .icon("")
             .icon_font("MaterialIcons-Regular")
             .icon_size(orbtk_fonts::ICON_FONT_SIZE_12)
             .icon_brush(colors::LINK_WATER_COLOR)
             .spacing(4)
            .close_button(Visibility::Visible)
             .child(mouse_behavior.build(ctx))
             .child(
                 Container::new()
                     .padding(id)
                     .child(
                         Stack::new()
                            .id(TAB_HEADER_CONTAINER)
                            .spacing(id)
                             .orientation("horizontal")
                             .child(
                                 TextBlock::new()
                                    .text(id)
                                    .v_align("center")
                                     .h_align("start")
                                     .font(id)
                                     .font_size(id)
                                     .foreground(id)
                                     .build(ctx),
                             )
                             .child(button.v_align("center").build(ctx))
             .child(
                 Container::new()
                     .id(HEADER_BAR)
                     .v_align("start")
                     .visibility("collapsed")
                    .style("tab_header_bar")
                     .build(ctx),
             )
     }
     // }
 }
 
///Used to internally
 enum TabWidgetAction {
    SelectByIndex(usize),
    SelectByBody(Entity),
     Add(String, Entity),
     Remove(Entity),
     SetCloseButtonVisibility(bool),
 }
/**
Through the TabWidgetState it is possible to control the behaviour of TabWidget.
Nearly every called function on TabWidgetState will not be executed immediatly, but they are instead stored and executed during the update phase,
in the same order they are submitted.
*/
 #[derive(Default, AsAny)]
 pub struct TabWidgetState {
    ///Store all the pending actions. During the update call they will be resolved
     actions: Vec<TabWidgetAction>,
 
     header_container: Entity,
     body_container: Entity,
 
    tabs: Vec<(Entity, Entity)>, //Header , Body
     selected: usize,
 
     close_button_visibility: bool,
 }
 
 impl TabWidgetState {
    /**
    Switch the selected tab with one with the passed index.
    If the index is greater than the tab count, the last one will be selected.
    */
    pub fn select_by_index(&mut self, index: usize) {
        self.actions.push(TabWidgetAction::SelectByIndex(index));
    }
    /**
    Switch the selected tab with one with the passed body.
    If the no such body is found among the present tabs, nothing happen.
    */
    pub fn select_by_body(&mut self, entity: Entity) {
        self.actions.push(TabWidgetAction::SelectByBody(entity));
    }

    /**
    Remove the tab with the passed body.
    If the no such body is found among the present tabs, nothing happen.
    */
    pub fn remove_by_body(&mut self, entity: Entity) {
        self.actions.push(TabWidgetAction::Remove(entity));
    }

    ///Add a new tab to the widget.
     pub fn add_tab<T: Into<String>>(&mut self, header: T, body: Entity) {
         self.actions.push(TabWidgetAction::Add(header.into(), body));
 
        //At this point the tab has not been added yet, so we can check if len is == 0
         if self.tabs.is_empty() {
             self.select_by_index(0);
         }
     }
 
    ///Set the close button visibility of all the tabs.
    pub fn set_close_button_visibility(&mut self, value: bool) {
        self.actions
            .push(TabWidgetAction::SetCloseButtonVisibility(value));
     }
    ///Get the close button visibility status
    pub fn get_close_button_visibility(&self) -> bool {
        self.close_button_visibility
     }
 
    /**
    Get the tab index associated with the passed body.
    If it is not found, None is returned.
    */
     pub fn get_index(&self, tab_body: Entity) -> Option<usize> {
         for i in 0..self.tabs.len() {
             let (_, body) = self.tabs[i];
         None
     }
 
    /**
    Visually refresh the current tab. This is used when, for example, the selected tab is removed,
    so the tab that take it's place need to be visually updated.
    */
     fn refresh_selected_tab(&mut self, ctx: &mut Context) {
         let tab = self.tabs[self.selected];
         ctx.get_widget(tab.0).set("selected", true);
         toggle_flag("selected", &mut ctx.get_widget(tab.0));
         ctx.get_widget(tab.0).update(false);
         ctx.get_widget(tab.1).set("visibility", Visibility::Visible);
     }
 
    /**
    Change the selected tab by index. Unlike the public "select_by_index", this happen immediately.
    */
     fn select_by_index_internal(&mut self, ctx: &mut Context, mut index: usize) {
        //No tabs could be selected if there are no one, so return immediately
         if self.tabs.is_empty() {
             return;
         }
 
        //If the passed index is greater than tab count, select the last one
         if index >= self.tabs.len() && index != 0 {
             index = self.tabs.len() - 1;
         }
             let current_tab = self.tabs[self.selected];
             let new_tab = self.tabs[index];
 
            //Toggle current button, the new button is toggled by user click
             ctx.get_widget(current_tab.0).set("selected", false);
             toggle_flag("selected", &mut ctx.get_widget(current_tab.0));
             ctx.get_widget(current_tab.0).update(true);
 
            //Hide current body
             ctx.get_widget(current_tab.1)
                 .set("visibility", Visibility::Collapsed);
 
             ctx.get_widget(new_tab.0).set("selected", true);
             toggle_flag("selected", &mut ctx.get_widget(new_tab.0));
             ctx.get_widget(new_tab.0).update(false);
 
            //Show new body
             ctx.get_widget(new_tab.1)
                 .set("visibility", Visibility::Visible);
 
             self.selected = index;
         }
     }
 
    ///Add a new tab to the widget. Unlike the public "add_tab", this happen immediatly.
    fn add_tab_internal(&mut self, ctx: &mut Context, header_text: String, body: Entity) {
        //Create the new tab
        let header = self.create_tab_header(ctx, header_text, body);

        //Set tab body collapsed
        ctx.get_widget(body)
            .set("visibility", Visibility::Collapsed);

        //Push button to the header container
        ctx.append_child_entity_to(header, self.header_container);

        //Push the body to the body container
        ctx.append_child_entity_to(body, self.body_container);

        //Push the new tab to the list
        self.tabs.push((header, body));

        //If the added tab is the first
        if self.tabs.len() == 1 {
            //Select the tab just inserted
            self.selected = 0;
            self.refresh_selected_tab(ctx);
        }
    }

    ///Remove a tab from the widget. Unlike the public "remove_tab", this happen immediatly.
    fn remove_tab_internal(&mut self, ctx: &mut Context, body: Entity) {
        if let Some(index) = self.get_index(body) {
            //Pop the index tab out of the stored tabs
            let (header, body) = self.tabs.remove(index);

            //Push button to the header container
            ctx.remove_child_from(header, self.header_container);

            //Push the body to the body container
            ctx.remove_child_from(body, self.body_container);

            //If there is at least one tab
            if !self.tabs.is_empty() {
                //If selected is greater than tab count, select the last one
                if self.selected >= self.tabs.len() {
                    self.selected = self.tabs.len() - 1;
                }

                //Only update current selection if the removed tab is lesser than the selected.
                //If it is greater, there is no need to update, but simply remove the target tab
                if index <= self.selected {
                    self.refresh_selected_tab(ctx);
                }
            }
        }
    }

    /**
    Set the visibility of the close button on all tabs. Unlike the public "set_close_button_visibility", this happen immediatly.
    If the passed "value" is equal to "self.close_button_visibility", so the requested visibility is already present, nothing happen.
    */
     fn set_close_button_visibility_internal(&mut self, ctx: &mut Context, value: bool) {
         if self.close_button_visibility != value {
             self.close_button_visibility = value;
             }
         }
     }

    //Create a new TabHeader entity and return it. For internal use.
    fn create_tab_header(&self, ctx: &mut Context, text: String, body: Entity) -> Entity {
        let cloned_entity = ctx.entity();
        TabHeader::new()
            .close_button(if self.close_button_visibility {
                Visibility::Visible
            } else {
                Visibility::Collapsed
            })
            .text(text)
            .on_header_mouse_down(move |states, _| {
                states
                    .get_mut::<TabWidgetState>(cloned_entity)
                    .select_by_body(body);
                true
            })
            .on_close_click(move |states, _| {
                states
                    .get_mut::<TabWidgetState>(cloned_entity)
                    .remove_by_body(body);
                true
            })
            .build(&mut ctx.build_context())
    }
 }
 
 impl State for TabWidgetState {
 }
 
 widget!(
    /**
    The `TabWidget` widget can store and control multiple tabs with arbitrary content. Only the selected tab will show it's content.

    This example creates a TabWidget:
    ```rust
    TabWidget::new()
    .tab("Tab header 1",TextBlock::new().text("Tab content 1").build(ctx))
    .tab("Tab header 2",TextBlock::new().text("Tab content 2").build(ctx))
    .tab("Tab header 3",TextBlock::new().text("Tab content 3").build(ctx))
    .build(ctx)
     ```
     */
     TabWidget<TabWidgetState> {
        /// Sets or shares the spacing between tabs.
        spacing: f64,

         /// Sets or shares the background property.
         background: Brush,
 
         border_brush: Brush,
 
         /// Sets or shares the padding property.
        padding: Thickness
     }
 );
 
 impl TabWidget {
    ///Set the close button visibility
     pub fn close_button(mut self, value: bool) -> Self {
         self.state
             .actions
             .push(TabWidgetAction::SetCloseButtonVisibility(value));
         self
     }
    ///Add a tab the widget
     pub fn tab<T: Into<String>>(mut self, header: T, body: Entity) -> Self {
         self.state
             .actions
 
 impl Template for TabWidget {
     fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TabWidget").style("tab_widget").child(
             Grid::new()
                 .rows("34, *")
                 .child(
                     Stack::new()
                         .id(HEADER_CONTAINER)
                         .orientation("horizontal")
                         .spacing(id)
                         .build(ctx),
                 )
                 .child(
                     Container::new()
                         .id(BODY_CONTAINER)
                         .background(id)
                         .border_brush(id)
                         .border_width(id)