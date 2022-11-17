/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

// Attach sources.
pub mod md_frontmatter;
pub mod styled_text;
pub mod r3bl_syntect_theme;

// Re-export
pub use md_frontmatter::*;
pub use styled_text::*;
pub use r3bl_syntect_theme::*;

// Tests.
mod test_md_frontmatter;
mod test_styled_text;
mod test_common;
mod test_md_parse;
mod test_r3bl_syntect_theme;