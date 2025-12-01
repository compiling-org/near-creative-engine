// Stabilized WebGPU Pipeline Implementation
// Based on wgpu-compute-toy reference patterns with robust error handling

class StabilizedWebGPUPipeline {
    constructor() {
        this.device = null;
        this.context = null;
        this.pipeline = null;
        this.bindGroup = null;
        this.uniformBuffer = null;
        this.commandEncoder = null;
        this.initialized = false;
        this.errorHandler = null;
    }

    async initialize(canvas, options = {}) {
        try {
            const { 
                powerPreference = 'high-performance',
                forceFallbackAdapter = false,
                validation = true 
            } = options;

            // Check WebGPU support
            if (!navigator.gpu) {
                throw new Error('WebGPU not supported in this browser');
            }

            // Request adapter with robust error handling
            const adapter = await navigator.gpu.requestAdapter({
                powerPreference,
                forceFallbackAdapter
            });

            if (!adapter) {
                throw new Error('Failed to get WebGPU adapter');
            }

            // Request device with validation and error handling
            const deviceDescriptor = {
                requiredFeatures: [],
                requiredLimits: {
                    maxComputeWorkgroupSizeX: 256,
                    maxComputeWorkgroupSizeY: 256,
                    maxComputeWorkgroupSizeZ: 64,
                    maxComputeInvocationsPerWorkgroup: 256,
                    maxComputeWorkgroupStorageSize: 16384,
                    maxComputeSharedStorageSize: 32768,
                }
            };

            if (validation) {
                deviceDescriptor.requiredFeatures.push('timestamp-query');
            }

            this.device = await adapter.requestDevice(deviceDescriptor);

            // Set up error handling
            this.device.lost.then((info) => {
                console.error(`WebGPU device lost: ${info.reason}`, info.message);
                this.initialized = false;
                if (this.errorHandler) {
                    this.errorHandler(info);
                }
            });

            // Configure canvas context
            this.context = canvas.getContext('webgpu');
            if (!this.context) {
                throw new Error('Failed to get WebGPU canvas context');
            }

            const canvasFormat = navigator.gpu.getPreferredCanvasFormat();
            this.context.configure({
                device: this.device,
                format: canvasFormat,
                alphaMode: 'premultiplied',
                usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC
            });

            this.canvasFormat = canvasFormat;
            this.initialized = true;

            console.log('✅ WebGPU pipeline initialized successfully');
            return true;

        } catch (error) {
            console.error('❌ WebGPU initialization failed:', error);
            this.initialized = false;
            throw error;
        }
    }

    createUniformBuffer(uniformData, label = 'uniforms') {
        if (!this.initialized) {
            throw new Error('WebGPU pipeline not initialized');
        }

        try {
            // Create uniform buffer with proper alignment
            const uniformBufferSize = Math.max(uniformData.byteLength, 256); // 256-byte alignment
            
            this.uniformBuffer = this.device.createBuffer({
                label: `${label}-buffer`,
                size: uniformBufferSize,
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
                mappedAtCreation: true,
            });

            // Copy uniform data
            const uniformArray = new Float32Array(this.uniformBuffer.getMappedRange());
            uniformArray.set(new Float32Array(uniformData.buffer));
            this.uniformBuffer.unmap();

            console.log(`✅ Uniform buffer created: ${uniformBufferSize} bytes`);
            return this.uniformBuffer;

        } catch (error) {
            console.error('❌ Failed to create uniform buffer:', error);
            throw error;
        }
    }

    createComputePipeline(computeShaderCode, label = 'compute') {
        if (!this.initialized) {
            throw new Error('WebGPU pipeline not initialized');
        }

        try {
            // Create compute shader module
            const computeModule = this.device.createShaderModule({
                label: `${label}-compute-module`,
                code: computeShaderCode,
            });

            // Create compute pipeline with robust layout
            this.pipeline = this.device.createComputePipeline({
                label: `${label}-compute-pipeline`,
                layout: 'auto',
                compute: {
                    module: computeModule,
                    entryPoint: 'main',
                },
            });

            console.log('✅ Compute pipeline created successfully');
            return this.pipeline;

        } catch (error) {
            console.error('❌ Failed to create compute pipeline:', error);
            throw error;
        }
    }

