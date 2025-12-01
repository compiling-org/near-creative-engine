//! Leap Motion Integration for Advanced Hand Tracking
//! JavaScript bindings for Leap Motion WebSocket connection with Rust WASM

use wasm_bindgen::prelude::*;
use web_sys::{window, WebSocket, MessageEvent, ErrorEvent, CloseEvent};
use js_sys::{Object, Array, Reflect, Promise};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Leap Motion integration wrapper
#[wasm_bindgen]
pub struct LeapMotionIntegration {
    websocket: Option<WebSocket>,
    connected: Arc<Mutex<bool>>,
    frame_data: Arc<Mutex<LeapFrameData>>,
    gesture_callbacks: Arc<Mutex<Vec<js_sys::Function>>>,
}

/// Leap Motion frame data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapFrameData {
    pub id: i32,
    pub timestamp: f64,
    pub hands: Vec<LeapHandData>,
    pub gestures: Vec<LeapGestureData>,
    pub pointables: Vec<LeapPointableData>,
    pub frame_rate: f32,
}

/// Leap Motion hand data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapHandData {
    pub id: i32,
    pub palm_position: LeapVector,
    pub palm_velocity: LeapVector,
    pub palm_normal: LeapVector,
    pub direction: LeapVector,
    pub palm_width: f32,
    pub grab_strength: f32,
    pub pinch_strength: f32,
    pub confidence: f32,
    pub time_visible: f32,
    pub arm: Option<LeapArmData>,
    pub fingers: Vec<LeapFingerData>,
    pub sphere_radius: f32,
}

/// Leap Motion finger data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapFingerData {
    pub id: i32,
    pub type_: String, // thumb, index, middle, ring, pinky
    pub length: f32,
    pub width: f32,
    pub tip_position: LeapVector,
    pub tip_velocity: LeapVector,
    pub direction: LeapVector,
    pub stabilized_tip_position: LeapVector,
    pub bones: Vec<LeapBoneData>,
    pub extended: bool,
}

/// Leap Motion bone data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapBoneData {
    pub type_: String, // metacarpal, proximal, intermediate, distal
    pub prev_joint: LeapVector,
    pub next_joint: LeapVector,
    pub center: LeapVector,
    pub direction: LeapVector,
    pub length: f32,
    pub width: f32,
}

/// Leap Motion arm data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapArmData {
    pub elbow_position: LeapVector,
    pub wrist_position: LeapVector,
    pub direction: LeapVector,
    pub center: LeapVector,
    pub length: f32,
    pub width: f32,
}

/// Leap Motion gesture data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapGestureData {
    pub id: i32,
    pub type_: String, // circle, swipe, key_tap, screen_tap
    pub state: String, // start, update, stop
    pub duration: f32,
    pub duration_seconds: f32,
    pub confidence: f32,
    pub hand_ids: Vec<i32>,
    pub pointable_ids: Vec<i32>,
    pub progress: f32,
    pub radius: f32,
    pub angle: f32,
    pub direction: LeapVector,
    pub normal: LeapVector,
    pub position: LeapVector,
}

/// Leap Motion pointable data (tools or extended fingers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapPointableData {
    pub id: i32,
    pub type_: String, // finger, tool
    pub length: f32,
    pub width: f32,
    pub direction: LeapVector,
    pub tip_position: LeapVector,
    pub tip_velocity: LeapVector,
    pub stabilized_tip_position: LeapVector,
    pub time_visible: f32,
    pub touch_zone: String, // none, hovering, touching
    pub touch_distance: f32,
}

/// 3D vector for Leap Motion coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeapVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Creative gesture interpretation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeGesture {
    pub gesture_type: String,
    pub confidence: f32,
    pub creative_intent: CreativeIntentData,
    pub timestamp: f64,
}

/// Creative intent data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeIntentData {
    pub action: String,
    pub parameters: HashMap<String, f32>,
    pub emotion_hint: String,
    pub suggested_tools: Vec<String>,
}

