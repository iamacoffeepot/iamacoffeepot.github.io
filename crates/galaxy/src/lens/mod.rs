//! The lens: the root that shows the galaxy. At `Start` it makes what it draws one ask after another, each answer
//! carrying what was made so far: the quad, the stars' program, the bloom's effects, the uniforms the stars are drawn
//! with, and the window's frame. It asks the disc to move the stars in that frame's compute pass, and once the disc
//! answers that they move it places them in it. It follows the window's size, and writes the stars' view again at
//! each resize, so their sprites keep their size in pixels and the disc keeps its shape.

mod bloom;
mod frames;
mod stars;

use crate::camera::Camera;
use crate::disc::Disc;
use crate::mail::{Canvas, Move, Moving, Nothing};
use crate::page::say;
use aethereal_actor::{
    Actor, Allows, Answer, Answered, Asks, Beginning, BlobRef, ChildOf, Ending, Fault, Handles, Level, On, Path, Start,
    Tells, Top, Turn, path,
};
use aethereal_data::{Bound, Plain};
use aethereal_render::Renderer;
use aethereal_render::effect::{CreateEffect, EffectId, Effected, MadeEffect};
use aethereal_render::frame::builder::Float;
use aethereal_render::frame::{CreateFrame, Frame, Framed, ShapeOf, Shaped};
use aethereal_render::mesh::{CreateMesh, MeshId, Meshed};
use aethereal_render::pass::Main;
use aethereal_render::placement::{Place, Placed};
use aethereal_render::program::{CreateProgram, ProgramId, Programmed};
use aethereal_render::shape::ColourFormat;
use aethereal_render::uniforms::{CreateUniforms, Uniformed, WriteUniforms};
use aethereal_window::{self as window, Event, Window, WindowId};
use bloom::Effects;
use stars::Drawing;

/// The lens: the window it shows the galaxy on, how the disc is seen there, and how far it has come in showing it.
#[derive(Bound)]
#[kind(name = "aether.example.galaxy.lens")]
pub struct Lens {
    window: WindowId,
    camera: Camera,
    run: Run,
}

impl ChildOf for Lens {
    type Parent = Top;
}

/// How far the lens has come in showing the galaxy.
#[derive(Bound)]
pub enum Run {
    /// Its asks are under way, before the stars' view is made.
    Making,
    /// The stars' view is made, and the frame and the stars' placement are under way: drawn by `drawing`.
    Drawn { drawing: Drawing },
    /// The stars are placed, drawn by `drawing`.
    Showing { drawing: Drawing },
}

/// What was learned before the quad was asked for: the window's colour, which the composite draws into.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.learned")]
pub struct Learned {
    window: ColourFormat,
}

/// What was made before the stars' program was asked for.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.made_mesh")]
pub struct MadeMesh {
    window: ColourFormat,
    mesh: MeshId,
}

/// What was made before the bloom's effects were asked for.
#[derive(Debug, Clone, Copy, Plain)]
pub struct Drawn {
    window: ColourFormat,
    mesh: MeshId,
    program: ProgramId,
}

/// Which of the bloom's effects was asked for, with those made before it.
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.galaxy.effecting")]
pub enum Effecting {
    Bright(Drawn),
    Across(Drawn, EffectId<(Float,)>),
    Down(Drawn, EffectId<(Float,)>, EffectId<(Float,)>),
    Composite(Drawn, EffectId<(Float,)>, EffectId<(Float,)>, EffectId<(Float,)>),
}

/// What was made before the stars' view was asked for.
#[derive(Debug, Clone, Copy, Plain)]
pub struct Made {
    drawn: Drawn,
    effects: Effects,
}

/// The stars' view asked for, with all that was made before it and the camera it was asked through.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.viewing")]
pub struct Viewing {
    made: Made,
    camera: Camera,
}

impl Actor for Lens {
    type Config = Canvas;
    type Key = ();
    type Handles = (
        On<Start>,
        Event,
        Answer<Shaped, Nothing>,
        Answer<Meshed, Learned>,
        Answer<Programmed, MadeMesh>,
        Answer<Effected, Effecting>,
        Answer<Uniformed, Viewing>,
        Answer<Framed, Nothing>,
        Answer<Moving, Nothing>,
        Answer<Placed, Nothing>,
    );
    type BeginSends = (Tells<Path<Window>, window::Subscribe>,);
    type EndSends = ();

    fn begin(canvas: Canvas, turn: &mut Beginning<'_, Self, Self::BeginSends>) -> Result<Self, Fault> {
        turn.to(path::<Window>(canvas.window)).send(&window::Subscribe)?;
        turn.subscribe::<Start>();

        Ok(Self { window: canvas.window, camera: Camera::UNSIZED, run: Run::Making })
    }

    fn end(self, _: &mut Ending<'_, Self, Self::EndSends>) -> Result<(), Fault> {
        Ok(())
    }
}

