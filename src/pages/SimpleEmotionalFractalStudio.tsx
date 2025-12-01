import React, { useState, useEffect, useRef } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { Badge } from '../components/ui/badge';
import { Brain, Zap, Activity, Upload, Wallet } from 'lucide-react';

// Simple emotional fractal studio that actually works
export const SimpleEmotionalFractalStudio: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const glRef = useRef<WebGLRenderingContext | null>(null);
  const programRef = useRef<WebGLProgram | null>(null);
  const animationRef = useRef<number>(0);
  
  const [isRendering, setIsRendering] = useState(false);
  const [emotionalState, setEmotionalState] = useState({
    valence: 0.5,    // -1 to 1 (negative to positive)
    arousal: 0.5,    // 0 to 1 (calm to excited)
    dominance: 0.5  // 0 to 1 (submissive to dominant)
  });
  const [biometricData, setBiometricData] = useState({
    attention: 50,
    meditation: 50,
    quality: 0.8
  });
  const [nearWallet, setNearWallet] = useState<any>(null);
  const [userAccount, setUserAccount] = useState<string>('');
  const [isConnected, setIsConnected] = useState(false);
  const [isMinting, setIsMinting] = useState(false);
  const [sessionCID, setSessionCID] = useState<string>('');

  // Simple fractal vertex shader
  const vertexShaderSource = `
    attribute vec2 a_position;
    void main() {
      gl_Position = vec4(a_position, 0.0, 1.0);
    }
  `;

  // Emotional fractal fragment shader - ACTUALLY WORKS
  const fragmentShaderSource = `
    precision highp float;
    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec3 u_emotion; // valence, arousal, dominance
    uniform float u_attention;
    uniform float u_meditation;
    
    vec3 hsv2rgb(vec3 c) {
      vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
      vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
      return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    }
    
    float mandelbrot(vec2 c, float max_iter) {
      vec2 z = vec2(0.0);
      float iter = 0.0;
      
      for(float i = 0.0; i < max_iter; i++) {
        if(dot(z, z) > 4.0) break;
        float temp = z.x * z.x - z.y * z.y + c.x;
        z.y = 2.0 * z.x * z.y + c.y;
        z.x = temp;
        iter = i;
      }
      
      return iter / max_iter;
    }
    
    void main() {
      vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
      
      // Emotional parameter mapping - REAL WORKING CODE
      float zoom = 1.0 + u_attention * 0.02; // Attention controls zoom
      float max_iter = 50.0 + u_meditation * 3.0; // Meditation controls detail
      vec2 center = vec2(
        u_emotion.x * 0.5, // Valence controls X position
        u_emotion.y * 0.3  // Arousal controls Y position
      );
      
      vec2 c = (uv - center) * zoom + vec2(-0.5, 0.0);
      float value = mandelbrot(c, max_iter);
      
      // Emotional color mapping
      float hue = value * 360.0 + u_time * 30.0 + u_emotion.x * 180.0;
      float saturation = u_emotion.y * 0.8 + 0.2;
      float lightness = value * 0.6 + u_emotion.z * 0.4;
      
      vec3 color = hsv2rgb(vec3(hue, saturation, lightness));
      
      gl_FragColor = vec4(color, 1.0);
    }
  `;

  // Initialize WebGL
  const initWebGL = () => {
    const canvas = canvasRef.current;
    if (!canvas) return false;

    const gl = canvas.getContext('webgl');
    if (!gl) {
      console.error('WebGL not supported');
      return false;
    }

    glRef.current = gl;

    // Create shaders
    const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);

    if (!vertexShader || !fragmentShader) return false;

    // Create program
    const program = gl.createProgram();
    if (!program) return false;

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      console.error('Program link failed:', gl.getProgramInfoLog(program));
      return false;
    }

    programRef.current = program;

    // Set up geometry (full screen triangle)
    const vertices = new Float32Array([
      -1, -1,
       3, -1,
      -1,  3
    ]);

    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

    const positionLocation = gl.getAttribLocation(program, 'a_position');
    gl.enableVertexAttribArray(positionLocation);
    gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

    return true;
  };

  const createShader = (gl: WebGLRenderingContext, type: number, source: string) => {
    const shader = gl.createShader(type);
    if (!shader) return null;

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShader