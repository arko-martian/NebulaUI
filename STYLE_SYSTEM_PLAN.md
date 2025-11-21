# Nebula UI Style System - Implementation Plan

## Vision

Nebula UI will support ALL 200+ CSS properties in a **type-safe, compile-time checked, zero-cost Rust API**. Web developers can use their CSS knowledge directly, but with Rust's safety and performance!

## Design Philosophy

### Type-Safe CSS in Rust

Instead of string-based CSS:
```css
/* Old way (error-prone) */
.button {
    background-color: #0A0E17;
    padding: 20px;
    border-radius: 8px;
}
```

We have type-safe Rust:
```rust
// Nebula UI way (compile-time checked!)
Button::new("Click me")
    .background_color(Color::hex("#0A0E17"))
    .padding(20.px())
    .border_radius(8.px())
```

### Three Syntax Styles

All properties work with all three syntax modes:

**SwiftUI-like:**
```rust
VStack(spacing: 20) {
    Text("Hello")
        .font(.title)
        .foregroundColor(.blue)
        .padding(20)
}
```

**Flutter-like:**
```rust
Column(
    spacing: 20,
    children: [
        Text("Hello", style: TextStyle {
            fontSize: 24,
            color: Colors.blue,
            padding: EdgeInsets.all(20),
        })
    ]
)
```

**React-like:**
```rust
div(
    style: Style {
        display: Display::Flex,
        flexDirection: FlexDirection::Column,
        gap: 20.px(),
        padding: 20.px(),
    },
    children: [h1("Hello")]
)
```

## Property Categories



### Category 1: Layout (PHASE 0-1) - 40 properties
**Priority: CRITICAL** - Needed for basic UI

- **Flexbox:** display, flex, flex-direction, flex-wrap, flex-grow, flex-shrink, flex-basis, justify-content, align-items, align-content, align-self, gap, order
- **Grid:** grid, grid-template-columns, grid-template-rows, grid-column, grid-row, grid-auto-flow, grid-auto-columns, grid-auto-rows
- **Position:** position, top, right, bottom, left, inset, z-index
- **Sizing:** width, height, min-width, min-height, max-width, max-height, aspect-ratio
- **Spacing:** margin, padding (all variants: m-*, p-*, mx, my, px, py, mt, mr, mb, ml, pt, pr, pb, pl)

**Implementation File:** `nebula-core/src/style/layout.rs`

### Category 2: Visual (PHASE 1) - 50 properties
**Priority: HIGH** - Needed for beautiful UI

- **Background:** background-color, background-image, background-size, background-position, background-repeat, background-attachment, background-clip, background-origin, background-blend-mode
- **Border:** border, border-width, border-style, border-color, border-radius (all variants)
- **Shadow:** box-shadow, text-shadow
- **Opacity:** opacity
- **Overflow:** overflow, overflow-x, overflow-y, overscroll-behavior
- **Visibility:** visibility, display

**Implementation File:** `nebula-core/src/style/visual.rs`

### Category 3: Typography (PHASE 1) - 30 properties
**Priority: HIGH** - Text is everywhere!

- **Font:** font-family, font-size, font-weight, font-style, font-variant
- **Text:** color, text-align, text-decoration, text-transform, text-overflow, text-shadow
- **Line:** line-height, letter-spacing, word-spacing, white-space, word-break, line-clamp
- **List:** list-style, list-style-type, list-style-position, list-style-image

**Implementation File:** `nebula-core/src/style/typography.rs`

### Category 4: Transform & Animation (PHASE 1-2) - 30 properties
**Priority: MEDIUM** - For smooth interactions

- **Transform:** transform, rotate, scale, translate, skew, transform-origin, perspective
- **Animation:** animation, animation-name, animation-duration, animation-timing-function, animation-delay, animation-iteration-count, animation-direction, animation-fill-mode, animation-play-state
- **Transition:** transition, transition-property, transition-duration, transition-timing-function, transition-delay

**Implementation File:** `nebula-core/src/style/animation.rs`

### Category 5: Effects (PHASE 2) - 20 properties
**Priority: MEDIUM** - For advanced visuals

