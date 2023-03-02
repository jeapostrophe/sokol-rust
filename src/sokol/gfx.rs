// machine generated, do not edit


#[repr(C)]
pub struct Buffer {
    pub id: u32,
}
#[repr(C)]
pub struct Image {
    pub id: u32,
}
#[repr(C)]
pub struct Shader {
    pub id: u32,
}
#[repr(C)]
pub struct Pipeline {
    pub id: u32,
}
#[repr(C)]
pub struct Pass {
    pub id: u32,
}
#[repr(C)]
pub struct Context {
    pub id: u32,
}
#[repr(C)]
pub struct Range {
    pub ptr: *const std::ffi::c_void,
    pub size: usize,
}
pub const SG_INVALID_ID: i32 = 0;
pub const SG_NUM_SHADER_STAGES: i32 = 2;
pub const SG_NUM_INFLIGHT_FRAMES: i32 = 2;
pub const SG_MAX_COLOR_ATTACHMENTS: i32 = 4;
pub const SG_MAX_SHADERSTAGE_BUFFERS: i32 = 8;
pub const SG_MAX_SHADERSTAGE_IMAGES: i32 = 12;
pub const SG_MAX_SHADERSTAGE_UBS: i32 = 4;
pub const SG_MAX_UB_MEMBERS: i32 = 16;
pub const SG_MAX_VERTEX_ATTRIBUTES: i32 = 16;
pub const SG_MAX_MIPMAPS: i32 = 16;
pub const SG_MAX_TEXTUREARRAY_LAYERS: i32 = 128;
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
pub enum Backend {
    Glcore33,
    Gles2,
    Gles3,
    D3d11,
    MetalIos,
    MetalMacos,
    MetalSimulator,
    Wgpu,
    Dummy,
}
#[repr(C)]
pub enum PixelFormat {
    Default,
    None,
    R8,
    R8sn,
    R8ui,
    R8si,
    R16,
    R16sn,
    R16ui,
    R16si,
    R16f,
    Rg8,
    Rg8sn,
    Rg8ui,
    Rg8si,
    R32ui,
    R32si,
    R32f,
    Rg16,
    Rg16sn,
    Rg16ui,
    Rg16si,
    Rg16f,
    Rgba8,
    Rgba8sn,
    Rgba8ui,
    Rgba8si,
    Bgra8,
    Rgb10a2,
    Rg11b10f,
    Rg32ui,
    Rg32si,
    Rg32f,
    Rgba16,
    Rgba16sn,
    Rgba16ui,
    Rgba16si,
    Rgba16f,
    Rgba32ui,
    Rgba32si,
    Rgba32f,
    Depth,
    DepthStencil,
    Bc1Rgba,
    Bc2Rgba,
    Bc3Rgba,
    Bc4R,
    Bc4Rsn,
    Bc5Rg,
    Bc5Rgsn,
    Bc6hRgbf,
    Bc6hRgbuf,
    Bc7Rgba,
    PvrtcRgb2bpp,
    PvrtcRgb4bpp,
    PvrtcRgba2bpp,
    PvrtcRgba4bpp,
    Etc2Rgb8,
    Etc2Rgb8a1,
    Etc2Rgba8,
    Etc2Rg11,
    Etc2Rg11sn,
    Rgb9e5,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub struct PixelformatInfo {
    pub sample: bool,
    pub filter: bool,
    pub render: bool,
    pub blend: bool,
    pub msaa: bool,
    pub depth: bool,
}
#[repr(C)]
pub struct Features {
    pub instancing: bool,
    pub origin_top_left: bool,
    pub multiple_render_targets: bool,
    pub msaa_render_targets: bool,
    pub imagetype_3d: bool,
    pub imagetype_array: bool,
    pub image_clamp_to_border: bool,
    pub mrt_independent_blend_state: bool,
    pub mrt_independent_write_mask: bool,
}
#[repr(C)]
pub struct Limits {
    pub max_image_size_2d: i32,
    pub max_image_size_cube: i32,
    pub max_image_size_3d: i32,
    pub max_image_size_array: i32,
    pub max_image_array_layers: i32,
    pub max_vertex_attrs: i32,
    pub gl_max_vertex_uniform_vectors: i32,
}
#[repr(C)]
pub enum ResourceState {
    Initial,
    Alloc,
    Valid,
    Failed,
    Invalid,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum Usage {
    Default,
    Immutable,
    Dynamic,
    Stream,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum BufferType {
    Default,
    Vertexbuffer,
    Indexbuffer,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum IndexType {
    Default,
    None,
    Uint16,
    Uint32,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum ImageType {
    Default,
    Num2d,
    Cube,
    Num3d,
    Array,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum SamplerType {
    Default,
    Float,
    Sint,
    Uint,
}
#[repr(C)]
pub enum CubeFace {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum ShaderStage {
    Vs,
    Fs,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum PrimitiveType {
    Default,
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum Filter {
    Default,
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum Wrap {
    Default,
    Repeat,
    ClampToEdge,
    ClampToBorder,
    MirroredRepeat,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum BorderColor {
    Default,
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum VertexFormat {
    Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Byte4,
    Byte4n,
    Ubyte4,
    Ubyte4n,
    Short2,
    Short2n,
    Ushort2n,
    Short4,
    Short4n,
    Ushort4n,
    Uint10N2,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum VertexStep {
    Default,
    PerVertex,
    PerInstance,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum UniformType {
    Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
    Mat4,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum UniformLayout {
    Default,
    Native,
    Std140,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum CullMode {
    Default,
    None,
    Front,
    Back,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum FaceWinding {
    Default,
    Ccw,
    Cw,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum CompareFunc {
    Default,
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum StencilOp {
    Default,
    Keep,
    Zero,
    Replace,
    IncrClamp,
    DecrClamp,
    Invert,
    IncrWrap,
    DecrWrap,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum BlendFactor {
    Default,
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColor,
    OneMinusBlendColor,
    BlendAlpha,
    OneMinusBlendAlpha,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum BlendOp {
    Default,
    Add,
    Subtract,
    ReverseSubtract,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum ColorMask {
    Default = 0,
    None = 16,
    R = 1,
    G = 2,
    Rg = 3,
    B = 4,
    Rb = 5,
    Gb = 6,
    Rgb = 7,
    A = 8,
    Ra = 9,
    Ga = 10,
    Rga = 11,
    Ba = 12,
    Rba = 13,
    Gba = 14,
    Rgba = 15,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum Action {
    Default,
    Clear,
    Load,
    Dontcare,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub struct ColorAttachmentAction {
    pub action: Action,
    pub value: Color,
}
#[repr(C)]
pub struct DepthAttachmentAction {
    pub action: Action,
    pub value: f32,
}
#[repr(C)]
pub struct StencilAttachmentAction {
    pub action: Action,
    pub value: u8,
}
#[repr(C)]
pub struct PassAction {
    pub _start_canary: u32,
    pub colors: [ColorAttachmentAction; 4],
    pub depth: DepthAttachmentAction,
    pub stencil: StencilAttachmentAction,
    pub _end_canary: u32,
}
#[repr(C)]
pub struct Bindings {
    pub _start_canary: u32,
    pub vertex_buffers: [Buffer; 8],
    pub vertex_buffer_offsets: [i32; 8],
    pub index_buffer: Buffer,
    pub index_buffer_offset: i32,
    pub vs_images: [Image; 12],
    pub fs_images: [Image; 12],
    pub _end_canary: u32,
}
#[repr(C)]
pub struct BufferDesc {
    pub _start_canary: u32,
    pub size: usize,
    pub r#type: BufferType,
    pub usage: Usage,
    pub data: Range,
    pub label: *mut u8,
    pub gl_buffers: [u32; 2],
    pub mtl_buffers: [*const std::ffi::c_void; 2],
    pub d3d11_buffer: *const std::ffi::c_void,
    pub wgpu_buffer: *const std::ffi::c_void,
    pub _end_canary: u32,
}
#[repr(C)]
pub struct ImageData {
    pub subimage: [[Range; 6]; 16],
}
#[repr(C)]
pub struct ImageDesc {
    pub _start_canary: u32,
    pub r#type: ImageType,
    pub render_target: bool,
    pub width: i32,
    pub height: i32,
    pub num_slices: i32,
    pub num_mipmaps: i32,
    pub usage: Usage,
    pub pixel_format: PixelFormat,
    pub sample_count: i32,
    pub min_filter: Filter,
    pub mag_filter: Filter,
    pub wrap_u: Wrap,
    pub wrap_v: Wrap,
    pub wrap_w: Wrap,
    pub border_color: BorderColor,
    pub max_anisotropy: u32,
    pub min_lod: f32,
    pub max_lod: f32,
    pub data: ImageData,
    pub label: *mut u8,
    pub gl_textures: [u32; 2],
    pub gl_texture_target: u32,
    pub mtl_textures: [*const std::ffi::c_void; 2],
    pub d3d11_texture: *const std::ffi::c_void,
    pub d3d11_shader_resource_view: *const std::ffi::c_void,
    pub wgpu_texture: *const std::ffi::c_void,
    pub _end_canary: u32,
}
#[repr(C)]
pub struct ShaderAttrDesc {
    pub name: *mut u8,
    pub sem_name: *mut u8,
    pub sem_index: i32,
}
#[repr(C)]
pub struct ShaderUniformDesc {
    pub name: *mut u8,
    pub r#type: UniformType,
    pub array_count: i32,
}
#[repr(C)]
pub struct ShaderUniformBlockDesc {
    pub size: usize,
    pub layout: UniformLayout,
    pub uniforms: [ShaderUniformDesc; 16],
}
#[repr(C)]
pub struct ShaderImageDesc {
    pub name: *mut u8,
    pub image_type: ImageType,
    pub sampler_type: SamplerType,
}
#[repr(C)]
pub struct ShaderStageDesc {
    pub source: *mut u8,
    pub bytecode: Range,
    pub entry: *mut u8,
    pub d3d11_target: *mut u8,
    pub uniform_blocks: [ShaderUniformBlockDesc; 4],
    pub images: [ShaderImageDesc; 12],
}
#[repr(C)]
pub struct ShaderDesc {
    pub _start_canary: u32,
    pub attrs: [ShaderAttrDesc; 16],
    pub vs: ShaderStageDesc,
    pub fs: ShaderStageDesc,
    pub label: *mut u8,
    pub _end_canary: u32,
}
#[repr(C)]
pub struct BufferLayoutDesc {
    pub stride: i32,
    pub step_func: VertexStep,
    pub step_rate: i32,
}
#[repr(C)]
pub struct VertexAttrDesc {
    pub buffer_index: i32,
    pub offset: i32,
    pub format: VertexFormat,
}
#[repr(C)]
pub struct LayoutDesc {
    pub buffers: [BufferLayoutDesc; 8],
    pub attrs: [VertexAttrDesc; 16],
}
#[repr(C)]
pub struct StencilFaceState {
    pub compare: CompareFunc,
    pub fail_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub pass_op: StencilOp,
}
#[repr(C)]
pub struct StencilState {
    pub enabled: bool,
    pub front: StencilFaceState,
    pub back: StencilFaceState,
    pub read_mask: u8,
    pub write_mask: u8,
    pub r#ref: u8,
}
#[repr(C)]
pub struct DepthState {
    pub pixel_format: PixelFormat,
    pub compare: CompareFunc,
    pub write_enabled: bool,
    pub bias: f32,
    pub bias_slope_scale: f32,
    pub bias_clamp: f32,
}
#[repr(C)]
pub struct BlendState {
    pub enabled: bool,
    pub src_factor_rgb: BlendFactor,
    pub dst_factor_rgb: BlendFactor,
    pub op_rgb: BlendOp,
    pub src_factor_alpha: BlendFactor,
    pub dst_factor_alpha: BlendFactor,
    pub op_alpha: BlendOp,
}
#[repr(C)]
pub struct ColorState {
    pub pixel_format: PixelFormat,
    pub write_mask: ColorMask,
    pub blend: BlendState,
}
#[repr(C)]
pub struct PipelineDesc {
    pub _start_canary: u32,
    pub shader: Shader,
    pub layout: LayoutDesc,
    pub depth: DepthState,
    pub stencil: StencilState,
    pub color_count: i32,
    pub colors: [ColorState; 4],
    pub primitive_type: PrimitiveType,
    pub index_type: IndexType,
    pub cull_mode: CullMode,
    pub face_winding: FaceWinding,
    pub sample_count: i32,
    pub blend_color: Color,
    pub alpha_to_coverage_enabled: bool,
    pub label: *mut u8,
    pub _end_canary: u32,
}
#[repr(C)]
pub struct PassAttachmentDesc {
    pub image: Image,
    pub mip_level: i32,
    pub slice: i32,
}
#[repr(C)]
pub struct PassDesc {
    pub _start_canary: u32,
    pub color_attachments: [PassAttachmentDesc; 4],
    pub depth_stencil_attachment: PassAttachmentDesc,
    pub label: *mut u8,
    pub _end_canary: u32,
}
#[repr(C)]
pub struct SlotInfo {
    pub state: ResourceState,
    pub res_id: u32,
    pub ctx_id: u32,
}
#[repr(C)]
pub struct BufferInfo {
    pub slot: SlotInfo,
    pub update_frame_index: u32,
    pub append_frame_index: u32,
    pub append_pos: i32,
    pub append_overflow: bool,
    pub num_slots: i32,
    pub active_slot: i32,
}
#[repr(C)]
pub struct ImageInfo {
    pub slot: SlotInfo,
    pub upd_frame_index: u32,
    pub num_slots: i32,
    pub active_slot: i32,
    pub width: i32,
    pub height: i32,
}
#[repr(C)]
pub struct ShaderInfo {
    pub slot: SlotInfo,
}
#[repr(C)]
pub struct PipelineInfo {
    pub slot: SlotInfo,
}
#[repr(C)]
pub struct PassInfo {
    pub slot: SlotInfo,
}
#[repr(C)]
pub struct GlContextDesc {
    pub force_gles2: bool,
}
#[repr(C)]
pub struct MetalContextDesc {
    pub device: *const std::ffi::c_void,
    pub renderpass_descriptor_cb: *const extern fn() -> *const std::ffi::c_void,
    pub renderpass_descriptor_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    pub drawable_cb: *const extern fn() -> *const std::ffi::c_void,
    pub drawable_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct D3d11ContextDesc {
    pub device: *const std::ffi::c_void,
    pub device_context: *const std::ffi::c_void,
    pub render_target_view_cb: *const extern fn() -> *const std::ffi::c_void,
    pub render_target_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    pub depth_stencil_view_cb: *const extern fn() -> *const std::ffi::c_void,
    pub depth_stencil_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct WgpuContextDesc {
    pub device: *const std::ffi::c_void,
    pub render_view_cb: *const extern fn() -> *const std::ffi::c_void,
    pub render_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    pub resolve_view_cb: *const extern fn() -> *const std::ffi::c_void,
    pub resolve_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    pub depth_stencil_view_cb: *const extern fn() -> *const std::ffi::c_void,
    pub depth_stencil_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct ContextDesc {
    pub color_format: i32,
    pub depth_format: i32,
    pub sample_count: i32,
    pub gl: GlContextDesc,
    pub metal: MetalContextDesc,
    pub d3d11: D3d11ContextDesc,
    pub wgpu: WgpuContextDesc,
}
#[repr(C)]
pub struct CommitListener {
    pub func: *const extern fn(*mut std::ffi::c_void) -> (),
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Allocator {
    pub alloc: *const extern fn(usize, *mut std::ffi::c_void) -> *mut std::ffi::c_void,
    pub free: *const extern fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Logger {
    pub log_cb: *const extern fn(*const u8, *mut std::ffi::c_void) -> (),
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Desc {
    pub _start_canary: u32,
    pub buffer_pool_size: i32,
    pub image_pool_size: i32,
    pub shader_pool_size: i32,
    pub pipeline_pool_size: i32,
    pub pass_pool_size: i32,
    pub context_pool_size: i32,
    pub uniform_buffer_size: i32,
    pub staging_buffer_size: i32,
    pub sampler_cache_size: i32,
    pub max_commit_listeners: i32,
    pub disable_validation: bool,
    pub allocator: Allocator,
    pub logger: Logger,
    pub context: ContextDesc,
    pub _end_canary: u32,
}
extern { pub fn sg_setup(desc: *const Desc) -> (); }
pub fn setup(desc: Desc) -> () { unsafe {
    return sg_setup(&desc);
} }
extern { pub fn sg_shutdown() -> (); }
pub fn shutdown() -> () { unsafe {
    return sg_shutdown();
} }
extern { pub fn sg_isvalid() -> bool; }
pub fn isvalid() -> bool { unsafe {
    return sg_isvalid();
} }
extern { pub fn sg_reset_state_cache() -> (); }
pub fn reset_state_cache() -> () { unsafe {
    return sg_reset_state_cache();
} }
extern { pub fn sg_push_debug_group(name: *const u8) -> (); }
pub fn push_debug_group(name: *const u8) -> () { unsafe {
    return sg_push_debug_group(name);
} }
extern { pub fn sg_pop_debug_group() -> (); }
pub fn pop_debug_group() -> () { unsafe {
    return sg_pop_debug_group();
} }
extern { pub fn sg_add_commit_listener(listener: CommitListener) -> bool; }
pub fn add_commit_listener(listener: CommitListener) -> bool { unsafe {
    return sg_add_commit_listener(listener);
} }
extern { pub fn sg_remove_commit_listener(listener: CommitListener) -> bool; }
pub fn remove_commit_listener(listener: CommitListener) -> bool { unsafe {
    return sg_remove_commit_listener(listener);
} }
extern { pub fn sg_make_buffer(desc: *const BufferDesc) -> Buffer; }
pub fn make_buffer(desc: BufferDesc) -> Buffer { unsafe {
    return sg_make_buffer(&desc);
} }
extern { pub fn sg_make_image(desc: *const ImageDesc) -> Image; }
pub fn make_image(desc: ImageDesc) -> Image { unsafe {
    return sg_make_image(&desc);
} }
extern { pub fn sg_make_shader(desc: *const ShaderDesc) -> Shader; }
pub fn make_shader(desc: ShaderDesc) -> Shader { unsafe {
    return sg_make_shader(&desc);
} }
extern { pub fn sg_make_pipeline(desc: *const PipelineDesc) -> Pipeline; }
pub fn make_pipeline(desc: PipelineDesc) -> Pipeline { unsafe {
    return sg_make_pipeline(&desc);
} }
extern { pub fn sg_make_pass(desc: *const PassDesc) -> Pass; }
pub fn make_pass(desc: PassDesc) -> Pass { unsafe {
    return sg_make_pass(&desc);
} }
extern { pub fn sg_destroy_buffer(buf: Buffer) -> (); }
pub fn destroy_buffer(buf: Buffer) -> () { unsafe {
    return sg_destroy_buffer(buf);
} }
extern { pub fn sg_destroy_image(img: Image) -> (); }
pub fn destroy_image(img: Image) -> () { unsafe {
    return sg_destroy_image(img);
} }
extern { pub fn sg_destroy_shader(shd: Shader) -> (); }
pub fn destroy_shader(shd: Shader) -> () { unsafe {
    return sg_destroy_shader(shd);
} }
extern { pub fn sg_destroy_pipeline(pip: Pipeline) -> (); }
pub fn destroy_pipeline(pip: Pipeline) -> () { unsafe {
    return sg_destroy_pipeline(pip);
} }
extern { pub fn sg_destroy_pass(pass: Pass) -> (); }
pub fn destroy_pass(pass: Pass) -> () { unsafe {
    return sg_destroy_pass(pass);
} }
extern { pub fn sg_update_buffer(buf: Buffer, data: *const Range) -> (); }
pub fn update_buffer(buf: Buffer, data: Range) -> () { unsafe {
    return sg_update_buffer(buf, &data);
} }
extern { pub fn sg_update_image(img: Image, data: *const ImageData) -> (); }
pub fn update_image(img: Image, data: ImageData) -> () { unsafe {
    return sg_update_image(img, &data);
} }
extern { pub fn sg_append_buffer(buf: Buffer, data: *const Range) -> i32; }
pub fn append_buffer(buf: Buffer, data: Range) -> i32 { unsafe {
    return sg_append_buffer(buf, &data);
} }
extern { pub fn sg_query_buffer_overflow(buf: Buffer) -> bool; }
pub fn query_buffer_overflow(buf: Buffer) -> bool { unsafe {
    return sg_query_buffer_overflow(buf);
} }
extern { pub fn sg_query_buffer_will_overflow(buf: Buffer, size: usize) -> bool; }
pub fn query_buffer_will_overflow(buf: Buffer, size: usize) -> bool { unsafe {
    return sg_query_buffer_will_overflow(buf, size);
} }
extern { pub fn sg_begin_default_pass(pass_action: *const PassAction, width: i32, height: i32) -> (); }
pub fn begin_default_pass(pass_action: PassAction, width: i32, height: i32) -> () { unsafe {
    return sg_begin_default_pass(&pass_action, width, height);
} }
extern { pub fn sg_begin_default_passf(pass_action: *const PassAction, width: f32, height: f32) -> (); }
pub fn begin_default_passf(pass_action: PassAction, width: f32, height: f32) -> () { unsafe {
    return sg_begin_default_passf(&pass_action, width, height);
} }
extern { pub fn sg_begin_pass(pass: Pass, pass_action: *const PassAction) -> (); }
pub fn begin_pass(pass: Pass, pass_action: PassAction) -> () { unsafe {
    return sg_begin_pass(pass, &pass_action);
} }
extern { pub fn sg_apply_viewport(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) -> (); }
pub fn apply_viewport(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) -> () { unsafe {
    return sg_apply_viewport(x, y, width, height, origin_top_left);
} }
extern { pub fn sg_apply_viewportf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) -> (); }
pub fn apply_viewportf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) -> () { unsafe {
    return sg_apply_viewportf(x, y, width, height, origin_top_left);
} }
extern { pub fn sg_apply_scissor_rect(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) -> (); }
pub fn apply_scissor_rect(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) -> () { unsafe {
    return sg_apply_scissor_rect(x, y, width, height, origin_top_left);
} }
extern { pub fn sg_apply_scissor_rectf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) -> (); }
pub fn apply_scissor_rectf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) -> () { unsafe {
    return sg_apply_scissor_rectf(x, y, width, height, origin_top_left);
} }
extern { pub fn sg_apply_pipeline(pip: Pipeline) -> (); }
pub fn apply_pipeline(pip: Pipeline) -> () { unsafe {
    return sg_apply_pipeline(pip);
} }
extern { pub fn sg_apply_bindings(bindings: *const Bindings) -> (); }
pub fn apply_bindings(bindings: Bindings) -> () { unsafe {
    return sg_apply_bindings(&bindings);
} }
extern { pub fn sg_apply_uniforms(stage: ShaderStage, ub_index: u32, data: *const Range) -> (); }
pub fn apply_uniforms(stage: ShaderStage, ub_index: u32, data: Range) -> () { unsafe {
    return sg_apply_uniforms(stage, ub_index, &data);
} }
extern { pub fn sg_draw(base_element: u32, num_elements: u32, num_instances: u32) -> (); }
pub fn draw(base_element: u32, num_elements: u32, num_instances: u32) -> () { unsafe {
    return sg_draw(base_element, num_elements, num_instances);
} }
extern { pub fn sg_end_pass() -> (); }
pub fn end_pass() -> () { unsafe {
    return sg_end_pass();
} }
extern { pub fn sg_commit() -> (); }
pub fn commit() -> () { unsafe {
    return sg_commit();
} }
extern { pub fn sg_query_desc() -> Desc; }
pub fn query_desc() -> Desc { unsafe {
    return sg_query_desc();
} }
extern { pub fn sg_query_backend() -> Backend; }
pub fn query_backend() -> Backend { unsafe {
    return sg_query_backend();
} }
extern { pub fn sg_query_features() -> Features; }
pub fn query_features() -> Features { unsafe {
    return sg_query_features();
} }
extern { pub fn sg_query_limits() -> Limits; }
pub fn query_limits() -> Limits { unsafe {
    return sg_query_limits();
} }
extern { pub fn sg_query_pixelformat(fmt: PixelFormat) -> PixelformatInfo; }
pub fn query_pixelformat(fmt: PixelFormat) -> PixelformatInfo { unsafe {
    return sg_query_pixelformat(fmt);
} }
extern { pub fn sg_query_buffer_state(buf: Buffer) -> ResourceState; }
pub fn query_buffer_state(buf: Buffer) -> ResourceState { unsafe {
    return sg_query_buffer_state(buf);
} }
extern { pub fn sg_query_image_state(img: Image) -> ResourceState; }
pub fn query_image_state(img: Image) -> ResourceState { unsafe {
    return sg_query_image_state(img);
} }
extern { pub fn sg_query_shader_state(shd: Shader) -> ResourceState; }
pub fn query_shader_state(shd: Shader) -> ResourceState { unsafe {
    return sg_query_shader_state(shd);
} }
extern { pub fn sg_query_pipeline_state(pip: Pipeline) -> ResourceState; }
pub fn query_pipeline_state(pip: Pipeline) -> ResourceState { unsafe {
    return sg_query_pipeline_state(pip);
} }
extern { pub fn sg_query_pass_state(pass: Pass) -> ResourceState; }
pub fn query_pass_state(pass: Pass) -> ResourceState { unsafe {
    return sg_query_pass_state(pass);
} }
extern { pub fn sg_query_buffer_info(buf: Buffer) -> BufferInfo; }
pub fn query_buffer_info(buf: Buffer) -> BufferInfo { unsafe {
    return sg_query_buffer_info(buf);
} }
extern { pub fn sg_query_image_info(img: Image) -> ImageInfo; }
pub fn query_image_info(img: Image) -> ImageInfo { unsafe {
    return sg_query_image_info(img);
} }
extern { pub fn sg_query_shader_info(shd: Shader) -> ShaderInfo; }
pub fn query_shader_info(shd: Shader) -> ShaderInfo { unsafe {
    return sg_query_shader_info(shd);
} }
extern { pub fn sg_query_pipeline_info(pip: Pipeline) -> PipelineInfo; }
pub fn query_pipeline_info(pip: Pipeline) -> PipelineInfo { unsafe {
    return sg_query_pipeline_info(pip);
} }
extern { pub fn sg_query_pass_info(pass: Pass) -> PassInfo; }
pub fn query_pass_info(pass: Pass) -> PassInfo { unsafe {
    return sg_query_pass_info(pass);
} }
extern { pub fn sg_query_buffer_defaults(desc: *const BufferDesc) -> BufferDesc; }
pub fn query_buffer_defaults(desc: BufferDesc) -> BufferDesc { unsafe {
    return sg_query_buffer_defaults(&desc);
} }
extern { pub fn sg_query_image_defaults(desc: *const ImageDesc) -> ImageDesc; }
pub fn query_image_defaults(desc: ImageDesc) -> ImageDesc { unsafe {
    return sg_query_image_defaults(&desc);
} }
extern { pub fn sg_query_shader_defaults(desc: *const ShaderDesc) -> ShaderDesc; }
pub fn query_shader_defaults(desc: ShaderDesc) -> ShaderDesc { unsafe {
    return sg_query_shader_defaults(&desc);
} }
extern { pub fn sg_query_pipeline_defaults(desc: *const PipelineDesc) -> PipelineDesc; }
pub fn query_pipeline_defaults(desc: PipelineDesc) -> PipelineDesc { unsafe {
    return sg_query_pipeline_defaults(&desc);
} }
extern { pub fn sg_query_pass_defaults(desc: *const PassDesc) -> PassDesc; }
pub fn query_pass_defaults(desc: PassDesc) -> PassDesc { unsafe {
    return sg_query_pass_defaults(&desc);
} }
extern { pub fn sg_alloc_buffer() -> Buffer; }
pub fn alloc_buffer() -> Buffer { unsafe {
    return sg_alloc_buffer();
} }
extern { pub fn sg_alloc_image() -> Image; }
pub fn alloc_image() -> Image { unsafe {
    return sg_alloc_image();
} }
extern { pub fn sg_alloc_shader() -> Shader; }
pub fn alloc_shader() -> Shader { unsafe {
    return sg_alloc_shader();
} }
extern { pub fn sg_alloc_pipeline() -> Pipeline; }
pub fn alloc_pipeline() -> Pipeline { unsafe {
    return sg_alloc_pipeline();
} }
extern { pub fn sg_alloc_pass() -> Pass; }
pub fn alloc_pass() -> Pass { unsafe {
    return sg_alloc_pass();
} }
extern { pub fn sg_dealloc_buffer(buf: Buffer) -> (); }
pub fn dealloc_buffer(buf: Buffer) -> () { unsafe {
    return sg_dealloc_buffer(buf);
} }
extern { pub fn sg_dealloc_image(img: Image) -> (); }
pub fn dealloc_image(img: Image) -> () { unsafe {
    return sg_dealloc_image(img);
} }
extern { pub fn sg_dealloc_shader(shd: Shader) -> (); }
pub fn dealloc_shader(shd: Shader) -> () { unsafe {
    return sg_dealloc_shader(shd);
} }
extern { pub fn sg_dealloc_pipeline(pip: Pipeline) -> (); }
pub fn dealloc_pipeline(pip: Pipeline) -> () { unsafe {
    return sg_dealloc_pipeline(pip);
} }
extern { pub fn sg_dealloc_pass(pass: Pass) -> (); }
pub fn dealloc_pass(pass: Pass) -> () { unsafe {
    return sg_dealloc_pass(pass);
} }
extern { pub fn sg_init_buffer(buf: Buffer, desc: *const BufferDesc) -> (); }
pub fn init_buffer(buf: Buffer, desc: BufferDesc) -> () { unsafe {
    return sg_init_buffer(buf, &desc);
} }
extern { pub fn sg_init_image(img: Image, desc: *const ImageDesc) -> (); }
pub fn init_image(img: Image, desc: ImageDesc) -> () { unsafe {
    return sg_init_image(img, &desc);
} }
extern { pub fn sg_init_shader(shd: Shader, desc: *const ShaderDesc) -> (); }
pub fn init_shader(shd: Shader, desc: ShaderDesc) -> () { unsafe {
    return sg_init_shader(shd, &desc);
} }
extern { pub fn sg_init_pipeline(pip: Pipeline, desc: *const PipelineDesc) -> (); }
pub fn init_pipeline(pip: Pipeline, desc: PipelineDesc) -> () { unsafe {
    return sg_init_pipeline(pip, &desc);
} }
extern { pub fn sg_init_pass(pass: Pass, desc: *const PassDesc) -> (); }
pub fn init_pass(pass: Pass, desc: PassDesc) -> () { unsafe {
    return sg_init_pass(pass, &desc);
} }
extern { pub fn sg_uninit_buffer(buf: Buffer) -> (); }
pub fn uninit_buffer(buf: Buffer) -> () { unsafe {
    return sg_uninit_buffer(buf);
} }
extern { pub fn sg_uninit_image(img: Image) -> (); }
pub fn uninit_image(img: Image) -> () { unsafe {
    return sg_uninit_image(img);
} }
extern { pub fn sg_uninit_shader(shd: Shader) -> (); }
pub fn uninit_shader(shd: Shader) -> () { unsafe {
    return sg_uninit_shader(shd);
} }
extern { pub fn sg_uninit_pipeline(pip: Pipeline) -> (); }
pub fn uninit_pipeline(pip: Pipeline) -> () { unsafe {
    return sg_uninit_pipeline(pip);
} }
extern { pub fn sg_uninit_pass(pass: Pass) -> (); }
pub fn uninit_pass(pass: Pass) -> () { unsafe {
    return sg_uninit_pass(pass);
} }
extern { pub fn sg_fail_buffer(buf: Buffer) -> (); }
pub fn fail_buffer(buf: Buffer) -> () { unsafe {
    return sg_fail_buffer(buf);
} }
extern { pub fn sg_fail_image(img: Image) -> (); }
pub fn fail_image(img: Image) -> () { unsafe {
    return sg_fail_image(img);
} }
extern { pub fn sg_fail_shader(shd: Shader) -> (); }
pub fn fail_shader(shd: Shader) -> () { unsafe {
    return sg_fail_shader(shd);
} }
extern { pub fn sg_fail_pipeline(pip: Pipeline) -> (); }
pub fn fail_pipeline(pip: Pipeline) -> () { unsafe {
    return sg_fail_pipeline(pip);
} }
extern { pub fn sg_fail_pass(pass: Pass) -> (); }
pub fn fail_pass(pass: Pass) -> () { unsafe {
    return sg_fail_pass(pass);
} }
extern { pub fn sg_setup_context() -> Context; }
pub fn setup_context() -> Context { unsafe {
    return sg_setup_context();
} }
extern { pub fn sg_activate_context(ctx_id: Context) -> (); }
pub fn activate_context(ctx_id: Context) -> () { unsafe {
    return sg_activate_context(ctx_id);
} }
extern { pub fn sg_discard_context(ctx_id: Context) -> (); }
pub fn discard_context(ctx_id: Context) -> () { unsafe {
    return sg_discard_context(ctx_id);
} }
extern { pub fn sg_d3d11_device() -> *const std::ffi::c_void; }
pub fn d3d11_device() -> *const std::ffi::c_void { unsafe {
    return sg_d3d11_device();
} }
extern { pub fn sg_mtl_device() -> *const std::ffi::c_void; }
pub fn mtl_device() -> *const std::ffi::c_void { unsafe {
    return sg_mtl_device();
} }
extern { pub fn sg_mtl_render_command_encoder() -> *const std::ffi::c_void; }
pub fn mtl_render_command_encoder() -> *const std::ffi::c_void { unsafe {
    return sg_mtl_render_command_encoder();
} }
