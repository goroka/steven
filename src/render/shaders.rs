// Copyright 2015 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use render::glsl;

pub fn add_shaders(reg: &mut glsl::Registry) {
	reg.register("lookup_texture", include_str!("shaders/lookup_texture.glsl"));
	reg.register("get_light", include_str!("shaders/get_light.glsl"));

	reg.register("ui_vertex", include_str!("shaders/ui_vertex.glsl"));
	reg.register("ui_frag", include_str!("shaders/ui_frag.glsl"));
}