- **Filter:** filter, backdrop-filter (blur, brightness, contrast, grayscale, etc.)
- **Blend:** mix-blend-mode, background-blend-mode, isolation
- **Clip:** clip-path, mask
- **Cursor:** cursor, pointer-events

**Implementation File:** `nebula-core/src/style/effects.rs`

### Category 6: Advanced Layout (PHASE 2) - 15 properties
**Priority: LOW** - For complex layouts

- **Columns:** columns, column-count, column-width, column-gap, column-rule, column-span
- **Table:** table-layout, border-collapse, border-spacing, caption-side, empty-cells
- **Float:** float, clear

**Implementation File:** `nebula-core/src/style/advanced_layout.rs`

### Category 7: Scroll & Interaction (PHASE 2) - 10 properties
**Priority: LOW** - For enhanced UX

- **Scroll:** scroll-behavior, scroll-margin, scroll-padding, scroll-snap-type, scroll-snap-align
- **Resize:** resize
- **User Select:** user-select

**Implementation File:** `nebula-core/src/style/interaction.rs`

### Category 8: Rare/Specialized (PHASE 3) - 5 properties
**Priority: VERY LOW** - Edge cases

- **Writing Mode:** direction, writing-mode, text-orientation
- **Other:** appearance, all

**Implementation File:** `nebula-core/src/style/specialized.rs`



## Implementation Strategy

### Phase 0 (Current) - Basic Layout
**Goal:** Get basic layout working for tracer bullet

Properties to implement:
- display (block, flex, none)
- width, height
- margin, padding (basic)
- background-color
- position (relative, absolute)

**File:** `nebula-core/src/style.rs` (basic)

### Phase 1 (Month 1-3) - Core Properties
**Goal:** 80% of common use cases

Properties to implement:
- All flexbox properties
- All spacing properties (margin/padding variants)
- All border properties
- All typography properties
- Basic animations (opacity, transform)

**Files:**
- `nebula-core/src/style/layout.rs`
- `nebula-core/src/style/visual.rs`
- `nebula-core/src/style/typography.rs`

### Phase 2 (Month 4-6) - Advanced Properties
**Goal:** 95% of use cases

Properties to implement:
- Grid layout
- Filters and effects
- Advanced animations
- Scroll properties

**Files:**
- `nebula-core/src/style/animation.rs`
- `nebula-core/src/style/effects.rs`
- `nebula-core/src/style/interaction.rs`

### Phase 3 (Month 7-12) - Complete Coverage
**Goal:** 100% CSS property coverage

Properties to implement:
- Columns
- Table layout
- Specialized properties

**Files:**
- `nebula-core/src/style/advanced_layout.rs`
- `nebula-core/src/style/specialized.rs`



## Type System Design

### Core Types

```rust
// nebula-core/src/style/types.rs

/// Length units (px, em, rem, %, vh, vw, etc.)
#[derive(Debug, Clone, Copy)]
pub enum Length {
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Vh(f32),
    Vw(f32),
    Auto,
}

impl Length {
    pub fn px(value: f32) -> Self { Length::Px(value) }
    pub fn em(value: f32) -> Self { Length::Em(value) }
    pub fn percent(value: f32) -> Self { Length::Percent(value) }
}

// Convenient trait for numeric literals
pub trait LengthExt {
    fn px(self) -> Length;
    fn em(self) -> Length;
    fn percent(self) -> Length;
}

impl LengthExt for i32 {
    fn px(self) -> Length { Length::Px(self as f32) }
    fn em(self) -> Length { Length::Em(self as f32) }
    fn percent(self) -> Length { Length::Percent(self as f32) }
}

impl LengthExt for f32 {
    fn px(self) -> Length { Length::Px(self) }
    fn em(self) -> Length { Length::Em(self) }
    fn percent(self) -> Length { Length::Percent(self) }
}

/// Color with multiple formats
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn hex(hex: &str) -> Self {
        // Parse #RRGGBB or #RRGGBBAA
        // Implementation details...
        todo!()
    }
    
    // Named colors
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    // ... 140+ named colors
}

/// Display type
#[derive(Debug, Clone, Copy)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    None,
    Contents,
}

/// Position type
#[derive(Debug, Clone, Copy)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

/// Flex direction
#[derive(Debug, Clone, Copy)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

/// Justify content
#[derive(Debug, Clone, Copy)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Align items
#[derive(Debug, Clone, Copy)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

// ... 100+ more enums for all CSS values
```