/// Writes the stars' view through `camera` into `drawing`'s uniforms, in a turn whose sends allow it.
fn write_view<S, I>(drawing: Drawing, camera: Camera, turn: &mut Turn<'_, Lens, S>) -> Result<(), Fault>
where
    S: Allows<Tells<Path<Renderer>, WriteUniforms>, I>,
{
    let Ok(bytes) = turn.put(stars::view(camera)) else {
        turn.log(Level::Warn, "the stars' view was not written: its bytes were not put");
        return Ok(());
    };
    turn.to(path::<Renderer>(())).send(&WriteUniforms { uniforms: drawing.view, first: 0, bytes })?;

    Ok(())
}

/// A resize: the disc seen on the window as it is now, and the stars' view written again once it is made.
impl Handles<Event> for Lens {
    type Sends = (Tells<Path<Renderer>, WriteUniforms>,);

    fn handle(&mut self, event: Event, turn: &mut Turn<'_, Self, Self::Sends>) -> Result<(), Fault> {
        let Event::Resized(surface) = event else {
            return Ok(());
        };
        self.camera = Camera::of(surface.width, surface.height);
        match self.run {
            Run::Making => Ok(()),
            Run::Drawn { drawing } | Run::Showing { drawing } => write_view(drawing, self.camera, turn),
        }
    }
}

impl Handles<On<Start>> for Lens {
    type Sends = (Asks<Path<Renderer>, ShapeOf, Nothing>,);

    fn handle(&mut self, _: Start, turn: &mut Turn<'_, Self, Self::Sends>) -> Result<(), Fault> {
        turn.to(path::<Renderer>(())).ask(&ShapeOf::pass::<Main>(Frame::Main(self.window)), &Nothing)?;

        Ok(())
    }
}

impl Handles<Answer<Shaped, Nothing>> for Lens {
    type Sends = (Asks<Path<Renderer>, CreateMesh, Learned>,);

    fn handle(
        &mut self,
        (answered, _): (Answered<Shaped>, Nothing),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Shaped::Shape(shape)) = answered else {
            say(format!("the window's main pass has no shape: {answered:?}"));
            return Ok(());
        };
        let (layout, corners, indices) = stars::quad();
        let (vertices, indices) = match (turn.put(corners), turn.put(indices)) {
            (Ok(vertices), Ok(indices)) => (vertices, indices),
            (Err(why), _) | (_, Err(why)) => {
                say(format!("no quad asked: {why}"));
                return Ok(());
            }
        };
        turn.to(path::<Renderer>(()))
            .ask(&CreateMesh { layout, vertices, indices }, &Learned { window: shape.colour })?;

        Ok(())
    }
}

impl Handles<Answer<Meshed, Learned>> for Lens {
    type Sends = (Asks<Path<Renderer>, CreateProgram, MadeMesh>,);

    fn handle(
        &mut self,
        (answered, learned): (Answered<Meshed>, Learned),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Meshed::Made(mesh)) = answered else {
            say(format!("the quad was not made: {answered:?}"));
            return Ok(());
        };
        let wgsl = match turn.put(Vec::from(stars::STARS_WGSL)) {
            Ok(wgsl) => wgsl,
            Err(why) => {
                say(format!("no stars' program asked: {why}"));
                return Ok(());
            }
        };
        turn.to(path::<Renderer>(())).ask(&stars::program(wgsl), &MadeMesh { window: learned.window, mesh })?;

        Ok(())
    }
}

impl Effecting {
    /// The WGSL of the effect asked for so.
    const fn source(&self) -> &'static str {
        match self {
            Self::Bright(_) => bloom::BRIGHT_WGSL,
            Self::Across(..) | Self::Down(..) => bloom::BLUR_WGSL,
            Self::Composite(..) => bloom::COMPOSITE_WGSL,
        }
    }

    /// The ask for the effect asked for so, of the WGSL the blob `wgsl` holds.
    fn create(&self, wgsl: BlobRef) -> CreateEffect {
        match self {
            Self::Bright(_) => bloom::bright(wgsl),
            Self::Across(..) => bloom::across(wgsl),
            Self::Down(..) => bloom::down(wgsl),
            Self::Composite(drawn, ..) => bloom::composite(wgsl, drawn.window),
        }
    }

    /// What follows the effect asked for so once it is made, `made`: the next effect, or, once the last is, all that
    /// was made; or nothing more, when it reads what it was not asked to.
    fn then(self, made: &MadeEffect) -> Next {
        let read = EffectId::of(made);
        let both = EffectId::of(made);
        match (self, read, both) {
            (Self::Bright(drawn), Some(made), _) => Next::Effect(Self::Across(drawn, made)),
            (Self::Across(drawn, bright), Some(made), _) => Next::Effect(Self::Down(drawn, bright, made)),
            (Self::Down(drawn, bright, across), Some(made), _) => {
                Next::Effect(Self::Composite(drawn, bright, across, made))
            }
            (Self::Composite(drawn, bright, across, down), _, Some(composite)) => {
                Next::Made(Made { drawn, effects: Effects { bright, across, down, composite } })
            }
            _ => Next::Unread,
        }
    }
}

/// What follows an effect once it is made.
enum Next {
    /// The next effect, asked for so.
    Effect(Effecting),
    /// Nothing more of the bloom: all that was made.
    Made(Made),
    /// Nothing: the effect reads what it was not asked to.
    Unread,
}

