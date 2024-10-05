use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui,
};
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

use crate::game::event::{
    BallCatchEvent, BallMixerRotateEvent, BallReleaseEvent, BallRigidChange,
    DrawInnerStickDownEvent, DrawInnerStickUpEvent, DrawStickDownEvent, DrawStickRigidChangeEvent,
    DrawStickUpEvent, GameRunEvent, PoolOutletCoverCloseEvent, PoolOutletCoverOpenEvent,
};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        // gui plugin
        app.add_plugins(EguiPlugin)
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
            .add_systems(Update, inspector_ui);

        // perf ui
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, setup);
    }
}
fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            // egui::CollapsingHeader::new("Materials").show(ui, |ui| {
            //     bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            // });

            // ui.heading("Entities");
            // bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);

            if ui.button("mixer 0").clicked() {
                world.send_event(BallMixerRotateEvent(0.));
            }

            if ui.button("mixer 1").clicked() {
                world.send_event(BallMixerRotateEvent(1.));
            }

            if ui.button("mixer 10").clicked() {
                world.send_event(BallMixerRotateEvent(10.));
            }

            if ui.button("stick down").clicked() {
                world.send_event(DrawStickDownEvent);
            }

            if ui.button("stick up").clicked() {
                world.send_event(DrawStickUpEvent);
            }

            if ui.button("stick inner down").clicked() {
                world.send_event(DrawInnerStickDownEvent);
            }

            if ui.button("stick inner up").clicked() {
                world.send_event(DrawInnerStickUpEvent);
            }

            //

            if ui.button("stick static").clicked() {
                world.send_event(DrawStickRigidChangeEvent(true));
            }

            if ui.button("stick static remove").clicked() {
                world.send_event(DrawStickRigidChangeEvent(false));
            }

            //

            if ui.button("ball static").clicked() {
                world.send_event(BallRigidChange(false));
            }

            if ui.button("ball dynamci").clicked() {
                world.send_event(BallRigidChange(true));
            }

            //

            if ui.button("ball catch").clicked() {
                world.send_event(BallCatchEvent);
            }

            if ui.button("ball release").clicked() {
                world.send_event(BallReleaseEvent);
            }

            //

            if ui.button("pool outlet  open").clicked() {
                world.send_event(PoolOutletCoverOpenEvent);
            }

            if ui.button("pool outlet  close").clicked() {
                world.send_event(PoolOutletCoverCloseEvent);
            }

            //

            if ui.button("game run").clicked() {
                world.send_event(GameRunEvent);
            }
        });
    });
}
fn setup(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
}