### Style Builder Pattern

```rust
// nebula-core/src/style/builder.rs

#[derive(Debug, Clone, Default)]
pub struct Style {
    // Layout
    pub display: Option<Display>,
    pub position: Option<Position>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub margin: Spacing,
    pub padding: Spacing,
    
    // Flexbox
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub gap: Option<Length>,
    
    // Visual
    pub background_color: Option<Color>,
    pub border: Border,
    pub border_radius: BorderRadius,
    pub opacity: Option<f32>,
    
    // Typography
    pub color: Option<Color>,
    pub font_size: Option<Length>,
    pub font_weight: Option<FontWeight>,
    pub text_align: Option<TextAlign>,
    
    // Transform & Animation
    pub transform: Option<Transform>,
    pub animation: Option<Animation>,
    
    // ... all 200+ properties
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
    
    // Builder methods for all properties
    pub fn display(mut self, display: Display) -> Self {
        self.display = Some(display);
        self
    }
    
    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }
    
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }
    
    // ... 200+ builder methods
}
```



## Usage Examples

### Example 1: Button with Full Styling

```rust
Button::new("Click Me")
    // Layout
    .display(Display::InlineFlex)
    .padding(Spacing::all(16.px()))
    .margin(Spacing::vertical(8.px()))
    
    // Visual
    .background_color(Color::hex("#0A0E17"))
    .border_radius(BorderRadius::all(8.px()))
    .border(Border::solid(2.px(), Color::hex("#3a86ff")))
    .box_shadow(Shadow::new(0.px(), 4.px(), 12.px(), Color::rgba(0, 0, 0, 128)))
    
    // Typography
    .color(Color::WHITE)
    .font_size(16.px())
    .font_weight(FontWeight::Bold)
    .text_align(TextAlign::Center)
    
    // Interaction
    .cursor(Cursor::Pointer)
    .transition(Transition::all(0.3.s(), Easing::EaseOut))
    
    // Hover state
    .on_hover(|style| style
        .background_color(Color::hex("#1a1e27"))
        .transform(Transform::scale(1.05))
    )
```

### Example 2: Responsive Grid Layout

```rust
Container::new()
    .display(Display::Grid)
    .grid_template_columns(vec![
        GridTrack::fr(1.0),
        GridTrack::fr(1.0),
        GridTrack::fr(1.0),
    ])
    .gap(24.px())
    .padding(Spacing::all(32.px()))
    
    // Responsive breakpoints
    .on_mobile(|style| style
        .grid_template_columns(vec![GridTrack::fr(1.0)])
    )
    .on_tablet(|style| style
        .grid_template_columns(vec![
            GridTrack::fr(1.0),
            GridTrack::fr(1.0),
        ])
    )
```

### Example 3: Animated Card

```rust
Card::new()
    // Layout
    .width(300.px())
    .height(400.px())
    .padding(Spacing::all(24.px()))
    
    // Visual
    .background_color(Color::WHITE)
    .border_radius(BorderRadius::all(16.px()))
    .box_shadow(Shadow::new(0.px(), 8.px(), 24.px(), Color::rgba(0, 0, 0, 64)))
    
    // Effects
    .backdrop_filter(Filter::blur(10.px()))
    .mix_blend_mode(BlendMode::Normal)
    
    // Animation
    .animation(Animation::new("slide-in")
        .duration(0.5.s())
        .timing_function(Easing::EaseOut)
        .fill_mode(FillMode::Forwards)
    )
    
    // Transform on hover
    .on_hover(|style| style
        .transform(Transform::translate_y(-8.px()).scale(1.02))
        .box_shadow(Shadow::new(0.px(), 16.px(), 48.px(), Color::rgba(0, 0, 0, 128)))
    )
```



## Performance Considerations

### Compile-Time Optimizations

1. **Zero-Cost Abstractions:** All style methods compile to direct field assignments
2. **Const Evaluation:** Color parsing, unit conversions happen at compile time
3. **Dead Code Elimination:** Unused properties are stripped from binary
4. **Monomorphization:** Specialized code for each style combination

