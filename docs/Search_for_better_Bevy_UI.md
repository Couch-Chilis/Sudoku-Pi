# The search for a better Bevy UI

The other day I read this excellent blog post on the 
[challenges and opportunities in the future of `bevy_ui`](https://www.leafwing-studios.com/blog/ecs-gui-framework/).
If you haven't read it yet, I recommend you do, since it provides quite some
context for what I'm about to talk about here.

As part of our [Sudoku Pi project](https://github.com/Couch-Chilis/sudoku-pi), I
ended up building a custom UI layer as an alternative to Bevy UI. The main
blocker for us, that made me decide not to use Bevy UI, was the inability with
Bevy UI to add custom transforms to nodes, effectively eliminating any
opportunity to add animations to them. Animations were very important to us.

But the lack of animations isn't the only problem with Bevy UI, as the blog
mentioned above points out in excellent detail. Some of those we have been able
to side-step using our own custom UI layer, but many we didn't. What I can say
though, is that I saw many of these problems up close, and as such I have
opinions on them. Some of these opinions might even be valuable to others :)

To tamper expectations, don't expect detailed solutions from this post. The UI
solution I built is highly specific to Sudoku Pi, and it shares enough of the
limitations of Bevy UI that I wouldn't recommend adopting it wholesale. But if
you're in a position where you need to build a similar solution, or maybe you're
someone wishing to improve Bevy UI itself, hopefully these thoughts are useful
to you.

But, to Bevy's credit, there's one thing I like to share upfront: Building a
custom UI solution isn't actually that *hard*. Bevy already provides you with
the building blocks, so if you can implement a layout algorithm, you're mostly
there. The trickier part is making something that is actually pleasant to use.

## Layout

Just like Bevy UI, I decided to create a flex-inspired layout system. It
shouldn't really be a surprise, since like many, I have a fair share of web
experience. I'm not really a fan of CSS though, so when the other blog mentioned
[Morphism](https://github.com/vizia/morphorm) as being their preferred layout
algorithm, I checked it out and regretted I had not tried it out instead. A
missed opportunity on my part!

It wasn't a huge loss, since I only needed to implement a flex-like subset that
was sufficient for a single project. But I like Morphism's simplicity, and I do
hope one day Bevy UI will adopt it or something like it. I didn't try it out
yet, but I'm particularly intrigued by how they solve respecting aspect ratios
by allowing you to define width or height using a ratio of the other. (My own
solution was a `preserve_aspect_ratio` boolean, which was still better than the
hoops you may need to jump through with CSS, but it was still awkward to
implement.)

### Expressions

That said, there is one thing my own layout system implemented, which I didn't
see in either Bevy UI or Morphism: The ability to calculate values through
expressions. Every now and then, you may want to perform a calculation like the
following:

```rs
height - 2. * line_height
```

But there's a catch: Bevy UI has a `Val` enum that allows you to express values
such as `Val::Px(300.)` or `Val::Percent(50.)`. So what if the `height` above is
a percentage value, but the `line_height` is a pixel value? Wherever you define
your values, you may not know how many pixels fit in a percentage, so you cannot
perform the calculation on the spot. You need to somehow pass this expression to
your layout system and let it perform the calculation for you. CSS once again
offers us a suggestion: It allows us to express such calculations like this:

```css
calc(50% - 2 * 300px)
```

Similarly, my own layout system has a `Val` enum just like Bevy's, but with an
additional variant: `Val::Calc(Box<Expr>)`, where `Expr` is a type that allows
for composing expressions that themselves also use `Val` values.

The real kicker? Thanks to Rust's operating overloading, the expression that you
see above *just works* and automatically evaluates to a `Val`, simply because
`height` and `line_height` are (see [source](https://github.com/Couch-Chilis/sudoku-pi/blob/c6c8e1d0587db319fedbb466fdacd2c817ac2d1f/lib/src/game/game_ui.rs#L134)).

### Transforms and animations

Before I move on to the next step, let me say just a few things about the
`Transform`. After all, Bevy UI's inability to let me touch its transforms was
the main reason I opted to create my own layout system in the first place.

The solution I opted to use was very straightforward: I have a `FlexItemStyle`
component (my version of Bevy UI's `Style` component), and I simply added
another `Transform` to it. During the layout phase my algorithm checks if the
transform is set to anything but the default value, and if it is, it applies
the `Transform` from the `FlexItemStyle` to the item's `Transform`, on top of
the scaling and the translation that were determined by the layout algorithm.

Then I'm able to apply animations to these custom transforms using
`bevy_tweening`, a crate I've absolutely learned to love during my time with
Bevy. I saw the other blog suggest that Bevy should have their own tweening too,
but I'd go one step further: Why not integrate the `bevy_tweening` crate into
Bevy wholesale? There's probably intricacies I'm unaware of, but if it can be
done, I'd be in favor.

## Verbosity

The first time I saw how hierarchies are created with Bevy UI, I balked. Oh my.
But then I learned it's not even Bevy UI in particular that's at fault here.
It's simply how Bevy entity hierarchies are created in general. And so it
quickly turned out that even creating my own UI system didn't really help here.
Being able to only create children for an entity within a callback feels too
limited. There's probably reasons at work here that I'm not privy to, although
I suspect it has to do with wanting to optimize for GPU batching. Either way,
it's clear to me the Bevy devs are skilled at what they do, so I don't doubt
they have good reasons. But whatever the reasons, it does not make for a very
compelling UI development experience.

So here I am, with my own little UI system, but hamstrung by Bevy's verbose
method for defining hierarchies. Could I somehow work around it to make things
more ergonomic? I started experimenting with my own APIs.

In the following examples I will use a few code snippets that represent our
timer "widget". It's basically just a text node surrounded by two horizontal
borders, one on the top and one on the bottom. Nowadays, Bevy UI has some
built-in support for borders, but back when I started they didn't, and my own
layout system doesn't support them either. So the two borders are just
additional nodes that I need to insert myself.

Here's what it looks like with a standard Bevy UI coding style:

```rs
fn build_timer(cb: &mut ChildBuilder, resources: &ResourceBag) {
    let width = Val::Pixel(100);
    let height = Val::Pixel(42);
    let line_height = Val::Pixel(1);

    let text_style = TextStyle {
        font: resources.fonts.medium.clone(),
        font_size: 70.,
        color: COLOR_TIMER_TEXT,
    };

    cb.spawn((
        FlexItemBundle {
            style: FlexItemStyle {
                flex_base: Size::new(width.clone(), line_height.clone()),
                ..default()
            },
            ..default()
        },
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));

    cb.spawn(FlexBundle {
        item: FlexItemBundle {
            style: FlexItemStyle {
                flex_base: Size::new(width.clone(), height - 2. * line_height.clone()),
                flex_grow: 1.,
                ..default()
            },
            ..default()
        }
        ..default()
    })
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Timer,
            FlexTextBundle {
                text: Text2dBundle {
                    text: Text::from_section("0:00", text_style),
                    ..default()
                },
                ..default()
            },
        ));
    });

    cb.spawn((
        FlexItemBundle {
            style: FlexItemStyle {
                flex_base: Size::new(width, line_height),
                ..default()
            },
            ..default()
        },
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));
}
```

In order to improve things just a little bit, the first thing I did was to
define builder methods for commonly constructed bundles and components. Soon
enough, the same snippet started looking like this:

```rs
fn build_timer(cb: &mut ChildBuilder, resources: &ResourceBag) {
    let width = Val::Pixel(100);
    let height = Val::Pixel(42);
    let line_height = Val::Pixel(1);

    let text_style = TextStyle {
        font: resources.fonts.medium.clone(),
        font_size: 70.,
        color: COLOR_TIMER_TEXT,
    };

    cb.spawn((
        FlexItemBundle::from_style(FlexItemStyle::fixed_size(
            width.clone(),
            line_height.clone(),
        )),
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));

    cb.spawn(FlexBundle::from_item_style(FlexItemStyle::minimum_size(
        width.clone(),
        height - 2. * line_height.clone(),
    )))
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Timer,
            FlexTextBundle::from_text(Text::from_section("0:00", text_style)),
        ));
    });

    cb.spawn((
        FlexItemBundle::from_style(FlexItemStyle::fixed_size(width, line_height)),
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));
}
```

I wouldn't call it a huge improvement, but it was still significantly more
readable than the initial syntax. In fact, I was happy enough that I built the
first version of our game this way. But the structure was still the same, and
while it got the job done, the verbosity quickly became worse again whenever I
needed to make layouts responsive. When I added iPad support, I ended up with
all these ugly checks inline:

```rs
fn build_timer(cb: &mut ChildBuilder, resources: &ResourceBag) {
    let width = if resources.screen_sizing.is_tablet() {
        Val::Pixel(150)
    } else {
        Val::Pixel(100)
    };
    let height = if resources.screen_sizing.is_tablet() {
        Val::Pixel(64)
    } else {
        Val::Pixel(42)
    };
    let line_height = Val::Pixel(1);

    let text_style = TextStyle {
        font: resources.fonts.medium.clone(),
        font_size: if resources.screen_sizing.is_tablet() {
            105.
        } else {
            70.
        },
        color: COLOR_TIMER_TEXT,
    };

    cb.spawn((
        FlexItemBundle::from_style(FlexItemStyle::fixed_size(
            width.clone(),
            line_height.clone(),
        )),
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));

    cb.spawn(FlexBundle::from_item_style(FlexItemStyle::minimum_size(
        width.clone(),
        height - 2. * line_height.clone(),
    )))
    .with_children(|text_leaf| {
        text_leaf.spawn((
            Timer,
            FlexTextBundle::from_text(Text::from_section("0:00", text_style)),
        ));
    });

    cb.spawn((
        FlexItemBundle::from_style(FlexItemStyle::fixed_size(width, line_height)),
        SpriteBundle {
            sprite: Sprite::from_color(COLOR_TIMER_BORDER),
            ..default()
        },
    ));
}
```

Ouch.

Well, at least I got the iPad version delivered. But now I wanted to branch out
with Android support and suddenly the amount of screen ratios would become too
many to test. And I also wanted to support the Steam Deck, which meant landscape
support.

I needed a better way to handle responsiveness.

## Responsiveness

As I started to think about how I could improve the responsiveness without going
through months of tedious testing and manual tweaking of endless screen sizes,
I had yet another look at how I compose these "widgets". If I could somehow make
my UI definitions easier, with the ability to resize and relayout at runtime,
while centralizing the code responsible for the styling, instead of littering
pixels counts and break points and other magic values all across the codebase,
then maybe making things responsive was actually feasible.

Redesigning my UI that way was also months of work, but at least it was fun
work :)

And I managed.

Compare the following snippet with the last one from the previous section:

```rs
fn timer() -> impl FnOnce(&Props, &mut ChildBuilder) {
    fragment3(
        rect(COLOR_TIMER_BORDER, game_screen_timer_line_size),
        row(
            game_screen_timer_inner_size,
            (),
            text_t(
                Timer,
                "0:00",
                (
                    font_medium,
                    game_screen_timer_font_size,
                    text_color(COLOR_TIMER_TEXT),
                ),
            ),
        ),
        rect(COLOR_TIMER_BORDER, game_screen_timer_line_size),
    )
}
```

This snippet is *significantly* easier to read and to maintain. It's by no means
perfect, and the comparison with the previous snippet is also unfair, because I
have pushed all the styling into helper functions.

But its strength also lies in these very helper functions. At first sight, and
with a bit of squinting, you could say it reads almost like adding CSS classes
to the UI nodes. Personally, I think the readability is a huge improvement.

But readability isn't their only advantage. I don't just run these functions
when the UI is constructed to calculate dimensions and such, I also let them
register *dynamic* styles. This way, whenever I need to relayout (for instance,
when I resize a window on my desktop, or when a user rotates their phone) these
dynamic styles get to re-evaluate and the UI becomes truly responsive.

This allows me to resize a window and quickly confirm visually whether the UI
scales correctly with various screen sizes and aspect ratios.

Mission accomplished.

## Closing Words


