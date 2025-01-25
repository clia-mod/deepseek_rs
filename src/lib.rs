// pub mod client;
// pub mod errors;
// use serde::{Deserialize, Serialize};

// pub struct DeepSeekClient {
//     pub(crate) url: &'static str,
//     pub(crate)  api_key: &'static str,
//     pub(crate) client: reqwest::Client,
// }
// impl DeepSeekClient {
//     pub fn new(url: &'static str, api_key: &'static str) -> DeepSeekClient {
//         DeepSeekClient {
//             url,
//             api_key,
//             client: reqwest::Client::new(),
//         }
//     }
//     pub async fn get_chat_completions(&self) {
//         let body = ChatCompletionRequest {
//             messages: vec![
//                 Message {
//                     content: r#"<examples>\n<example>\n<example_description>\nBecause the solution is correct, a short sentence is needed here stating that it's correct and motivating the student.\n</example_description>\n<Assignment>\n<p>Forme den Bruch &lt;math&gt;\\frac{x}{\\sqrt{3}}&lt;/math&gt; so um, dass der Nenner rational ist.</p>\n</Assignment>\n<System_solution>\n[Vorüberlegung]\nDer Nenner des Bruchs <math>\\frac{x}{\\sqrt{3}}</math> soll rational gemacht werden:\n \n<KE id=\"OQRVpjsJn0llOe2jbbmla5UonV-F\">3 ist keine Quadratzahl (Die Quadratwurzel aus 3 lässt sich nicht direkt berechnen)</KE>\n<KE id=\"4ukMwPi8llOvaILMk4XG6X4h7jBG\"><math>\\to \\sqrt{3}</math> ist irrational</KE>\n<math>\\to \\sqrt{3}</math> muss aus dem Nenner entfernt werden, damit er rational wird.\n \n(Schritt 1) Bruch erweitern, um den Nenner rational zu machen\n<KE id=\"DNuqame0PBohsp3SvPU911xSUgTK\">Der Nenner besteht nur aus der Quadratwurzel <math>\\sqrt{3}</math>. Wir können den Bruch mit ihr erweitern, um sie loszuwerden:</KE>\n \n<math>\\frac{x}{\\sqrt{3}}\\ </math><KE id=\"gb9UzH2lwpSSC-aXPY2_Eb1zRIUM\"><math>\\ | \\ \\text{Bruch erweitern mit } \\sqrt{3}</math></KE>\n<math>=\\frac{\\sqrt{3} \\cdot x}{\\sqrt{3}^2} \\ </math><KE id=\"TsrTR9ibS90yz4vOUmthdiCLT5bq\"><math>\\ | \\ \\sqrt{a}^2=a</math></KE>\n<math>=\\frac{\\sqrt{3} \\cdot x}{3}</math>\n \nAntwort: Wir können <math>\\frac{x}{\\sqrt{3}}</math> mit <math>\\sqrt{3}</math> erweitern, um den Nenner rational zu machen. Wir erhalten dann <math>\\frac{\\sqrt{3}x}{3}</math>.\n</System_solution>\n<Student_solution>\n\\(\\frac{x}{\\sqrt{3}}=\\frac{\\sqrt{3} \\cdot x}{\\sqrt{3} \\cdot \\sqrt{3}}=\\frac{\\sqrt{3}}{3} \\cdot x\\)\n</Student_solution>\n<ideal_output>\n{\n\"correct\": true,\n\"messages\": \"&lt;p&gt;Deine Lösung sieht für mich richtig aus! Gut gemacht! Mach weiter so!&lt;/p&gt;\",\n\"student_solution\": \"&lt;p&gt; &lt;math&gt; \\frac{x}{\\sqrt{3}}=\\frac{\\sqrt{3}\\cdot x}{\\sqrt{3}\\cdot \\sqrt{3}}=\\frac{\\sqrt{3}}{3}\\cdot x&lt;/math&gt; &lt;/p&gt;\",\n\"corrected_solution\":\"\"\n}\n</ideal_output>\n</example>\n<example>\n<example_description>\nthe student_solution has very weird spelling because of OCR quality. the answer is really good since it tried to recognize the initial meaning of the student and did so correctly.\n</example_description>\n<Assignment>\nSophia hat eine Smartwatch, welche ihren Herzschlag zu jedem Zeitpunkt misst. \n  \n  \nGib zwei Zuordnungen an, die den Sachkontext beschreiben. Begründe, ob die jeweilige Zuordnung eine Funktion ist oder nicht.\n</Assignment>\n<System_solution>\n[Vorüberlegung]\n- <KE id=\"KrU02mPOHPh3JykAOe6M5GJcldXI\">Ordnet man eine Größe <math>x</math> einer anderen Größe <math>y</math> zu, so hängt <math>y</math> von <math>x</math> ab und man schreibt <math>x \\mapsto y</math> .</KE>\n- <KE id=\"44BouN6P1Lh-HdfChJEEgfhCET_O\">Eine Funktion ordnet jedem <math>x</math>-Wert genau ein <math>y</math> zu (=eindeutige Zuordnung).</KE>\n \n(Schritt 1) Bestimmen der Zuordnungen im Sachkontext\n<KE id=\"aeofazSDJgevVADGyn3phv3Ffs0t\">Sophias Herzschlag hängt mit den gemessenen Zeitpunkten zusammen.</KE>\n \nWir sollen aus den zwei Größen Herzschlag und Zeitpunkt zwei Zuordnungen bilden.\n \n<KE id=\"TQQLGmyO8l7rpS9cAdCA8iLXFm6u\">Uns bleiben nur <math> \\text{Zeitpunkt} \\mapsto \\text{Herzschlag}</math> und <math>\\text{Herzschlag} \\mapsto \\text{Zeitpunkt}</math> als Zuordnungskombinationen.</KE>\n \n(Schritt 2) Entscheiden über Funktionseigenschaft\nFür <math> \\text{Zeitpunkt} \\mapsto \\text{Herzschlag}</math>:\n- <KE id=\"KrU02mPOHPh3JykAOe6M5GJcldXI\">Interpretation: Wir betrachten den Herzschlag in Abhängigkeit vom Zeitpunkt. </KE>\n- Beispiel: Sophia misst in der 2. Minute einen Herzschlag von 110 und in der 3. Minute einen Herzschlag von 125.\n<math>\\to</math> Es ist im Sachkontext logisch, dass zu jedem Zeitpunkt nur ein Herzschlag gemessen werden kann\n<KE id=\"TQQLGmyO8l7rpS9cAdCA8iLXFm6u\"><math>\\to</math> Wegen der Eindeutigkeit, ist <math> \\text{Zeitpunkt} \\mapsto \\text{Herzschlag}</math> eine Funktion.</KE>\n \nFür <math>\\text{Herzschlag} \\mapsto \\text{Zeitpunkt}</math>:\n- <KE id=\"KrU02mPOHPh3JykAOe6M5GJcldXI\">Interpretation: Wir betrachten den Zeitpunkt in Abhängigkeit vom Herzschlag. </KE>\n- Beispiel: Sophia misst den Herzschlag von 110 in der 2. Minute sowie in der 9. Minute.\n<math>\\to</math> Es ist im Sachkontext logisch, dass ein Herzschlag zu mehreren Zeitpunkten gemessen werden kann\n<KE id=\"TQQLGmyO8l7rpS9cAdCA8iLXFm6u\"><math>\\to</math> Wegen der Mehrdeutigkeit, ist <math>\\text{Herzschlag} \\mapsto \\text{Zeitpunkt}</math> keine Funktion.</KE>\n \nAntwort: <math> \\text{Zeitpunkt} \\mapsto \\text{Herzschlag}</math> ist als eindeutige Zuordnung eine Funktion und <math>\\text{Herzschlag} \\mapsto \\text{Zeitpunkt}</math> ist als mehrdeutige Zuordnung keine Funktion.\n</System_solution>\n<Student_solution>\nHerzschlag \\(\\rightarrow\\) Zait int Funktion, wil rassict in Echt\n\nPuls \\(\\mapsto\\) Akfiritil iot keic Foulhin, da kuin dichte Zusmemeherg\n</Student_solution>\n<ideal_output>\n{\n\"correct\": false,\n\"messages\": \"&lt;p&gt;Ihre Lösung enthält einige Fehler. Gehen wir sie Schritt für Schritt durch:&lt;/p&gt; &lt;p&gt;1. Die erste Abbildung (&lt;math&gt;\\text{Herzschlag} \\to \\text{Zeit}&lt;/math&gt;) ist falsch. Du hast gesagt, es sei eine Funktion, aber das ist es nicht. Derselbe Herzschlagwert kann zu verschiedenen Zeiten auftreten. Zum Beispiel kann ein Herzschlag von 110 sowohl bei 2 Minuten als auch bei 9 Minuten auftreten.2. Die zweite Abbildung ((&lt;math&gt;\\text{Puls} \\to \\text{Aktivitaet}&lt;/math&gt;)) hat nichts mit der Aufgabe zu tun. Die Frage bezieht sich auf den Zusammenhang zwischen der Zeit und den Herzschlagmessungen von Sophias Smartwatch. Du hättest (&lt;math&gt;\\text{Zeit} \\to \\text{Herzschlag}&lt;/math&gt;) als zweite Abbildung diskutieren sollen.&lt;/p&gt;&lt;p&gt;3. Deinen Erklärungen fehlt die mathematische Argumentation. Anstatt zu sagen 'weil passiert in Echt', solltest du mit Hilfe der Idee der eindeutigen Zuordnung erklären, warum die Abbildung eine Funktion ist oder nicht. &lt;/p&gt;Hier ist die richtige Lösung: (&lt;math&gt;\\text{Zeit} \\to \\text{Herzschlag}&lt;/math&gt;) ist eine Funktion, weil es für jeden bestimmten Zeitpunkt genau eine Herzschlagmessung gibt. (&lt;math&gt;\\text{Herzschlag} \\to \\text{Zeit}&lt;/math&gt;) ist keine Funktion, weil derselbe Herzschlagwert zu mehreren Zeitpunkten vorkommen kann.&lt;/p&gt;\",\n\"corrected_solution\": \"&lt;p&gt; &lt;/math&gt;\\text{Herzschlag} \\to \\text{Zeit}&lt;/math&gt; ist &lt;corrected&gt;keine&lt;/corrected&gt; Funktion, &lt;corrected&gt;weil dies eine mehrdeutige Zuordung ist &lt;/corrected&gt; &lt;/p&gt; &lt;p&gt; &lt;corrected&gt;&lt;/math&gt;\\text{Zeitpunkt} \\to \\text{Herzschlag}&lt;/math&gt;&lt;/corrected&gt; ist &lt;corrected&gt;eine&lt;/corrected&gt; Funktion, da &lt;corrected&gt; dies eine eindeutige Zuordung ist&lt;/corrected&gt;&lt;/p&gt;\"\n}\n\n\n</ideal_output>\n</example>\n</examples>\n\n"#.to_string(),

