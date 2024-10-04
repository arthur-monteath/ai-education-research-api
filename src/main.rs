use axum::{routing::{get, post}, Router, extract::Path, Json, response::IntoResponse, http::StatusCode };
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Question {
    text: String,
    alternatives: Vec<String>,  // 4 possible answers
    correct_answer_index: usize, // Index of the correct alternative (0-3)
}

#[derive(Serialize, Deserialize, Clone)]
struct ExercisePacket {
    name: String,
    questions: Vec<Question>,
}

#[derive(Serialize)]
struct SubmitResponse {
    correct: bool,
    message: String,
}

// Mock database: HashMap of exercise packets
fn get_mock_packets() -> HashMap<String, ExercisePacket> {
    let mut packets = HashMap::new();

    // Packet 1: Math with multiple-choice questions
    packets.insert(
        "math".to_string(),
        ExercisePacket {
            name: "Math Packet".to_string(),
            questions: vec![
                Question {
                    text: "What is 2 + 2?".to_string(),
                    alternatives: vec!["3".to_string(), "4".to_string(), "5".to_string(), "6".to_string()],
                    correct_answer_index: 1, // "4" is correct
                },
                Question {
                    text: "What is 3 * 3?".to_string(),
                    alternatives: vec!["6".to_string(), "7".to_string(), "8".to_string(), "9".to_string()],
                    correct_answer_index: 3, // "9" is correct
                },
            ],
        },
    );

    // Packet 2: Science with multiple-choice questions
    packets.insert(
        "science".to_string(),
        ExercisePacket {
            name: "Science Packet".to_string(),
            questions: vec![
                Question {
                    text: "What planet is known as the Red Planet?".to_string(),
                    alternatives: vec!["Earth".to_string(), "Mars".to_string(), "Jupiter".to_string(), "Venus".to_string()],
                    correct_answer_index: 1, // "Mars" is correct
                },
                Question {
                    text: "What is the chemical symbol for water?".to_string(),
                    alternatives: vec!["O2".to_string(), "CO2".to_string(), "H2O".to_string(), "N2".to_string()],
                    correct_answer_index: 2, // "H2O" is correct
                },
            ],
        },
    );

    packets
}

// List available packets
async fn list_packets() -> Json<Vec<String>> {
    let packets = get_mock_packets();
    let packet_names: Vec<String> = packets.keys().cloned().collect();
    Json(packet_names)
}

async fn get_packet(Path(packet_name): Path<String>) -> impl IntoResponse {
    let packets = get_mock_packets();

    // Find the packet
    if let Some(packet) = packets.get(&packet_name) {
        
        return (StatusCode::OK, Json(packet.clone()));
    }

    (StatusCode::NOT_FOUND, Json(ExercisePacket {
        name: "Packet not found".to_string(),
        questions: vec![]
    }))
}

async fn get_question(Path((packet_name, question_index)): Path<(String, usize)>) -> impl IntoResponse {
    let packets = get_mock_packets();

    // Find the packet
    if let Some(packet) = packets.get(&packet_name) {
        // Ensure question index is valid
        if question_index < packet.questions.len() {
            let question = packet.questions[question_index].clone();
            return (StatusCode::OK, Json(question));
        }
    }

    (StatusCode::NOT_FOUND, Json(Question {
        text: "Question not found".to_string(),
        alternatives: vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
        correct_answer_index: 0,
    }))
}

// Submit an answer for a question
#[derive(Deserialize)]
struct AnswerPayload {
    answer: String,
}
/*
async fn submit_answer(
    Path((packet_name, question_index)): Path<(String, usize)>,
    Json(payload): Json<AnswerPayload>,
) -> Option<Json<SubmitResponse>> {
    let packets = get_mock_packets();

    // Find the packet
    if let Some(packet) = packets.get(&packet_name) {
        // Ensure question index is valid
        if question_index < packet.questions.len() {
            let correct_answer = &packet.questions[question_index].correct_answer;
            let is_correct = correct_answer == &payload.answer;

            // Prepare the response
            let response = SubmitResponse {
                correct: is_correct,
                message: if is_correct {
                    "Correct!".to_string()
                } else {
                    format!("Wrong! The correct answer is: {}", correct_answer)
                },
            };

            return Some(Json(response));
        }
    }

    None  // Return None if not found
}
*/
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/packets", get(list_packets))
        .route("/packets/:packet_name", get(get_packet))
        .route("/packet/:packet_name/questions/:question_index", get(get_question))
        ; //.route("/packet/:packet_name/questions/:question_index/submit", post(submit_answer))

    Ok(router.into())
}
