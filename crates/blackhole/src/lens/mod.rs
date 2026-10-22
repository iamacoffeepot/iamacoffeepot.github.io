//! The lens: the root that shows the black hole. At `Start` it learns the window's colour, then makes the frame's
//! effects one ask after another, each answer carrying what was made so far: the trace and the bloom's. It asks the
//! orbit for the view, then makes the window's frame, its trace bound to the view. Nothing is placed: every pass is an
//! effect, and the frame follows the window's size itself.

mod bloom;
mod frames;
mod trace;

use crate::mail::{Canvas, Look, Looking, Nothing};
use crate::orbit::Orbit;
use crate::page::say;
use aethereal_actor::{
    Actor, Allows, Answer, Answered, Asks, Beginning, BlobRef, ChildOf, Ending, Fault, Handles, On, Path, Start, Top,
    Turn, path,
};
use aethereal_data::{Bound, Plain};
use aethereal_render::Renderer;
use aethereal_render::effect::{CreateEffect, EffectId, Effected, MadeEffect};
use aethereal_render::frame::builder::Float;
use aethereal_render::frame::{CreateFrame, Frame, Framed, ShapeOf, Shaped};
use aethereal_render::pass::Main;
use aethereal_render::shape::ColourFormat;
use bloom::Effects;

/// The lens: how far it has come in showing the hole on the canvas.
#[derive(Bound)]
#[kind(name = "aether.example.blackhole.lens")]
pub enum Lens {
    /// Its asks are under way.
    Making { canvas: Canvas },
    /// The window's frame is made.
    Showing,
}

impl ChildOf for Lens {
    type Parent = Top;
}

/// Which of the frame's effects was asked for, with the window's colour, which the composite draws into, and the
/// effects made before it.
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.blackhole.effecting")]
pub enum Effecting {
    Trace(ColourFormat),
    Bright(ColourFormat, EffectId<()>),
    Across(ColourFormat, EffectId<()>, EffectId<(Float,)>),
    Down(ColourFormat, EffectId<()>, EffectId<(Float,)>, EffectId<(Float,)>),
    Composite(ColourFormat, EffectId<()>, EffectId<(Float,)>, EffectId<(Float,)>, EffectId<(Float,)>),
}

impl Actor for Lens {
    type Config = Canvas;
    type Key = ();
    type Handles = (
        On<Start>,
        Answer<Shaped, Nothing>,
        Answer<Effected, Effecting>,
        Answer<Looking, Effects>,
        Answer<Framed, Nothing>,
    );
    type BeginSends = ();
    type EndSends = ();

    fn begin(canvas: Canvas, turn: &mut Beginning<'_, Self, Self::BeginSends>) -> Result<Self, Fault> {
        turn.subscribe::<Start>();

        Ok(Self::Making { canvas })
    }

    fn end(self, _: &mut Ending<'_, Self, Self::EndSends>) -> Result<(), Fault> {
        Ok(())
    }
}

impl Handles<On<Start>> for Lens {
    type Sends = (Asks<Path<Renderer>, ShapeOf, Nothing>,);

    fn handle(&mut self, _: Start, turn: &mut Turn<'_, Self, Self::Sends>) -> Result<(), Fault> {
        let Self::Making { canvas } = *self else {
            return Ok(());
        };
        turn.to(path::<Renderer>(())).ask(&ShapeOf::pass::<Main>(Frame::Main(canvas.window)), &Nothing)?;

        Ok(())
    }
}

impl Handles<Answer<Shaped, Nothing>> for Lens {
    type Sends = (Asks<Path<Renderer>, CreateEffect, Effecting>,);

    fn handle(
        &mut self,
        (answered, _): (Answered<Shaped>, Nothing),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Shaped::Shape(shape)) = answered else {
            say(format!("the window's main pass has no shape: {answered:?}"));
            return Ok(());
        };

        ask_effect(Effecting::Trace(shape.colour), turn)
    }
}