#[wasm_bindgen]
impl LeapMotionIntegration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            websocket: None,
            connected: Arc::new(Mutex::new(false)),
            frame_data: Arc::new(Mutex::new(LeapFrameData {
                id: 0,
                timestamp: 0.0,
                hands: Vec::new(),
                gestures: Vec::new(),
                pointables: Vec::new(),
                frame_rate: 0.0,
            })),
            gesture_callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Connect to Leap Motion WebSocket server
    #[wasm_bindgen]
    pub async fn connect(&mut self, host: Option<String>) -> Result<(), JsValue> {
        let ws_host = host.unwrap_or_else(|| "ws://localhost:6437".to_string());
        let ws_url = format!("{}/v6.json", ws_host);
        
        let websocket = WebSocket::new(&ws_url)?;
        websocket.set_binary_type(web_sys::BinaryType::Arraybuffer);
        
        let connected = Arc::clone(&self.connected);
        let frame_data = Arc::clone(&self.frame_data);
        let gesture_callbacks = Arc::clone(&self.gesture_callbacks);
        
        // Set up onopen handler
        let onopen_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            web_sys::console::log_1(&"Leap Motion WebSocket connected".into());
            if let Ok(mut conn) = connected.lock() {
                *conn = true;
            }
        }) as Box<dyn FnMut(_)>);
        
        websocket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        // Set up onmessage handler
        let frame_data_clone = Arc::clone(&frame_data);
        let gesture_callbacks_clone = Arc::clone(&gesture_callbacks);
        
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(array_buffer) = event.data().dyn_into::<js_sys::ArrayBuffer>() {
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let mut data = vec![0; uint8_array.length() as usize];
                uint8_array.copy_to(&mut data);
                
                // Parse JSON data
                if let Ok(json_str) = String::from_utf8(data) {
                    if let Ok(frame) = serde_json::from_str::<LeapFrameData>(&json_str) {
                        // Update frame data
                        if let Ok(mut frame_data) = frame_data_clone.lock() {
                            *frame_data = frame.clone();
                        }
                        
                        // Process gestures and trigger callbacks
                        Self::process_gestures(&frame, &gesture_callbacks_clone);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        websocket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        // Set up onerror handler
        let onerror_callback = Closure::wrap(Box::new(move |event: ErrorEvent| {
            web_sys::console::error_1(&format!("Leap Motion WebSocket error: {:?}", event).into());
        }) as Box<dyn FnMut(_)>);
        
        websocket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
        
        // Set up onclose handler
        let connected_clone = Arc::clone(&self.connected);
        let onclose_callback = Closure::wrap(Box::new(move |_event: CloseEvent| {
            web_sys::console::log_1(&"Leap Motion WebSocket disconnected".into());
            if let Ok(mut conn) = connected_clone.lock() {
                *conn = false;
            }
        }) as Box<dyn FnMut(_)>);
        
        websocket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
        
        self.websocket = Some(websocket);
        Ok(())
    }

    /// Process gestures from frame data
    fn process_gestures(frame: &LeapFrameData, gesture_callbacks: &Arc<Mutex<Vec<js_sys::Function>>>) {
        for gesture in &frame.gestures {
            if let Ok(creative_gesture) = Self::interpret_gesture_creatively(gesture, frame) {
                // Trigger JavaScript callbacks
                if let Ok(callbacks) = gesture_callbacks.lock() {
                    for callback in callbacks.iter() {
                        if let Ok(json_str) = serde_json::to_string(&creative_gesture) {
                            let _ = callback.call1(&JsValue::NULL, &json_str.into());
                        }
                    }
                }
            }
        }
    }

    /// Interpret Leap Motion gesture creatively
    fn interpret_gesture_creatively(gesture: &LeapGestureData, frame: &LeapFrameData) -> Result<CreativeGesture, JsValue> {
        let (action, parameters, emotion_hint, suggested_tools) = match gesture.type_.as_str() {
            "circle" => (
                "rotate".to_string(),
                HashMap::from([
                    ("radius".to_string(), gesture.radius),
                    ("angle".to_string(), gesture.angle),
                    ("progress".to_string(), gesture.progress),
                ]),
                "flowing".to_string(),
                vec!["brush".to_string(), "shape".to_string()],
            ),
            "swipe" => (
                "swipe".to_string(),
                HashMap::from([
                    ("direction_x".to_string(), gesture.direction.x),
                    ("direction_y".to_string(), gesture.direction.y),
                    ("speed".to_string(), gesture.direction.x.abs() + gesture.direction.y.abs()),
                ]),
                "decisive".to_string(),
                vec!["eraser".to_string(), "selection".to_string()],
            ),
            "key_tap" => (
                "tap".to_string(),
                HashMap::from([
                    ("position_x".to_string(), gesture.position.x),
                    ("position_y".to_string(), gesture.position.y),
                    ("confidence".to_string(), gesture.confidence),
                ]),
                "precise".to_string(),
                vec!["select".to_string(), "activate".to_string()],
            ),
            "screen_tap" => (
                "select".to_string(),
                HashMap::from([
                    ("position_x".to_string(), gesture.position.x),
                    ("position_y".to_string(), gesture.position.y),
                    ("confidence".to_string(), gesture.confidence),
                ]),
                "intentional".to_string(),
                vec!["menu".to_string(), "tool".to_string()],
            ),
            _ => (
                "explore".to_string(),
                HashMap::new(),
                "curious".to_string(),
                vec!["explore".to_string()],
            ),
        };

        Ok(CreativeGesture {
            gesture_type: gesture.type_.clone(),
            confidence: gesture.confidence,
            creative_intent: CreativeIntentData {
                action,
                parameters,
                emotion_hint,
                suggested_tools,
            },
            timestamp: frame.timestamp,
        })
    }

    /// Add gesture callback function
    #[wasm_bindgen]
    pub fn add_gesture_callback(&mut self, callback: js_sys::Function) -> Result<(), JsValue> {
        if let Ok(mut callbacks) = self.gesture_callbacks.lock() {
            callbacks.push(callback);
        }
        Ok(())
    }

    /// Get current frame data as JSON
    #[wasm_bindgen]
    pub fn get_frame_data(&self) -> Result<String, JsValue> {
        if let Ok(frame_data) = self.frame_data.lock() {
            serde_json::to_string(&*frame_data)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize frame data: {}", e)))
        } else {
            Err(JsValue::from_str("Failed to lock frame data"))
        }
    }

    /// Get hand count
    #[wasm_bindgen]
    pub fn get_hand_count(&self) -> Result<u32, JsValue> {
        if let Ok(frame_data) = self.frame_data.lock() {
            Ok(frame_data.hands.len() as u32)
        } else {
            Ok(0)
        }
    }

    /// Get gesture count
    #[wasm_bindgen]
    pub fn get_gesture_count(&self) -> Result<u32, JsValue> {
        if let Ok(frame_data) = self.frame_data.lock() {
            Ok(frame_data.gestures.len() as u32)
        } else {
            Ok(0)
        }
    }

    /// Get latest gesture
    #[wasm_bindgen]
    pub fn get_latest_gesture(&self) -> Result<String, JsValue> {
        if let Ok(frame_data) = self.frame_data.lock() {
            if let Some(gesture) = frame_data.gestures.last() {
                serde_json::to_string(gesture)
                    .map_err(|e| JsValue::from_str(&format!("Failed to serialize gesture: {}", e)))
            } else {
                Err(JsValue::from_str("No gestures available"))
            }
        } else {
            Err(JsValue::from_str("Failed to lock frame data"))
        }
    }

    /// Check if connected
    #[wasm_bindgen]
    pub fn is_connected(&self) -> bool {
        if let Ok(connected) = self.connected.lock() {
            *connected
        } else {
            false
        }
    }

    /// Enable gesture detection
    #[wasm_bindgen]
    pub fn enable_gestures(&mut self, enable: bool) -> Result<(), JsValue> {
        if let Some(websocket) = &self.websocket {
            if websocket.ready_state() == WebSocket::OPEN {
                let config = serde_json::json!({
                    "enableGestures": enable,
                    "background": !enable,
                });
                
                if let Ok(config_str) = serde_json::to_string(&config) {
                    let _ = websocket.send_with_str(&config_str);
                }
            }
        }
        Ok(())
    }

    /// Disconnect from Leap Motion
    #[wasm_bindgen]
    pub fn disconnect(&mut self) -> Result<(), JsValue> {
        if let Some(websocket) = &self.websocket {
            websocket.close()?;
        }
        
        if let Ok(mut connected) = self.connected.lock() {
            *connected = false;
        }
        
        self.websocket = None;
        Ok(())
    }
}

/// Hand tracking utilities for creative applications
#[wasm_bindgen]
pub struct CreativeHandTracker {
    leap_integration: LeapMotionIntegration,
    gesture_history: Arc<Mutex<Vec<CreativeGesture>>>,
    creative_state: Arc<Mutex<CreativeStateData>>,
}

/// Creative state data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeStateData {
    pub current_tool: String,
    pub brush_size: f32,
    pub color: String,
    pub opacity: f32,
    pub pressure: f32,
    pub mode: String,
}

