//! # Nebula Components - Building Blocks for Beautiful UIs! 🧱
//! 
//! Reactive, accessible, and works on ANY hardware!
//! 
//! ## Components:
//! - **Button**: Interactive buttons with click handlers
//! - **Text**: Reactive text display
//! 
//! ## Example Counter App:
//! ```rust,ignore
//! use nebula_components::{Button, Text};
//! use nebula_core::Signal;
//! 
//! // Create reactive counter
//! let count = Signal::new(0);
//! 
//! // Create text that displays count
//! let text = Text::from_signal(count.clone());
//! 
//! // Create increment button
//! let inc_button = Button::new("+")
//!     .on_click(move || count.update(|c| c + 1));
//! 
//! // Create decrement button
//! let dec_button = Button::new("-")
//!     .on_click(move || count.update(|c| c - 1));
//! ```

pub mod button;
pub mod text;
pub mod container;
pub mod spacer;
pub mod divider;
pub mod checkbox;
pub mod radio;
pub mod textfield;
pub mod grid;
pub mod image;
pub mod image_cache;
pub mod scroll;
pub mod modal;
pub mod dialog;
pub mod dropdown;
pub mod select;
pub mod tooltip;
pub mod toast;
pub mod context_menu;
pub mod popover;
pub mod menubar;
pub mod tabs;
pub mod progress_bar;
pub mod spinner;
pub mod navigation;
pub mod breadcrumb;
pub mod pagination;
pub mod slider;
pub mod toggle;
pub mod switch;
pub mod range;
pub mod datepicker;
pub mod colorpicker;
pub mod badge;
pub mod chip;
pub mod avatar;
pub mod card;
pub mod accordion;
pub mod alert;
pub mod banner;
pub mod skeleton;
pub mod list;
pub mod table;
pub mod treeview;
pub mod rating;
pub mod stepper;
pub mod drawer;
pub mod fileupload;
pub mod calendar;
pub mod timeline;
pub mod datagrid;
pub mod filebrowser;

pub use button::Button;
pub use text::Text;
pub use container::{VStack, HStack, ZStack, Alignment};
pub use spacer::{Spacer, SpacerType};
pub use divider::{Divider, DividerOrientation, DividerColor};
pub use checkbox::Checkbox;
pub use radio::{Radio, RadioGroup};
pub use textfield::TextField;
pub use grid::Grid;
pub use image::{Image, ImageSource, ImageState, ImageFit};
pub use image_cache::{ImageCache, CachedImage};
pub use scroll::{ScrollView, ScrollDirection};
pub use modal::Modal;
pub use dialog::{Dialog, DialogType};
pub use dropdown::{Dropdown, DropdownOption};
pub use select::{Select, SelectOption};
pub use tooltip::{Tooltip, TooltipPosition};
pub use toast::{Toast, ToastType, ToastPosition};
pub use context_menu::{ContextMenu, ContextMenuItem};
pub use popover::{Popover, PopoverPosition, PopoverTrigger};
pub use menubar::{MenuBar, Menu, MenuItem};
pub use tabs::{Tabs, Tab};
pub use progress_bar::ProgressBar;
pub use spinner::{Spinner, SpinnerSize, LabelPosition};
pub use navigation::{Navigation, NavItem};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use pagination::Pagination;
pub use slider::Slider;
pub use toggle::{Toggle, LabelPosition as ToggleLabelPosition};
pub use switch::Switch;
pub use range::Range;
pub use datepicker::{DatePicker, Date};
pub use colorpicker::{ColorPicker, Color};
pub use badge::{Badge, BadgeVariant};
pub use chip::{Chip, ChipVariant};
pub use avatar::{Avatar, AvatarSize};
pub use card::{Card, CardVariant};
pub use accordion::{Accordion, AccordionItem};
pub use alert::{Alert, AlertSeverity};
pub use banner::{Banner, BannerPosition, BannerVariant};
pub use skeleton::{Skeleton, SkeletonVariant};
pub use list::{List, ListItem, SelectionMode};
pub use table::{Table, TableColumn, TableRow, ColumnAlign, SortDirection};
pub use treeview::{TreeView, TreeNode};
pub use rating::Rating;
pub use stepper::{Stepper, Step, StepperOrientation};
pub use drawer::{Drawer, DrawerPosition, DrawerVariant};
pub use fileupload::{FileUpload, UploadedFile};
pub use calendar::{Calendar, CalendarDate, CalendarView};
pub use timeline::{Timeline, TimelineItem, TimelineMode};
pub use datagrid::{DataGrid, ColumnFilter, FilterOperator};
pub use filebrowser::{FileBrowser, FileEntry, FileType};