impl Effecting {
    /// The WGSL of the effect asked for so.
    const fn source(&self) -> &'static str {
        match self {
            Self::Trace(_) => trace::TRACE_WGSL,
            Self::Bright(..) => bloom::BRIGHT_WGSL,
            Self::Across(..) | Self::Down(..) => bloom::BLUR_WGSL,
            Self::Composite(..) => bloom::COMPOSITE_WGSL,
        }
    }

    /// The ask for the effect asked for so, of the WGSL the blob `wgsl` holds.
    fn create(&self, wgsl: BlobRef) -> CreateEffect {
        match self {
            Self::Trace(_) => trace::effect(wgsl),
            Self::Bright(..) => bloom::bright(wgsl),
            Self::Across(..) => bloom::across(wgsl),
            Self::Down(..) => bloom::down(wgsl),
            Self::Composite(window, ..) => bloom::composite(wgsl, *window),
        }
    }

    /// What follows the effect asked for so once it is made, `made`: the next effect, or, once the last is, all that
    /// was made; or nothing more, when it reads what it was not asked to.
    fn then(self, made: &MadeEffect) -> Next {
        let none = EffectId::of(made);
        let one = EffectId::of(made);
        let two = EffectId::of(made);
        match (self, none, one, two) {
            (Self::Trace(window), Some(trace), ..) => Next::Effect(Self::Bright(window, trace)),
            (Self::Bright(window, trace), _, Some(bright), _) => Next::Effect(Self::Across(window, trace, bright)),
            (Self::Across(window, trace, bright), _, Some(across), _) => {
                Next::Effect(Self::Down(window, trace, bright, across))
            }
            (Self::Down(window, trace, bright, across), _, Some(down), _) => {
                Next::Effect(Self::Composite(window, trace, bright, across, down))
            }
            (Self::Composite(_, trace, bright, across, down), _, _, Some(composite)) => {
                Next::Made(Effects { trace, bright, across, down, composite })
            }
            _ => Next::Unread,
        }
    }
}

/// What follows an effect once it is made.
enum Next {
    /// The next effect, asked for so.
    Effect(Effecting),
    /// Nothing more: every effect, made.
    Made(Effects),
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

impl Handles<Answer<Effected, Effecting>> for Lens {
    type Sends = (Asks<Path<Renderer>, CreateEffect, Effecting>, Asks<Path<Orbit>, Look, Effects>);

    fn handle(
        &mut self,
        (answered, effecting): (Answered<Effected>, Effecting),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(Effected::Made(made)) = answered else {
            say(format!("an effect was not made: {answered:?}"));
            return Ok(());
        };
        let effects = match effecting.then(&made) {
            Next::Effect(effecting) => return ask_effect(effecting, turn),
            Next::Made(effects) => effects,
            Next::Unread => {
                say(format!("an effect reads what it was not asked to: {:?}", made.reads()));
                return Ok(());
            }
        };
        turn.to(path::<Orbit>(())).ask(&Look, &effects)?;

        Ok(())
    }
}

impl Handles<Answer<Looking, Effects>> for Lens {
    type Sends = (Asks<Path<Renderer>, CreateFrame, Nothing>,);

    fn handle(
        &mut self,
        (answered, effects): (Answered<Looking>, Effects),
        turn: &mut Turn<'_, Self, Self::Sends>,
    ) -> Result<(), Fault> {
        let Answered::Yes(looking) = answered else {
            say(format!("the orbit gave no view: {answered:?}"));
            return Ok(());
        };
        let Self::Making { canvas } = *self else {
            return Ok(());
        };
        let create = match frames::shown(canvas.window, &effects, looking.view) {
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
    type Sends = ();

    fn handle(&mut self, (answered, _): (Answered<Framed>, Nothing), _: &mut Turn<'_, Self, ()>) -> Result<(), Fault> {
        let Answered::Yes(Framed::Made(_)) = answered else {
            say(format!("the window's frame was not made: {answered:?}"));
            return Ok(());
        };
        *self = Self::Showing;

        Ok(())
    }
}