//                     role: "system".to_string(),
//                 },
//                 Message {
//                     content:  r#"### Context: You are a teacher at a German math school, and you got many student homework to correct in German.\n\n### Role: A german math teacher reviewing the math homework of students in german. You review math assignments from grades 7 to 12 by comparing the student's solutions (provided in LaTeX text format) with the correct solutions in our system. \n\n\n# Actions\n\n1. **Review Each Problem and Solution:**\n   - Carefully read each problem and its corresponding solution provided in our system.\n   - Ensure a clear understanding of the mathematical concepts and operations involved.\n\n2. **Review User's Solution in LaTeX Format:**\n   - Examine the student's solution provided in LaTeX text format.\n   - Verify if the angles and geometric constructions match those in the provided solution.\n   - Check the mathematical formulas used and confirm their correctness based on the provided solutions.\n   - Students often solve assignments differently than in the system-solution. As long as the final result is correct, you should consider the student's solution as correct.\n\n3. **Perform Independent Calculations:**\n   - **Note:** Certain assignments may have some error tolerance in the final result, especially when π is involved.\n   - Independently perform the calculations presented in the student's solution.\n   - Use standard arithmetic operations to arrive at the answer.\n\n4. **Compare and Identify Errors:**\n   - Compare the student's solution with the correct solution from our system.\n   - Identify any calculation errors or inaccuracies in the geometric drawings.\n   - Note any discrepancies in formulas, calculations, or geometric constructions.\n   - Neglect spelling errors \n\n5. **Take a deep breath and focus on point 6**\n\n6. **Provide Feedback:**\n   - Determine if the student's solution is correct.\n   - If the solution is correct, just state that the solution is correct and say a short motivational sentence. \n   - If the solution is incorrect, prepare a message explaining why the solution is incorrect, highlighting specific errors. You should also shortly explain the mathematical rules that need to be used in order to get to the correct result\n\n7. **Provide the students solution**\n  - reformat the students' solution such that mathematical expressions are embedded by &lt;math&gt; x^2\\cdot 2 &lt;/math&gt; with latex syntax inside. \n  - Each step should be in a new paragraph &lt;p&gt;(...)&lt;/p&gt;\n  - If words are written unclear (e.g. \"praragrapu\") try to fix them in the context of the assignment (e.g. \"paragraph\")\n  - If the students' solution is mathematically wrong, highlight the wrong sections of the solution with &lt;wrong&gt;&lt;/wrong&gt;\n  - If the students solution is wrong, you have to underline at least one wrong part\n  - Keep the original language provided by the user\n\n8. **Prepare a corrected solution**\n  - Based on the students' solution approach, create a corrected solution with a similar structure such that the student can easily compare and understand his mistakes\n  - Again use the math tag formatting \n  - Your corrected steps need to be embedded in &lt;corrected&gt; ... &lt;/corrected&gt;\n  - If the student had spelling or grammar mistakes, you should just correct them without tagging them. You can e.g. add articles \n  - If the students' solution is correct, don't provide any corrected solution\n\n###Format\n#General Formatting rules\n- Only use HTML tags like, e.g. <p></p> and &nbsp; .\n- Embed all mathematical expressions including single numbers in &lt;math&gt; x \\cdot y^2 &lt;/math&gt; tags, never use the notation <math></math>\n- Inside of math tags you use latex format, but only use formatting for mathematical expressions, no structural formatting like \\begin{aligned}\n\n# Input Format\nQuestion: {{Assignment}}\nSystem_Solution: {{System_solution}}\nUser_solution: {{Student_solution}}\n\n# Output Format\nProvide your assessment only in the following JSON format, without any reasoning before or after:\n\n
//                     \n{\n  \"correct\": <true or false>,\n  \"messages\": \"<feedback>\",\n  \"student_solution\": \"<the students solution in correct formatting>\",\n  \"corrected_solution\" : \"<the corrected solution of the student if it was wrong>\"\n  \n}\n\n\n# Notes\n- The students' solution doesn't need to match exactly with the provided system_solution, often it has different intermediate calculations. As long as the final result is mathematically the same, consider the calculation as correct. Note, these terms are equivalent \"4-1\" and \"-1+4\"\n- Pay particular attention to the subjectivity in geometric interpretations if the instructions leave some room for creative construction.\n- Ensure precision and clarity to avoid any misunderstanding, particularly in error explanations.\n\n###Target Audience:\nStudents from grade 7 to 12."#.to_string(),
//                     role: "system".to_string(),
//                 },
//                 Message {
//                     content:"Assignment: <p>Ein Kreisausschnitt mit dem Mittelpunktswinkel &lt;math&gt;\\alpha = 100^\\circ&lt;/math&gt; und dem Radius &lt;math&gt; 5 \\ \\mathrm{cm}&lt;/math&gt; wird zu einem Kegel zusammen gebogen.</p><p>&nbsp;</p><p>Berechne die Oberfläche &lt;math&gt;O&lt;/math&gt;. Diese schematische Skizze hilft dir:</p><p>![](https://rocketutor-math.s3.eu-central-1.amazonaws.com/MathBucket/rocketutor-math/LowerGrade_10_Kegel_5.png)</p>\n    /n System Solution: <p><strong>[Vorüberlegung]</strong></p>\n<p>Beachte hier, dass der Radius &lt;math&gt;s \\ &lt;/math&gt; des Teilkreises und der Radius &lt;math&gt;r \\&lt;/math&gt; der Grundfläche verschiedene Größen sind.</p>\n<p> </p>\n<p>&lt;KE id=\"E73xz-W-0JP11WftTU1YfhAK93Kn\"&gt;Die Formel zur Berechnung der Oberfläche eines Kegels lautet: &lt;math&gt;O_{\\text{Kegel}}= \\pi r^2 + \\pi r s. &lt;/math&gt;&lt;/KE&gt;</p>\n<p> </p>\n<p>&lt;math&gt; \\to &lt;/math&gt; Wir benötigen also die Mantellinie &lt;math&gt;s&lt;/math&gt; sowie den Radius &lt;math&gt;r&lt;/math&gt; der Grundfläche, welcher nicht gegeben ist. Deshalb machen wir zunächst eine Skizze wie der gefaltete Kegel aussehen würde (dabei entspricht die Mantellinie &lt;math&gt;s&lt;/math&gt; dem Radius des Kreisteils):</p>\n<p> </p>\n<p>![](https://rocketutor-math.s3.eu-central-1.amazonaws.com/MathBucket/rocketutor-math/LowerGrade_10_Kegel_5_sol.png)</p>\n<p> </p>\n<p>Wir sehen:</p>\n<p>Die Bogenlänge &lt;math&gt;b&lt;/math&gt; des Kreisausschnitts entspricht dem Umfang der Grundfläche *&lt;math&gt;G&lt;/math&gt;* des Kegels.</p>\n<p>&lt;math&gt; \\to &lt;/math&gt; Wir können zunächst die Bogenlänge (= Umfang der Grundfläche) berechnen. Anhand des Umfangs können wir dann durch Umstellen von &lt;KE id=\"7gqizlDhMsWg2OkBqYYTB7gFQ0xv\"&gt; &lt;math&gt;U = 2 \\pi \\cdot r&lt;/math&gt;&lt;/KE&gt; den Radius &lt;math&gt;r&lt;/math&gt; der Grundfläche &lt;math&gt;G&lt;/math&gt; berechnen.</p>\n<p> </p>\n<p><strong>(Schritt 1) Berechnen des Radius &lt;math&gt;r&lt;/math&gt; anhand der Bogenlänge</strong></p>\n<p>Wir berechnen die Bogenlänge und damit den Umfang der Grundfläche, um anschließend dadurch den Radius &lt;math&gt;r&lt;/math&gt; zu berechnen:</p>\n<p>&lt;KE id=\"w6RUHp7AZCbgKSVDKw1c5wBojupv\"&gt;Die Bogenlänge &lt;math&gt;b&lt;/math&gt; eines Kreisteils wird mit der Formel &lt;math&gt;b = 2 \\pi \\cdot r \\cdot \\frac{\\alpha}{360^{\\circ}}&lt;/math&gt; berechnet, wobei &lt;math&gt;r&lt;/math&gt; den Radius und &lt;math&gt;\\alpha&lt;/math&gt; den Mittelpunktswinkel des Kreisteils bezeichnet.&lt;/KE&gt;</p>\n<p>&lt;KE id=\"qHmCyl70RdGJHuNHuneWu-aBNMci\"&gt;&lt;math&gt; \\to \\ &lt;/math&gt; Einsetzen von &lt;math&gt;s = r_{Kreisteil} = 5 \\ \\mathrm{cm}&lt;/math&gt; und &lt;math&gt;\\alpha = 100^\\circ &lt;/math&gt;liefert die Bogenlänge:&lt;/KE&gt;</p>\n<p>&lt;math&gt;b = U_{Grundflaeche} = 2 \\pi \\cdot 5 \\mathrm{cm} \\cdot \\frac{100}{360} = \\frac{25\\pi}{9} \\mathrm{cm} &lt;/math&gt;</p>\n<p> </p>\n<p>&lt;KE id=\"7gqizlDhMsWg2OkBqYYTB7gFQ0xv\"&gt;Der Umfang &lt;math&gt;U&lt;/math&gt; eines Kreises wird mit der Formel &lt;math&gt;U = 2 \\pi \\cdot r&lt;/math&gt; berechnet, wobei &lt;math&gt;r&lt;/math&gt; der Radius des Kreises ist.&lt;/KE&gt;</p>\n<p>&lt;KE id=\"qHmCyl70RdGJHuNHuneWu-aBNMci\"&gt;Um den Radius &lt;math&gt;r&lt;/math&gt; zu berechnen, setzen wir &lt;math&gt;U_{Grundflaeche} = \\frac{25\\pi}{9} \\mathrm{cm} &lt;/math&gt; in die Formel ein&lt;/KE&gt; und &lt;KE id=\"W99QSKP3Lsb8LJm7pmorqPPKREgX\"&gt;lösen anschließend nach &lt;math&gt;r&lt;/math&gt; auf:&lt;/KE&gt;</p>\n<p>&lt;math&gt; 2\\pi r = \\frac{25\\pi}{9} \\mathrm{cm} &lt;/math&gt; &lt;math&gt;\\\\ &lt;/math&gt; &lt;KE id=\"17-XUlqn24G1UB5rbFiVdGxhcHdH\"&gt; &lt;math&gt; \\\\ | \\\\ \\div 2 \\pi &lt;/math&gt;&lt;/KE&gt;</p>\n<p>&lt;math&gt; \\Rightarrow \\\\ r = \\frac{\\frac{25\\pi}{9} \\mathrm{cm} }{2\\pi} = \\frac{25}{18} \\mathrm{cm} &lt;/math&gt;</p>\n<p> </p>\n<p><strong>(Schritt 2) Berechnen der Oberfläche &lt;math&gt;O&lt;/math&gt;</strong></p>\n<p>&lt;KE id=\"qHmCyl70RdGJHuNHuneWu-aBNMci\"&gt; Final können wir nun &lt;math&gt; s \\ &lt;/math&gt; und &lt;math&gt; r = \\frac{25}{18} \\mathrm{cm} \\ &lt;/math&gt; in &lt;math&gt;O_{\\text{Kegel}}= \\pi r^2 + \\pi r s &lt;/math&gt; einsetzen, um die Oberfläche zu berechnen: &lt;/KE&gt;</p>\n<p> </p>\n<p>&lt;math&gt;O = \\pi \\left(\\frac{25}{18}\\right)^2 + \\pi \\left(\\frac{25}{18}\\right) \\cdot 5 \\approx 27,88 \\ \\mathrm{cm}^2&lt;/math&gt;</p>\n<p>  </p>\n<p><strong>Antwort: Die Oberfläche des Kegels beträgt &lt;math&gt;O = 27,88 \\ \\mathrm{cm}^2 &lt;/math&gt;.</strong></p>\n\n    /n  student_solution: \n    \\[\\begin{array}{l}O_{\\text{kegel}} = A_{\\text{Grundfl.}} + A_{\\text{Mantel}} \\\\r_{\\text{Grundfl.}} = \\frac{100}{360} \\cdot 5 \\text{ cm}\\end{array}\\]\n     ".to_string(),
//       role: "user".to_string(),
//                 },
//             ],
//             model: "deepseek-reasoner".to_string(),
//             frequency_penalty: 0,
//             max_tokens: 2048,
//             presence_penalty: 0,
//             response_format: ResponseFormat {
//                 type_: "text".to_string(),
//             },
//             stop: None,
//             stream: false,
//             stream_options: None,
//             temperature: 0.1,
//             top_p: 0.01,
//             tools: None,
//             tool_choice: "none".to_string(),
//             logprobs: false,
//             top_logprobs: None,
//         };
//         let result = self
//             .client
//             .post(&format!("{}/{}", self.url, "chat/completions"))
//             .header("X-Api-Key", self.api_key)
//             .header(
//                 "Authorization",
//                 format!("Bearer {}", "sk-bc721f72713d4c058b3e23b7544b4247"),
//             )
//             .header("Content-Type", "application/json")
//             .body(serde_json::to_string(&body).unwrap())
//             .send()
//             .await;
//         let text = result.unwrap().text().await.unwrap();
//         println!("{}", text);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_get_chat_completions() {
//         let client = DeepSeekClient::new("https://api.deepseek.com", "");
//         client.get_chat_completions().await;
//     }
// }
// // {
// //     "messages": [
// //       {
// //         "content": "You are a helpful assistant",
// //         "role": "system"
// //       },
// //       {
// //         "content": "Hi",
// //         "role": "user"
// //       }
// //     ],
// //     "model": "deepseek-chat",
// //     "frequency_penalty": 0,
// //     "max_tokens": 2048,
// //     "presence_penalty": 0,
// //     "response_format": {
// //       "type": "text"
// //     },
// //     "stop": null,
// //     "stream": false,
// //     "stream_options": null,
// //     "temperature": 1,
// //     "top_p": 1,
// //     "tools": null,
// //     "tool_choice": "none",
// //     "logprobs": false,
// //     "top_logprobs": null
// //   }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChatCompletionRequest {
//     messages: Vec<Message>,
//     model: String,
//     frequency_penalty: i32,
//     max_tokens: i32,
//     presence_penalty: i32,
//     response_format: ResponseFormat,
//     stop: Option<String>,
//     stream: bool,
//     stream_options: Option<String>,
//     temperature: f64,
//     top_p: f32,
//     tools: Option<String>,
//     tool_choice: String,
//     logprobs: bool,
//     top_logprobs: Option<String>,
// }
// #[derive(Debug, Serialize, Deserialize)]

// pub struct Message {
//     content: String,
//     role: String,
// }
// #[derive(Debug, Serialize, Deserialize)]

// pub struct ResponseFormat {
//     #[serde(rename = "type")]
//     type_: String,
// }
