#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::mesh_bindings mesh
#import bevy_shader_utils::simplex_noise_2d simplex_noise_2d

@group(1) @binding(0)
var<uniform> height: f32;
@group(1) @binding(1)
var<uniform> size: f32;

const density: f32 = 5.0;
const total_height: f32 = 2.0;

fn random(v: vec2<f32>) -> f32 {
    return fract(sin(dot(v, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    var local_position = uv * size * density;
    var bending_displacement = (vec2<f32>(random(floor(local_position)), random(floor(local_position + 1.0))) - 0.5) * 2.0;
    var wind_displacement = (simplex_noise_2d(local_position * 0.01 + globals.time * 0.25) - 0.5) * 2.0;
    var local_position_displacement = bending_displacement * 2.0 + wind_displacement * 0.2;
    local_position += local_position_displacement * height;

    var shell = floor(local_position);

    var grass_height = random(shell) * simplex_noise_2d(shell * 0.1) * 0.5 + 0.5;
    var grass_height_progress = fract(grass_height);

    if (height <= grass_height) {
        if (height > 0.0)
        {
            var local_uv = fract(local_position);
            var distance_from_center = length(local_uv - vec2<f32>(0.5, 0.5));
            if (distance_from_center > 0.75 * (grass_height - height)) {
                discard;
            }
        }

        var red = simplex_noise_2d(local_position * 0.025) * 0.25 + 0.3;
        var color = vec4<f32>(red, 1.0, 0.0, 1.0);
        return color * (1.0 - (grass_height - height) );
    }

    discard;
}

struct Vertex {
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
};

fn ease_out(t: f32) -> f32 {
    return 1.0 - pow(1.0 - t, 3.0);
}

@vertex
fn vertex(vertex: Vertex) -> MeshVertexOutput {
    var out: MeshVertexOutput;

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_POSITIONS
    var y_displacement = total_height * ease_out(height);
    out.world_position = mesh_functions::mesh_position_local_to_world(
        mesh.model, 
        vec4<f32>(vertex.position + vec3(0.0, y_displacement, 0.0), 1.0)
    );
    out.position = mesh_functions::mesh_position_world_to_clip(out.world_position);
#endif
#ifdef VERTEX_NORMALS
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal);
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        mesh.model, 
        vertex.tangent
    );
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

    return out;
}