### Runtime Optimizations

1. **Style Caching:** Identical styles share memory
2. **Dirty Tracking:** Only recompute changed styles
3. **Batch Updates:** Multiple style changes = one layout pass
4. **GPU Acceleration:** Transforms, filters, animations run on GPU

### Memory Layout

```rust
// Compact memory representation
#[repr(C)]
pub struct ComputedStyle {
    // Packed into 128 bytes for cache efficiency
    layout: LayoutStyle,      // 32 bytes
    visual: VisualStyle,      // 32 bytes
    typography: TypographyStyle, // 32 bytes
    transform: TransformStyle,   // 32 bytes
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn style_builder_sets_properties() {
        let style = Style::new()
            .width(100.px())
            .height(200.px())
            .background_color(Color::RED);
        
        assert_eq!(style.width, Some(Length::Px(100.0)));
        assert_eq!(style.height, Some(Length::Px(200.0)));
        assert_eq!(style.background_color, Some(Color::RED));
    }
    
    #[test]
    fn length_conversions() {
        assert_eq!(20.px(), Length::Px(20.0));
        assert_eq!(1.5.em(), Length::Em(1.5));
        assert_eq!(50.percent(), Length::Percent(50.0));
    }
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn any_valid_color_parses(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let color = Color::rgb(r, g, b);
        assert_eq!(color.r, r);
        assert_eq!(color.g, g);
        assert_eq!(color.b, b);
    }
}
```

## Documentation Strategy

### Auto-Generated Docs

Every property will have:
1. **Description:** What it does
2. **CSS Equivalent:** The CSS property it maps to
3. **Examples:** Code examples
4. **Browser Support:** Which renderers support it
5. **Performance Notes:** Any performance considerations

Example:
```rust
/// Sets the background color of the element.
///
/// # CSS Equivalent
/// `background-color: #0A0E17;`
///
/// # Examples
/// ```
/// Button::new("Click")
///     .background_color(Color::hex("#0A0E17"))
///     .background_color(Color::rgb(10, 14, 23))
///     .background_color(Color::BLUE)
/// ```
///
/// # Performance
/// Changing background color triggers repaint but not layout.
pub fn background_color(mut self, color: Color) -> Self {
    self.background_color = Some(color);
    self
}
```

## Migration Path

### From CSS

```css
/* CSS */
.button {
    display: flex;
    padding: 16px 24px;
    background-color: #0A0E17;
    border-radius: 8px;
    color: white;
    font-size: 16px;
    font-weight: bold;
}
```

```rust
// Nebula UI
Button::new("Click")
    .display(Display::Flex)
    .padding(Spacing::horizontal(24.px()).vertical(16.px()))
    .background_color(Color::hex("#0A0E17"))
    .border_radius(BorderRadius::all(8.px()))
    .color(Color::WHITE)
    .font_size(16.px())
    .font_weight(FontWeight::Bold)
```

### From Tailwind

```html
<!-- Tailwind -->
<button class="flex px-6 py-4 bg-gray-900 rounded-lg text-white text-base font-bold">
    Click
</button>
```

```rust
// Nebula UI
Button::new("Click")
    .display(Display::Flex)
    .padding(Spacing::horizontal(24.px()).vertical(16.px()))
    .background_color(Color::GRAY_900)
    .border_radius(BorderRadius::all(8.px()))
    .color(Color::WHITE)
    .font_size(16.px())
    .font_weight(FontWeight::Bold)
```

## Next Steps

1. ✅ **Phase 0 (Current):** Implement basic layout properties for tracer bullet
2. **Phase 1 (Month 1):** Implement all layout + visual + typography properties
3. **Phase 2 (Month 2-3):** Implement animation + effects properties
4. **Phase 3 (Month 4-6):** Implement advanced + specialized properties
5. **Documentation:** Auto-generate docs for all 200+ properties
6. **Examples:** Create showcase app demonstrating all properties

---

**Status:** 📋 Planning Complete - Ready to implement incrementally!

**Total Properties:** 200+
**Categories:** 8
**Implementation Files:** 8
**Estimated Completion:** Phase 3 (Month 7-12)

**Let's build the most powerful style system in any UI framework!** 🚀🎨

