// machine generated, do not edit


#[repr(C)]
pub struct Buffer {
    id: u32,
}
#[repr(C)]
pub struct Image {
    id: u32,
}
#[repr(C)]
pub struct Shader {
    id: u32,
}
#[repr(C)]
pub struct Pipeline {
    id: u32,
}
#[repr(C)]
pub struct Pass {
    id: u32,
}
#[repr(C)]
pub struct Context {
    id: u32,
}
#[repr(C)]
pub struct Range {
    ptr: *const std::ffi::c_void,
    size: usize,
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
    r: f32,
    g: f32,
    b: f32,
    a: f32,
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
    sample: bool,
    filter: bool,
    render: bool,
    blend: bool,
    msaa: bool,
    depth: bool,
    __pad: [u32; 3],
}
#[repr(C)]
pub struct Features {
    instancing: bool,
    origin_top_left: bool,
    multiple_render_targets: bool,
    msaa_render_targets: bool,
    imagetype_3d: bool,
    imagetype_array: bool,
    image_clamp_to_border: bool,
    mrt_independent_blend_state: bool,
    mrt_independent_write_mask: bool,
    __pad: [u32; 3],
}
#[repr(C)]
pub struct Limits {
    max_image_size_2d: i32,
    max_image_size_cube: i32,
    max_image_size_3d: i32,
    max_image_size_array: i32,
    max_image_array_layers: i32,
    max_vertex_attrs: i32,
    gl_max_vertex_uniform_vectors: i32,
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
    action: Action,
    value: Color,
}
#[repr(C)]
pub struct DepthAttachmentAction {
    action: Action,
    value: f32,
}
#[repr(C)]
pub struct StencilAttachmentAction {
    action: Action,
    value: u8,
}
#[repr(C)]
pub struct PassAction {
    _start_canary: u32,
    colors: [ColorAttachmentAction; 4],
    depth: DepthAttachmentAction,
    stencil: StencilAttachmentAction,
    _end_canary: u32,
}
#[repr(C)]
pub struct Bindings {
    _start_canary: u32,
    vertex_buffers: [Buffer; 8],
    vertex_buffer_offsets: [i32; 8],
    index_buffer: Buffer,
    index_buffer_offset: i32,
    vs_images: [Image; 12],
    fs_images: [Image; 12],
    _end_canary: u32,
}
#[repr(C)]
pub struct BufferDesc {
    _start_canary: u32,
    size: usize,
    r#type: BufferType,
    usage: Usage,
    data: Range,
    label: *mut u8,
    gl_buffers: [u32; 2],
    mtl_buffers: [*const std::ffi::c_void; 2],
    d3d11_buffer: *const std::ffi::c_void,
    wgpu_buffer: *const std::ffi::c_void,
    _end_canary: u32,
}
#[repr(C)]
pub struct ImageData {
    subimage: [[Range; 6]; 16],
}
#[repr(C)]
pub struct ImageDesc {
    _start_canary: u32,
    r#type: ImageType,
    render_target: bool,
    width: i32,
    height: i32,
    num_slices: i32,
    num_mipmaps: i32,
    usage: Usage,
    pixel_format: PixelFormat,
    sample_count: i32,
    min_filter: Filter,
    mag_filter: Filter,
    wrap_u: Wrap,
    wrap_v: Wrap,
    wrap_w: Wrap,
    border_color: BorderColor,
    max_anisotropy: u32,
    min_lod: f32,
    max_lod: f32,
    data: ImageData,
    label: *mut u8,
    gl_textures: [u32; 2],
    gl_texture_target: u32,
    mtl_textures: [*const std::ffi::c_void; 2],
    d3d11_texture: *const std::ffi::c_void,
    d3d11_shader_resource_view: *const std::ffi::c_void,
    wgpu_texture: *const std::ffi::c_void,
    _end_canary: u32,
}
#[repr(C)]
pub struct ShaderAttrDesc {
    name: *mut u8,
    sem_name: *mut u8,
    sem_index: i32,
}
#[repr(C)]
pub struct ShaderUniformDesc {
    name: *mut u8,
    r#type: UniformType,
    array_count: i32,
}
#[repr(C)]
pub struct ShaderUniformBlockDesc {
    size: usize,
    layout: UniformLayout,
    uniforms: [ShaderUniformDesc; 16],
}
#[repr(C)]
pub struct ShaderImageDesc {
    name: *mut u8,
    image_type: ImageType,
    sampler_type: SamplerType,
}
#[repr(C)]
pub struct ShaderStageDesc {
    source: *mut u8,
    bytecode: Range,
    entry: *mut u8,
    d3d11_target: *mut u8,
    uniform_blocks: [ShaderUniformBlockDesc; 4],
    images: [ShaderImageDesc; 12],
}
#[repr(C)]
pub struct ShaderDesc {
    _start_canary: u32,
    attrs: [ShaderAttrDesc; 16],
    vs: ShaderStageDesc,
    fs: ShaderStageDesc,
    label: *mut u8,
    _end_canary: u32,
}
#[repr(C)]
pub struct BufferLayoutDesc {
    stride: i32,
    step_func: VertexStep,
    step_rate: i32,
    __pad: [u32; 2],
}
#[repr(C)]
pub struct VertexAttrDesc {
    buffer_index: i32,
    offset: i32,
    format: VertexFormat,
    __pad: [u32; 2],
}
#[repr(C)]
pub struct LayoutDesc {
    buffers: [BufferLayoutDesc; 8],
    attrs: [VertexAttrDesc; 16],
}
#[repr(C)]
pub struct StencilFaceState {
    compare: CompareFunc,
    fail_op: StencilOp,
    depth_fail_op: StencilOp,
    pass_op: StencilOp,
}
#[repr(C)]
pub struct StencilState {
    enabled: bool,
    front: StencilFaceState,
    back: StencilFaceState,
    read_mask: u8,
    write_mask: u8,
    r#ref: u8,
}
#[repr(C)]
pub struct DepthState {
    pixel_format: PixelFormat,
    compare: CompareFunc,
    write_enabled: bool,
    bias: f32,
    bias_slope_scale: f32,
    bias_clamp: f32,
}
#[repr(C)]
pub struct BlendState {
    enabled: bool,
    src_factor_rgb: BlendFactor,
    dst_factor_rgb: BlendFactor,
    op_rgb: BlendOp,
    src_factor_alpha: BlendFactor,
    dst_factor_alpha: BlendFactor,
    op_alpha: BlendOp,
}
#[repr(C)]
pub struct ColorState {
    pixel_format: PixelFormat,
    write_mask: ColorMask,
    blend: BlendState,
}
#[repr(C)]
pub struct PipelineDesc {
    _start_canary: u32,
    shader: Shader,
    layout: LayoutDesc,
    depth: DepthState,
    stencil: StencilState,
    color_count: i32,
    colors: [ColorState; 4],
    primitive_type: PrimitiveType,
    index_type: IndexType,
    cull_mode: CullMode,
    face_winding: FaceWinding,
    sample_count: i32,
    blend_color: Color,
    alpha_to_coverage_enabled: bool,
    label: *mut u8,
    _end_canary: u32,
}
#[repr(C)]
pub struct PassAttachmentDesc {
    image: Image,
    mip_level: i32,
    slice: i32,
}
#[repr(C)]
pub struct PassDesc {
    _start_canary: u32,
    color_attachments: [PassAttachmentDesc; 4],
    depth_stencil_attachment: PassAttachmentDesc,
    label: *mut u8,
    _end_canary: u32,
}
#[repr(C)]
pub struct SlotInfo {
    state: ResourceState,
    res_id: u32,
    ctx_id: u32,
}
#[repr(C)]
pub struct BufferInfo {
    slot: SlotInfo,
    update_frame_index: u32,
    append_frame_index: u32,
    append_pos: i32,
    append_overflow: bool,
    num_slots: i32,
    active_slot: i32,
}
#[repr(C)]
pub struct ImageInfo {
    slot: SlotInfo,
    upd_frame_index: u32,
    num_slots: i32,
    active_slot: i32,
    width: i32,
    height: i32,
}
#[repr(C)]
pub struct ShaderInfo {
    slot: SlotInfo,
}
#[repr(C)]
pub struct PipelineInfo {
    slot: SlotInfo,
}
#[repr(C)]
pub struct PassInfo {
    slot: SlotInfo,
}
#[repr(C)]
pub struct GlContextDesc {
    force_gles2: bool,
}
#[repr(C)]
pub struct MetalContextDesc {
    device: *const std::ffi::c_void,
    renderpass_descriptor_cb: *const extern fn() -> *const std::ffi::c_void,
    renderpass_descriptor_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    drawable_cb: *const extern fn() -> *const std::ffi::c_void,
    drawable_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct D3d11ContextDesc {
    device: *const std::ffi::c_void,
    device_context: *const std::ffi::c_void,
    render_target_view_cb: *const extern fn() -> *const std::ffi::c_void,
    render_target_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    depth_stencil_view_cb: *const extern fn() -> *const std::ffi::c_void,
    depth_stencil_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct WgpuContextDesc {
    device: *const std::ffi::c_void,
    render_view_cb: *const extern fn() -> *const std::ffi::c_void,
    render_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    resolve_view_cb: *const extern fn() -> *const std::ffi::c_void,
    resolve_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    depth_stencil_view_cb: *const extern fn() -> *const std::ffi::c_void,
    depth_stencil_view_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> *const std::ffi::c_void,
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct ContextDesc {
    color_format: i32,
    depth_format: i32,
    sample_count: i32,
    gl: GlContextDesc,
    metal: MetalContextDesc,
    d3d11: D3d11ContextDesc,
    wgpu: WgpuContextDesc,
}
#[repr(C)]
pub struct CommitListener {
    func: *const extern fn(*mut std::ffi::c_void) -> (),
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Allocator {
    alloc: *const extern fn(usize, *mut std::ffi::c_void) -> *mut std::ffi::c_void,
    free: *const extern fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Logger {
    log_cb: *const extern fn(*const u8, *mut std::ffi::c_void) -> (),
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Desc {
    _start_canary: u32,
    buffer_pool_size: i32,
    image_pool_size: i32,
    shader_pool_size: i32,
    pipeline_pool_size: i32,
    pass_pool_size: i32,
    context_pool_size: i32,
    uniform_buffer_size: i32,
    staging_buffer_size: i32,
    sampler_cache_size: i32,
    max_commit_listeners: i32,
    disable_validation: bool,
    allocator: Allocator,
    logger: Logger,
    context: ContextDesc,
    _end_canary: u32,
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