#[wasm_bindgen]
impl CreativeHandTracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            leap_integration: LeapMotionIntegration::new(),
            gesture_history: Arc::new(Mutex::new(Vec::new())),
            creative_state: Arc::new(Mutex::new(CreativeStateData {
                current_tool: "brush".to_string(),
                brush_size: 5.0,
                color: "#000000".to_string(),
                opacity: 1.0,
                pressure: 0.5,
                mode: "draw".to_string(),
            })),
        }
    }

    /// Connect to Leap Motion
    #[wasm_bindgen]
    pub async fn connect(&mut self, host: Option<String>) -> Result<(), JsValue> {
        self.leap_integration.connect(host).await
    }

    /// Start creative hand tracking
    #[wasm_bindgen]
    pub fn start_tracking(&mut self) -> Result<(), JsValue> {
        // Set up gesture callback to update creative state
        let gesture_history = Arc::clone(&self.gesture_history);
        let creative_state = Arc::clone(&self.creative_state);
        
        let callback = Closure::wrap(Box::new(move |gesture_json: JsValue| {
            if let Ok(gesture_str) = gesture_json.as_string() {
                if let Ok(gesture) = serde_json::from_str::<CreativeGesture>(&gesture_str) {
                    // Update gesture history
                    if let Ok(mut history) = gesture_history.lock() {
                        history.push(gesture.clone());
                        if history.len() > 100 {
                            history.remove(0);
                        }
                    }
                    
                    // Update creative state based on gesture
                    Self::update_creative_state(&gesture, &creative_state);
                }
            }
        }) as Box<dyn FnMut(JsValue)>);
        
        self.leap_integration.add_gesture_callback(callback.into_js_value().dyn_into().unwrap())?;
        
        // Enable gestures
        self.leap_integration.enable_gestures(true)?;
        
        Ok(())
    }

    /// Update creative state based on gesture
    fn update_creative_state(gesture: &CreativeGesture, creative_state: &Arc<Mutex<CreativeStateData>>) {
        if let Ok(mut state) = creative_state.lock() {
            match gesture.creative_intent.action.as_str() {
                "rotate" => {
                    state.mode = "rotate".to_string();
                    if let Some(radius) = gesture.creative_intent.parameters.get("radius") {
                        state.brush_size = radius * 10.0;
                    }
                },
                "swipe" => {
                    state.mode = "erase".to_string();
                    if let Some(speed) = gesture.creative_intent.parameters.get("speed") {
                        state.opacity = (1.0 - speed * 0.1).clamp(0.1, 1.0);
                    }
                },
                "tap" => {
                    state.mode = "select".to_string();
                    // Cycle through tools
                    let tools = vec!["brush", "eraser", "selector", "zoom"];
                    let current_index = tools.iter().position(|&t| t == state.current_tool.as_str()).unwrap_or(0);
                    state.current_tool = tools[(current_index + 1) % tools.len()].to_string();
                },
                "select" => {
                    state.mode = "menu".to_string();
                },
                _ => {
                    state.mode = "draw".to_string();
                }
            }
            
            // Update pressure based on confidence
            state.pressure = gesture.confidence;
        }
    }

    /// Get current creative state
    #[wasm_bindgen]
    pub fn get_creative_state(&self) -> Result<String, JsValue> {
        if let Ok(state) = self.creative_state.lock() {
            serde_json::to_string(&*state)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize creative state: {}", e)))
        } else {
            Err(JsValue::from_str("Failed to lock creative state"))
        }
    }

    /// Get gesture history
    #[wasm_bindgen]
    pub fn get_gesture_history(&self) -> Result<String, JsValue> {
        if let Ok(history) = self.gesture_history.lock() {
            serde_json::to_string(&*history)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize gesture history: {}", e)))
        } else {
            Err(JsValue::from_str("Failed to lock gesture history"))
        }
    }

    /// Get current hand data for drawing
    #[wasm_bindgen]
    pub fn get_drawing_data(&self) -> Result<String, JsValue> {
        if let Ok(frame_data) = self.leap_integration.frame_data.lock() {
            if let Some(hand) = frame_data.hands.first() {
                let drawing_data = serde_json::json!({
                    "position": hand.palm_position,
                    "pressure": hand.grab_strength,
                    "tool": if let Ok(state) = self.creative_state.lock() {
                        state.current_tool.clone()
                    } else { "brush".to_string() },
                    "brush_size": if let Ok(state) = self.creative_state.lock() {
                        state.brush_size
                    } else { 5.0 },
                    "color": if let Ok(state) = self.creative_state.lock() {
                        state.color.clone()
                    } else { "#000000".to_string() },
                });
                
                Ok(drawing_data.to_string())
            } else {
                Err(JsValue::from_str("No hand data available"))
            }
        } else {
            Err(JsValue::from_str("Failed to lock frame data"))
        }
    }

    /// Disconnect from Leap Motion
    #[wasm_bindgen]
    pub fn disconnect(&mut self) -> Result<(), JsValue> {
        self.leap_integration.disconnect()
    }
}