    createRenderPipeline(vertexShaderCode, fragmentShaderCode, options = {}) {
        if (!this.initialized) {
            throw new Error('WebGPU pipeline not initialized');
        }

        try {
            const { 
                topology = 'triangle-list',
                cullMode = 'back',
                frontFace = 'ccw',
                depthCompare = 'less',
                depthWriteEnabled = true 
            } = options;

            // Create shader modules
            const vertexModule = this.device.createShaderModule({
                label: 'vertex-module',
                code: vertexShaderCode,
            });

            const fragmentModule = this.device.createShaderModule({
                label: 'fragment-module',
                code: fragmentShaderCode,
            });

            // Create render pipeline with robust configuration
            this.pipeline = this.device.createRenderPipeline({
                label: 'render-pipeline',
                layout: 'auto',
                vertex: {
                    module: vertexModule,
                    entryPoint: 'vs_main',
                    buffers: [
                        {
                            arrayStride: 24, // 3 * 8 bytes for position + uv + normal
                            attributes: [
                                { shaderLocation: 0, offset: 0, format: 'float32x3' }, // position
                                { shaderLocation: 1, offset: 12, format: 'float32x2' }, // uv
                                { shaderLocation: 2, offset: 20, format: 'float32x3' }, // normal
                            ],
                        },
                    ],
                },
                fragment: {
                    module: fragmentModule,
                    entryPoint: 'fs_main',
                    targets: [
                        {
                            format: this.canvasFormat,
                            blend: {
                                color: {
                                    srcFactor: 'src-alpha',
                                    dstFactor: 'one-minus-src-alpha',
                                    operation: 'add',
                                },
                                alpha: {
                                    srcFactor: 'one',
                                    dstFactor: 'one-minus-src-alpha',
                                    operation: 'add',
                                },
                            },
                        },
                    ],
                },
                primitive: {
                    topology,
                    cullMode,
                    frontFace,
                },
                depthStencil: {
                    depthWriteEnabled,
                    depthCompare,
                    format: 'depth24plus',
                },
            });

            console.log('✅ Render pipeline created successfully');
            return this.pipeline;

        } catch (error) {
            console.error('❌ Failed to create render pipeline:', error);
            throw error;
        }
    }

    createBindGroup(resources, label = 'bind-group') {
        if (!this.initialized) {
            throw new Error('WebGPU pipeline not initialized');
        }

        try {
            const entries = [];
            
            resources.forEach((resource, index) => {
                if (resource.type === 'uniform') {
                    entries.push({
                        binding: index,
                        resource: { buffer: resource.buffer },
                    });
                } else if (resource.type === 'texture') {
                    entries.push({
                        binding: index,
                        resource: resource.texture.createView(),
                    });
                } else if (resource.type === 'sampler') {
                    entries.push({
                        binding: index,
                        resource: resource.sampler,
                    });
                }
            });

            this.bindGroup = this.device.createBindGroup({
                label,
                layout: this.pipeline.getBindGroupLayout(0),
                entries,
            });

            console.log('✅ Bind group created successfully');
            return this.bindGroup;

        } catch (error) {
            console.error('❌ Failed to create bind group:', error);
            throw error;
        }
    }

    updateUniforms(uniformData, offset = 0) {
        if (!this.uniformBuffer) {
            throw new Error('Uniform buffer not created');
        }

        try {
            this.device.queue.writeBuffer(
                this.uniformBuffer,
                offset,
                uniformData.buffer,
                uniformData.byteOffset,
                uniformData.byteLength
            );
        } catch (error) {
            console.error('❌ Failed to update uniforms:', error);
            throw error;
        }
    }

    createCommandEncoder(label = 'command-encoder') {
        if (!this.initialized) {
            throw new Error('WebGPU pipeline not initialized');
        }

        this.commandEncoder = this.device.createCommandEncoder({ label });
        return this.commandEncoder;
    }

    beginComputePass(label = 'compute-pass') {
        if (!this.commandEncoder) {
            throw new Error('Command encoder not created');
        }

        return this.commandEncoder.beginComputePass({ label });
    }

    beginRenderPass(renderPassDescriptor, label = 'render-pass') {
        if (!this.commandEncoder) {
            throw new Error('Command encoder not created');
        }

        return this.commandEncoder.beginRenderPass({
            label,
            ...renderPassDescriptor,
        });
    }

