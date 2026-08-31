# Avatar

An avatar styled with daisyUI's `avatar` classes, wrapping the `dioxus-primitives` avatar.

```rust
Avatar { label: "Ada Lovelace", status: AvatarStatus::Online,
    AvatarImage { src: "/ada.jpg", alt: "" }
    AvatarFallback { "AL" }
}

// Or as its parts, which is how the outer element is reached.
AvatarRoot { aria_labelledby: "profile-name",
    AvatarFrame { class: "w-24",
        AvatarImage { src: "/ada.jpg", alt: "" }
        AvatarFallback { "AL" }
    }
}
```

The primitive is what makes this worth wrapping: it watches the image's `load` and `error` events
(and reconciles an image that was already in the cache), then unmounts the image and mounts the
fallback when there is nothing to show.

Write a fallback even when you are sure of the image. The primitive renders a `??` of its own for
an avatar that has an image child and no fallback child, and that stand-in lands outside the
frame, where daisyUI's rules do not reach it.

## State bridging

**Tier 2** on the placeholder. daisyUI centres a placeholder's content through
`avatar-placeholder` on the root and matches nothing the primitive sets: not the `data-state`
that is already there, and not the image's load events. So the class is emitted from Rust as a
complete literal.

The state is **mirrored rather than lifted** (ADR-0011), and for a simpler reason than the
accordion item's: there is no state to lift. Whether an avatar has an image is decided by the
browser, and the primitive offers no controlled prop for it because a caller has nothing to
control it with. `AvatarRoot` therefore seeds a signal with the state the primitive starts in,
empty, and updates it from the change callback, which fires for every transition including the
first one out of empty. A caller's own `on_state_change` still runs, because the callback is
intercepted rather than replaced.

The status dot needs no bridging at all: it is a value a caller passes, not a state anything
observes.

## Axes

- `status: AvatarStatus`: `avatar-online`, `avatar-offline`.
- `appearance: AvatarFrameAppearance`: whether the frame emits the utilities that size and shape
  it (`w-16 rounded-full`).
- `appearance: AvatarFallbackAppearance`: whether the fallback emits the utilities that paint the
  placeholder (`flex size-full items-center justify-center bg-neutral text-neutral-content`).

`AvatarStatus::Default` emits nothing and draws no dot. Both appearance axes run the other way,
as ADR-0004 requires of utilities the registry emits: the default arm emits them and `None` emits
nothing, so a caller who wants a *smaller* avatar (or an unpainted placeholder) switches ours
off rather than trying to out-rank a utility they only tie with.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

There is no size axis, because daisyUI has no size classes for an avatar: its own examples write
`w-8` through `w-32` on the frame. The frame is where a caller's classes land, so a size is one
class away and a shape is the same.

## Deviations

**The frame is the registry's, not the primitive's.** daisyUI writes the avatar's shape as
`.avatar > div` and the image fill as `.avatar img`, so the element carrying the size, the
clipping and the corner radius is a `div` the primitive does not render. The component renders it
instead, as the dropdown menu does for its list, for the same reason. Recorded as ADR-0013.

**Caller attributes land on the frame, and the name is routed back to the root.** The frame is
the element worth reaching, so `class`, `id` and the rest go there. The root would then have no
way to be named, and it needs one: the primitive gives it `role="img"`, and an image takes its
name from the author rather than from its content: a nameless one is announced as "image" and
nothing more. So the collapsed component has a `label` prop that lands on the root as
`aria-label`. Anything more than that (`aria-labelledby` at some other element on the page)
means using the parts, which is what the parts are for.

**The placeholder is painted on the fallback, where daisyUI paints the frame.** daisyUI's
placeholder examples put `bg-neutral text-neutral-content` on the frame, which works because
their frame either has an image or has initials, never both in turn. Here the frame is the same
element in both states, so a fill written there would sit behind a loaded image: visible for as
long as the image takes to arrive, and permanently under a transparent one. The fallback is
mounted only while there is no image, so painting it is the same look with none of that.

**The image's inline style stays.** The primitive writes `width`, `height` and `object-fit`
directly on the `img`, which is what daisyUI's `.avatar img` sets too. The two agree, so the
duplication costs nothing and is left alone, and an inline style outranks a class either way, so
a caller who wants a different crop sets `object-fit` through their own inline style on
`AvatarImage` rather than through a utility.

## daisyUI classes deliberately not used

- `avatar-group` and its `-space-x-*` companions: a layout for a *set* of avatars rather than
  part of one, and the set is the caller's element. Writing `avatar-group` on it is one class.
- `mask` and the `mask-*` shapes: daisyUI's general-purpose clipping, which happens to be
  documented alongside avatars. It goes on the frame, which is where a caller's classes land, so
  a squircle avatar is `class: "mask mask-squircle"` and needs nothing from this component.
- `placeholder` on its own: the daisyUI 4 spelling of `avatar-placeholder`. The pinned daisyUI
  is 5.
- The responsive prefixes daisyUI generates for the status classes: a caller reaches those
  through `class`, which concatenates.
