# Chat

One daisyUI chat message with an image, header, bubble, and footer.

```rust
Chat { placement: ChatPlacement::Start,
    ChatImage { class: "avatar",
        div { class: "w-10 rounded-full",
            img { src: "/ada.png", alt: "Ada Lovelace" }
        }
    }
    ChatHeader {
        "Ada Lovelace"
        time { class: "opacity-50", datetime: "10:42", "10:42" }
    }
    ChatBubble { color: ChatBubbleColor::Primary, "The build is green." }
    ChatFooter { span { class: "badge badge-ghost badge-sm", "Delivered" } }
}
```

This is a Presentational component and wraps no Primitive. All five Compound parts render `div`
elements carrying daisyUI's documented classes: `Chat` renders `div.chat` with its required
placement modifier; `ChatImage`, `ChatHeader`, `ChatBubble`, and `ChatFooter` render
`div.chat-image`, `div.chat-header`, `div.chat-bubble`, and `div.chat-footer`.

The four inner parts must remain direct children of `Chat`. The root is a grid, and daisyUI assigns
each part to a grid row and column according to `chat-start` or `chat-end`. An inserted wrapper
turns the parts into ordinary descendants instead of grid items, breaking the image, header,
bubble, and footer placement. The same placement rule removes the bubble's logical start or end
corner and positions its `::before` tail.

The tail mask comes from daisyUI's private inherited `--mask-chat` variable. daisyUI defines the
variable on `.chat`; the direct `ChatBubble` inherits it and uses it from `::before`. It is an
implementation detail rather than a Registry prop or an Axis. Under RTL, start and end remain
logical edges: daisyUI moves the grid columns and tail to the opposite physical edge, then mirrors
the tail mask so it still points away from the bubble.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. Placement
and bubble colour are styling Axes, not interaction state. The Component adds no focus, keyboard,
live-region, or ARIA behaviour.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `placement: ChatPlacement` on `Chat` is required and emits `chat-start` or `chat-end`.
- `color: ChatBubbleColor` on `ChatBubble` emits no modifier for `Default`, or one of
  `chat-bubble-neutral`, `chat-bubble-primary`, `chat-bubble-secondary`, `chat-bubble-accent`,
  `chat-bubble-info`, `chat-bubble-success`, `chat-bubble-warning`, and `chat-bubble-error`.

Both enums are non-exhaustive and expose `ALL` for the Preview and browser spec. The placement has
no unclassed value because daisyUI requires one of the two placements. The default bubble colour
is daisyUI's `base-300` treatment and is distinct from neutral.

Caller classes concatenate and other caller attributes survive on all five parts.

## Conversation semantics

`Chat` represents one visually arranged message, not the conversation that contains it. It does
not emit `role="log"`, `aria-live`, or `aria-relevant`; a caller that receives new messages wraps
the sequence in the appropriate live-region or log semantics for that application. Author names,
image alternative text, machine-readable `time` values, delivery status, and any other semantics
likewise belong to caller content.

Avatar and Badge are optional presentation for that content, not dependencies of Chat. Fuse raw
`avatar` classes onto `ChatImage`, and write the avatar frame and image beneath it. Write raw
`badge` classes on caller content in the header, bubble, or footer when desired. The Component
does not import the Registry's Avatar or Badge Components.

## Deviations

None from daisyUI's class structure. The five parts reproduce its documented chat markup, and the
required placement prop prevents a `chat` root that daisyUI documents as incomplete. All parts use
daisyUI's documented `div` elements and add no semantics of their own.

## daisyUI classes deliberately not used

- `avatar`, `badge`, and their modifiers - these style caller content and remain raw classes rather
  than cross-Component dependencies.
- Responsive-prefixed placement or colour classes - callers add these through `class` when a
  breakpoint should change the presentation.