    submitCommands() {
        if (!this.commandEncoder) {
            throw new Error('Command encoder not created');
        }

        try {
            const commandBuffer = this.commandEncoder.finish();
            this.device.queue.submit([commandBuffer]);
            this.commandEncoder = null; // Reset for next frame
            
            console.log('✅ Commands submitted successfully');
        } catch (error) {
            console.error('❌ Failed to submit commands:', error);
            throw error;
        }
    }

    setErrorHandler(handler) {
        this.errorHandler = handler;
    }

    destroy() {
        if (this.uniformBuffer) {
            this.uniformBuffer.destroy();
        }
        if (this.pipeline) {
            // Pipeline destruction is handled automatically
        }
        if (this.device) {
            this.device.destroy();
        }
        this.initialized = false;
        console.log('✅ WebGPU pipeline destroyed');
    }
}

// Emotional fractal shader templates
const EmotionalFractalShaders = {
    vertex: `
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Full-screen triangle
    if (vertex_index == 0) {
        out.position = vec4<f32>(-1.0, -1.0, 0.0, 1.0);
        out.uv = vec2<f32>(0.0, 0.0);
    } else if (vertex_index == 1) {
        out.position = vec4<f32>(3.0, -1.0, 0.0, 1.0);
        out.uv = vec2<f32>(2.0, 0.0);
    } else {
        out.position = vec4<f32>(-1.0, 3.0, 0.0, 1.0);
        out.uv = vec2<f32>(0.0, 2.0);
    }
    
    return out;
}`,

    fragment: `
struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
    valence: f32,      // Emotional parameter: -1 to 1
    arousal: f32,      // Emotional parameter: 0 to 1
    dominance: f32,    // Emotional parameter: 0 to 1
    zoom: f32,
    iterations: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Normalize UV coordinates
    let normalized_uv = (uv - 0.5) * 2.0;
    let aspect_ratio = uniforms.resolution.x / uniforms.resolution.y;
    let coord = vec2<f32>(normalized_uv.x * aspect_ratio, normalized_uv.y);
    
    // Emotional modulation parameters
    let time_mod = uniforms.time * (1.0 + uniforms.arousal * 0.5);
    let color_shift = uniforms.valence * 0.3;
    let complexity = uniforms.dominance * 10.0 + 5.0;
    let zoom_factor = uniforms.zoom;
    
    // Mandelbrot set with emotional modulation
    var c = coord * zoom_factor;
    c.x += sin(time_mod * 0.1) * uniforms.arousal * 0.1;
    c.y += cos(time_mod * 0.15) * uniforms.dominance * 0.1;
    
    var z = vec2<f32>(0.0, 0.0);
    var iter = 0.0;
    
    for (var i = 0; i < i32(complexity); i = i + 1) {
        if (length(z) > 2.0) {
            break;
        }
        
        // Emotional influence on iteration
        let emotional_factor = 1.0 + uniforms.arousal * sin(f32(i) * 0.1);
        
        z = vec2<f32>(
            z.x * z.x - z.y * z.y + c.x * emotional_factor,
            2.0 * z.x * z.y + c.y * emotional_factor
        );
        
        iter = f32(i);
    }
    
    // Color based on iteration count and emotional state
    let smooth_iter = iter + 1.0 - log(log(length(z))) / log(2.0);
    let normalized_iter = smooth_iter / complexity;
    
    // Emotional color mapping
    var color = vec3<f32>(
        normalized_iter * (1.0 + color_shift),
        normalized_iter * uniforms.arousal * 0.8,
        (1.0 - normalized_iter) * (1.0 + uniforms.dominance * 0.2)
    );
    
    // Add emotional sparkle effect
    let sparkle = sin(time_mod * 15.0 + coord.x * 50.0) * 
                  sin(time_mod * 13.0 + coord.y * 40.0) * 
                  uniforms.arousal * 0.1;
    
    color = color + vec3<f32>(sparkle);
    
    // Apply gamma correction
    color = pow(color, vec3<f32>(0.8));
    
    return vec4<f32>(color, 1.0);
}`
};

// Export for use
window.StabilizedWebGPUPipeline = StabilizedWebGPUPipeline;
window.EmotionalFractalShaders = EmotionalFractalShaders;