/// Asks for the effect `effecting` names, in a turn whose sends allow it, saying why it was not when its WGSL could
/// not be put.
fn ask_effect<S, I>(effecting: Effecting, turn: &mut Turn<'_, Lens, S>) -> Result<(), Fault>
where
    S: Allows<Asks<Path<Renderer>, CreateEffect, Effecting>, I>,
{
    let wgsl = match turn.put(Vec::from(effecting.source())) {
        Ok(wgsl) => wgsl,
        Err(why) => {
            say(format!("no effect asked: {why}"));
            return Ok(());
        }
    };
    turn.to(path::<Renderer>(())).ask(&effecting.create(wgsl), &effecting)?;

    Ok(())
}

impl Handles<Answer<Programmed, MadeMesh>> for Lens {
    type Sends = (Asks<Path<Renderer>, CreateEffect, Effecting>,);

    fn handle(
        &mut self,
        (answered, made): (Answered<Programmed>, MadeMesh),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Programmed::Made(program)) = answered else {
            say(format!("the stars' program was not made: {answered:?}"));
            return Ok(());
        };

        ask_effect(Effecting::Bright(Drawn { window: made.window, mesh: made.mesh, program }), turn)
    }
}

impl Handles<Answer<Effected, Effecting>> for Lens {
    type Sends = (Asks<Path<Renderer>, CreateEffect, Effecting>, Asks<Path<Renderer>, CreateUniforms, Viewing>);

    fn handle(
        &mut self,
        (answered, effecting): (Answered<Effected>, Effecting),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Effected::Made(made)) = answered else {
            say(format!("an effect was not made: {answered:?}"));
            return Ok(());
        };
        let made = match effecting.then(&made) {
            Next::Effect(effecting) => return ask_effect(effecting, turn),
            Next::Made(made) => made,
            Next::Unread => {
                say(format!("an effect reads what it was not asked to: {:?}", made.reads()));
                return Ok(());
            }
        };
        let Ok(bytes) = turn.put(stars::view(self.camera)) else {
            say(String::from("no stars' view asked: its bytes were not put"));
            return Ok(());
        };
        turn.to(path::<Renderer>(())).ask(&CreateUniforms { bytes }, &Viewing { made, camera: self.camera })?;

        Ok(())
    }
}

impl Handles<Answer<Uniformed, Viewing>> for Lens {
    type Sends = (Tells<Path<Renderer>, WriteUniforms>, Asks<Path<Renderer>, CreateFrame, Nothing>);

    fn handle(
        &mut self,
        (answered, viewing): (Answered<Uniformed>, Viewing),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Uniformed::Made(view)) = answered else {
            say(format!("the stars' view was not made: {answered:?}"));
            return Ok(());
        };
        let made = viewing.made;
        let drawing = Drawing { mesh: made.drawn.mesh, program: made.drawn.program, view };
        self.run = Run::Drawn { drawing };
        // The window was resized while the view was being made: it is written as the window now stands.
        let resized = viewing.camera != self.camera;
        if resized {
            write_view(drawing, self.camera, turn)?;
        }
        let create = match frames::shown(self.window, &made.effects) {
            Ok(create) => create,
            Err(why) => {
                say(format!("the window's frame was not built: {why:?}"));
                return Ok(());
            }
        };
        turn.to(path::<Renderer>(())).ask(&create, &Nothing)?;

        Ok(())
    }
}

impl Handles<Answer<Framed, Nothing>> for Lens {
    type Sends = (Asks<Path<Disc>, Move, Nothing>,);

    fn handle(
        &mut self,
        (answered, _): (Answered<Framed>, Nothing),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Framed::Made(_)) = answered else {
            say(format!("the window's frame was not made: {answered:?}"));
            return Ok(());
        };
        turn.to(path::<Disc>(())).ask(&Move, &Nothing)?;

        Ok(())
    }
}

impl Handles<Answer<Moving, Nothing>> for Lens {
    type Sends = (Asks<Path<Renderer>, Place, Nothing>,);

    fn handle(
        &mut self,
        (answered, _): (Answered<Moving>, Nothing),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(moving) = answered else {
            say(format!("the stars do not move: {answered:?}"));
            return Ok(());
        };
        let Run::Drawn { drawing } = self.run else {
            return Ok(());
        };
        turn.to(path::<Renderer>(())).ask(&drawing.place(moving.stars, self.window), &Nothing)?;

        Ok(())
    }
}

impl Handles<Answer<Placed, Nothing>> for Lens {
    type Sends = ();

    fn handle(&mut self, (answered, _): (Answered<Placed>, Nothing), _: &mut Turn<'_, Self, ()>) -> Result<(), Fault> {
        let Answered::Yes(Placed::Made(_)) = answered else {
            say(format!("the stars were not placed: {answered:?}"));
            return Ok(());
        };
        let Run::Drawn { drawing } = self.run else {
            return Ok(());
        };
        self.run = Run::Showing { drawing };

        Ok(())
    }
}
