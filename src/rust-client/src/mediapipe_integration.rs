//! MediaPipe Integration for Gesture and Face Tracking
//! JavaScript bindings for MediaPipe integration with Rust WASM

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlVideoElement, HtmlCanvasElement, CanvasRenderingContext2d};
use js_sys::{Object, Array, Reflect, Promise};
use std::collections::HashMap;

/// MediaPipe integration wrapper
#[wasm_bindgen]
pub struct MediaPipeIntegration {
    hands: Option<JsValue>,
    face_mesh: Option<JsValue>,
    pose: Option<JsValue>,
    video_element: Option<HtmlVideoElement>,
    canvas_element: Option<HtmlCanvasElement>,
    canvas_context: Option<CanvasRenderingContext2d>,
}

#[wasm_bindgen]
impl MediaPipeIntegration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            hands: None,
            face_mesh: None,
            pose: None,
            video_element: None,
            canvas_element: None,
            canvas_context: None,
        }
    }

    /// Initialize MediaPipe with camera access
    #[wasm_bindgen]
    pub async fn initialize(&mut self) -> Result<(), JsValue> {
        let window = window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;

        // Create video element for camera input
        let video = document
            .create_element("video")?
            .dyn_into::<HtmlVideoElement>()?;
        video.set_autoplay(true);
        video.set_muted(true);
        video.set_width(640);
        video.set_height(480);

        // Create canvas element for drawing results
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(640);
        canvas.set_height(480);

        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2D context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        self.video_element = Some(video.clone());
        self.canvas_element = Some(canvas);
        self.canvas_context = Some(context);

        // Request camera access
        self.setup_camera(&video).await?;

        // Initialize MediaPipe models
        self.initialize_models().await?;

        Ok(())
    }

    /// Set up camera access
    async fn setup_camera(&self, video: &HtmlVideoElement) -> Result<(), JsValue> {
        let window = window().ok_or("No window available")?;
        let navigator = window.navigator();
        let media_devices = navigator.media_devices()?;

        let mut constraints = web_sys::MediaStreamConstraints::new();
        let video_object = Object::new();
        Reflect::set(&video_object, &"width".into(), &640.into())?;
        Reflect::set(&video_object, &"height".into(), &480.into())?;
        Reflect::set(&video_object, &"facingMode".into(), &"user".into())?;
        constraints.video(&video_object);

        let stream = JsFuture::from(media_devices.get_user_media_with_constraints(&constraints)?)
            .await?
            .dyn_into::<web_sys::MediaStream>()?;

        video.set_src_object(Some(&stream));

        Ok(())
    }

    /// Initialize MediaPipe models via JavaScript
    async fn initialize_models(&mut self) -> Result<(), JsValue> {
        let window = window().ok_or("No window available")?;

        // Load MediaPipe scripts dynamically
        self.load_script("https://cdn.jsdelivr.net/npm/@mediapipe/hands@0.4.1675469240/hands.min.js").await?;
        self.load_script("https://cdn.jsdelivr.net/npm/@mediapipe/face_mesh@0.4.1633559619/face_mesh.min.js").await?;
        self.load_script("https://cdn.jsdelivr.net/npm/@mediapipe/pose@0.5.1675469404/pose.min.js").await?;
        self.load_script("https://cdn.jsdelivr.net/npm/@mediapipe/camera_utils@0.3.1640029074/camera_utils.min.js").await?;
        self.load_script("https://cdn.jsdelivr.net/npm/@mediapipe/drawing_utils@0.3.1620248257/drawing_utils.min.js").await?;

        // Initialize models using JavaScript
        self.init_hands_model(&window).await?;
        self.init_face_mesh_model(&window).await?;
        self.init_pose_model(&window).await?;

        Ok(())
    }

    /// Load external JavaScript script
    async fn load_script(&self, src: &str) -> Result<(), JsValue> {
        let window = window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;

        let script = document.create_element("script")?;
        script.set_attribute("src", src)?;
        script.set_attribute("type", "text/javascript")?;

        let promise = Promise::new(&mut |resolve, _reject| {
            let resolve_clone = resolve.clone();
            let closure = Closure::wrap(Box::new(move || {
                resolve_clone.call0(&JsValue::UNDEFINED).unwrap();
            }) as Box<dyn FnMut()>);

            script.set_onload(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        });

        document.head().ok_or("No head element")?.append_child(&script)?;

        JsFuture::from(promise).await?;
        Ok(())
    }

    /// Initialize hands model
    async fn init_hands_model(&mut self, window: &web_sys::Window) -> Result<(), JsValue> {
        let hands_class = js_sys::Reflect::get(window, &"Hands".into())?;
        let hands_constructor = js_sys::Reflect::get(&hands_class, &"Hands".into())?;

        let hands = js_sys::Reflect::construct(&hands_constructor, &Array::new())?;

        // Set up hands configuration
        let config = Object::new();
        Reflect::set(&config, &"locateFile".into(), &Closure::wrap(Box::new(|file: String| {
            format!("https://cdn.jsdelivr.net/npm/@mediapipe/hands@0.4.1675469240/{}", file)
        }) as Box<dyn FnMut(String) -> String>).into())?;

        js_sys::Reflect::apply(&hands_class, &hands, &Array::of1(&config))?;

        // Set up hands solution options
        let options = Object::new();
        Reflect::set(&options, &"maxNumHands".into(), &2.into())?;
        Reflect::set(&options, &"modelComplexity".into(), &1.into())?;
        Reflect::set(&options, &"minDetectionConfidence".into(), &0.5.into())?;
        Reflect::set(&options, &"minTrackingConfidence".into(), &0.5.into())?;

        js_sys::Reflect::apply(
            &js_sys::Reflect::get(&hands, &"setOptions".into())?,
            &hands,
            &Array::of1(&options),
        )?;

        self.hands = Some(hands);
        Ok(())
    }

    /// Initialize face mesh model
    async fn init_face_mesh_model(&mut self, window: &web_sys::Window) -> Result<(), JsValue> {
        let face_mesh_class = js_sys::Reflect::get(window, &"FaceMesh".into())?;
        let face_mesh_constructor = js_sys::Reflect::get(&face_mesh_class, &"FaceMesh".into())?;

        let face_mesh = js_sys::Reflect::construct(&face_mesh_constructor, &Array::new())?;

        // Set up face mesh configuration
        let config = Object::new();
        Reflect::set(&config, &"locateFile".into(), &Closure::wrap(Box::new(|file: String| {
            format!("https://cdn.jsdelivr.net/npm/@mediapipe/face_mesh@0.4.1633559619/{}", file)
        }) as Box<dyn FnMut(String) -> String>).into())?;

        js_sys::Reflect::apply(&face_mesh_class, &face_mesh, &Array::of1(&config))?;

        // Set up face mesh solution options
        let options = Object::new();
        Reflect::set(&options, &"maxNumFaces".into(), &1.into())?;
        Reflect::set(&options, &"refineLandmarks".into(), &false.into())?;
        Reflect::set(&options, &"minDetectionConfidence".into(), &0.5.into())?;
        Reflect::set(&options, &"minTrackingConfidence".into(), &0.5.into())?;

        js_sys::Reflect::apply(
            &js_sys::Reflect::get(&face_mesh, &"setOptions".into())?,
            &face_mesh,
            &Array::of1(&options),
        )?;

        self.face_mesh = Some(face_mesh);
        Ok(())
    }

    /// Initialize pose model
    async fn init_pose_model(&mut self, window: &web_sys::Window) -> Result<(), JsValue> {
        let pose_class = js_sys::Reflect::get(window, &"Pose".into())?;
        let pose_constructor = js_sys::Reflect::get(&pose_class, &"Pose".into())?;

        let pose = js_sys::Reflect::construct(&pose_constructor, &Array::new())?;

        // Set up pose configuration
        let config = Object::new();
        Reflect::set(&config, &"locateFile".into(), &Closure::wrap(Box::new(|file: String| {
            format!("https://cdn.jsdelivr.net/npm/@mediapipe/pose@0.5.1675469404/{}", file)
        }) as Box<dyn FnMut(String) -> String>).into())?;

        js_sys::Reflect::apply(&pose_class, &pose, &Array::of1(&config))?;

        // Set up pose solution options
        let options = Object::new();
        Reflect::set(&options, &"modelComplexity".into(), &1.into())?;
        Reflect::set(&options, &"smoothLandmarks".into(), &true.into())?;
        Reflect::set(&options, &"minDetectionConfidence".into(), &0.5.into())?;
        Reflect::set(&options, &"minTrackingConfidence".into(), &0.5.into())?;

        js_sys::Reflect::apply(
            &js_sys::Reflect::get(&pose, &"setOptions".into())?,
            &pose,
            &Array::of1(&options),
        )?;

        self.pose = Some(pose);
        Ok(())
    }

    /// Start processing with camera
    #[wasm_bindgen]
    pub async fn start_processing(&mut self) -> Result<(), JsValue> {
        if let (Some(video), Some(hands), Some(face_mesh), Some(pose)) = 
            (&self.video_element, &self.hands, &self.face_mesh, &self.pose) {
            
            // Set up results handlers
            self.setup_hands_results_handler(hands)?;
            self.setup_face_mesh_results_handler(face_mesh)?;
            self.setup_pose_results_handler(pose)?;

            // Start camera
            let camera_utils = js_sys::Reflect::get(&window().ok_or("No window")?, &"Camera".into())?;
            let camera_constructor = js_sys::Reflect::get(&camera_utils, &"Camera".into())?;

            let camera = js_sys::Reflect::construct(&camera_constructor, &Array::new())?;

            let camera_options = Object::new();
            Reflect::set(&camera_options, &"videoElement".into(), video)?;
            Reflect::set(&camera_options, &"onFrame".into(), &Closure::wrap(Box::new(move || {
                // Process frame with all models
                if let (Some(hands), Some(face_mesh), Some(pose)) = 
                    (&hands.clone(), &face_mesh.clone(), &pose.clone()) {
                    
                    js_sys::Reflect::apply(
                        &js_sys::Reflect::get(&hands, &"send".into()).unwrap(),
                        &hands,
                        &Array::new(),
                    ).unwrap();

                    js_sys::Reflect::apply(
                        &js_sys::Reflect::get(&face_mesh, &"send".into()).unwrap(),
                        &face_mesh,
                        &Array::new(),
                    ).unwrap();

                    js_sys::Reflect::apply(
                        &js_sys::Reflect::get(&pose, &"send".into()).unwrap(),
                        &pose,
                        &Array::new(),
                    ).unwrap();
                }
            }) as Box<dyn FnMut()>).into())?;

            js_sys::Reflect::apply(&camera_constructor, &camera, &Array::of1(&camera_options))?;

            // Start camera
            js_sys::Reflect::apply(
                &js_sys::Reflect::get(&camera, &"start".into())?,
                &camera,
                &Array::new(),
            )?;
        }

        Ok(())
    }

    /// Set up hands results handler
    fn setup_hands_results_handler(&self, hands: &JsValue) -> Result<(), JsValue> {
        let closure = Closure::wrap(Box::new(move |results: JsValue| {
            // Process hands results
            if let Some(multi_hand_landmarks) = js_sys::Reflect::get(&results, &"multiHandLandmarks".into()).ok() {
                if let Ok(landmarks_array) = multi_hand_landmarks.dyn_into::<Array>() {
                    for i in 0..landmarks_array.length() {
                        if let Ok(hand_landmarks) = landmarks_array.get(i).dyn_into::<Array>() {
                            // Process hand landmarks
                            web_sys::console::log_1(&format!("Hand {} landmarks: {}", i, hand_landmarks.length()).into());
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        js_sys::Reflect::set(hands, &"onResults".into(), &closure.into())?;
        closure.forget();

        Ok(())
    }

    /// Set up face mesh results handler
    fn setup_face_mesh_results_handler(&self, face_mesh: &JsValue) -> Result<(), JsValue> {
        let closure = Closure::wrap(Box::new(move |results: JsValue| {
            // Process face mesh results
            if let Some(multi_face_landmarks) = js_sys::Reflect::get(&results, &"multiFaceLandmarks".into()).ok() {
                if let Ok(landmarks_array) = multi_face_landmarks.dyn_into::<Array>() {
                    for i in 0..landmarks_array.length() {
                        if let Ok(face_landmarks) = landmarks_array.get(i).dyn_into::<Array>() {
                            // Process face landmarks
                            web_sys::console::log_1(&format!("Face {} landmarks: {}", i, face_landmarks.length()).into());
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        js_sys::Reflect::set(face_mesh, &"onResults".into(), &closure.into())?;
        closure.forget();

        Ok(())
    }

    /// Set up pose results handler
    fn setup_pose_results_handler(&self, pose: &JsValue) -> Result<(), JsValue> {
        let closure = Closure::wrap(Box::new(move |results: JsValue| {
            // Process pose results
            if let Some(pose_landmarks) = js_sys::Reflect::get(&results, &"poseLandmarks".into()).ok() {
                if let Ok(landmarks) = pose_landmarks.dyn_into::<Array>() {
                    // Process pose landmarks
                    web_sys::console::log_1(&format!("Pose landmarks: {}", landmarks.length()).into());
                }
            }
        }) as Box<dyn FnMut(JsValue)>);

        js_sys::Reflect::set(pose, &"onResults".into(), &closure.into())?;
        closure.forget();

        Ok(())
    }

    /// Get video element for embedding in DOM
    #[wasm_bindgen]
    pub fn get_video_element(&self) -> Result<HtmlVideoElement, JsValue> {
        self.video_element.clone().ok_or("Video element not initialized".into())
    }

    /// Get canvas element for embedding in DOM
    #[wasm_bindgen]
    pub fn get_canvas_element(&self) -> Result<HtmlCanvasElement, JsValue> {
        self.canvas_element.clone().ok_or("Canvas element not initialized".into())
    }

    /// Stop processing
    #[wasm_bindgen]
    pub fn stop_processing(&mut self) -> Result<(), JsValue> {
        // Clean up resources
        self.hands = None;
        self.face_mesh = None;
        self.pose = None;
        
        if let Some(video) = &self.video_element {
            if let Some(stream) = video.src_object() {
                if let Ok(media_stream) = stream.dyn_into::<web_sys::MediaStream>() {
                    let tracks = media_stream.get_tracks();
                    for i in 0..tracks.length() {
                        if let Ok(track) = tracks.get(i).dyn_into::<web_sys::MediaStreamTrack>() {
                            track.stop();
                        }
                    }
                }
            }
            video.set_src_object(None);
        }

        Ok(())
    }
}

/// Simple voice processing integration
#[wasm_bindgen]
pub struct VoiceIntegration {
    audio_context: Option<web_sys::AudioContext>,
    analyser: Option<web_sys::AnalyserNode>,
    data_array: Option<js_sys::Uint8Array>,
}

#[wasm_bindgen]
impl VoiceIntegration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            audio_context: None,
            analyser: None,
            data_array: None,
        }
    }

    /// Initialize voice processing
    #[wasm_bindgen]
    pub async fn initialize(&mut self) -> Result<(), JsValue> {
        let window = window().ok_or("No window available")?;
        let navigator = window.navigator();
        let media_devices = navigator.media_devices()?;

        let mut constraints = web_sys::MediaStreamConstraints::new();
        let audio_object = Object::new();
        Reflect::set(&audio_object, &"echoCancellation".into(), &true.into())?;
        Reflect::set(&audio_object, &"noiseSuppression".into(), &true.into())?;
        Reflect::set(&audio_object, &"sampleRate".into(), &44100.into())?;
        constraints.audio(&audio_object);

        let stream = JsFuture::from(media_devices.get_user_media_with_constraints(&constraints)?)
            .await?
            .dyn_into::<web_sys::MediaStream>()?;

        // Create audio context and analyser
        let audio_context = web_sys::AudioContext::new()?;
        let source = audio_context.create_media_stream_source(&stream)?;
        let analyser = audio_context.create_analyser()?;
        
        analyser.set_fft_size(2048);
        analyser.set_smoothing_time_constant(0.8);
        
        source.connect_with_audio_node(&analyser)?;

        let buffer_length = analyser.frequency_bin_count();
        let data_array = js_sys::Uint8Array::new_with_length(buffer_length);

        self.audio_context = Some(audio_context);
        self.analyser = Some(analyser);
        self.data_array = Some(data_array);

        Ok(())
    }

    /// Get audio features
    #[wasm_bindgen]
    pub fn get_audio_features(&mut self) -> Result<String, JsValue> {
        if let (Some(analyser), Some(data_array)) = (&self.analyser, &self.data_array) {
            analyser.get_byte_frequency_data(data_array);

            let mut total_energy = 0.0;
            let mut spectral_centroid = 0.0;
            let mut total_magnitude = 0.0;

            for i in 0..data_array.length() {
                let magnitude = data_array.get_index(i) as f32;
                total_energy += magnitude * magnitude;
                spectral_centroid += i as f32 * magnitude;
                total_magnitude += magnitude;
            }

            let energy = if data_array.length() > 0 {
                total_energy / data_array.length() as f32
            } else {
                0.0
            };

            let centroid = if total_magnitude > 0.0 {
                spectral_centroid / total_magnitude
            } else {
                0.0
            };

            let features = serde_json::json!({
                "energy": energy,
                "spectral_centroid": centroid,
                "volume": (energy / 255.0).sqrt(),
            });

            Ok(features.to_string())
        } else {
            Err("Audio not initialized".into())
        }
    }

    /// Stop voice processing
    #[wasm_bindgen]
    pub fn stop(&mut self) -> Result<(), JsValue> {
        if let Some(context) = &self.audio_context {
            context.close()?;
        }
        self.audio_context = None;
        self.analyser = None;
        self.data_array = None;
        Ok(())
    }
}