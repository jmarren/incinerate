use maud::{DOCTYPE, Markup, html};

pub async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "incinerate" }
                script src="https://unpkg.com/htmx.org@2.0.4" {}
                style {
                    "canvas { border: 1px solid black; touch-action: none; }"
                }
            }
            body hx-boost="true" {
                h1 { "Hello, World!" }
                canvas id="draw-canvas" hx-encoding="multipart/form-data" width="392" height="392" {}
                br {}
                button  id="submit-button"  {
                    "Submit"
                }
                p { "Prediction: " span id="prediction" {} }
                script {
                    (maud::PreEscaped(r#"
                    const canvas = document.getElementById('draw-canvas');
                    const submitButton = document.getElementById('submit-button');
                    const inputElt = document.getElementById('drawing-input');
                    const ctx = canvas.getContext('2d');
                    ctx.lineWidth = 14;
                    let drawing = false;

                    submitButton.onclick = () => {

                        canvas.toBlob((blob) =>  {
                            const formData = new FormData();
                            formData.append('drawing', blob, 'drawing.png');

                            htmx.ajax('POST', '/drawings', {
                                target: '#prediction',
                                swap: 'innerHTML',
                                source: '#draw-canvas',
                                values: formData,
                            });

                        }, 'image/png');
                    }

                    function pos(e) {
                        const rect = canvas.getBoundingClientRect();
                        return { x: e.clientX - rect.left, y: e.clientY - rect.top };
                    }

                    canvas.addEventListener('mousedown', (e) => {
                        drawing = true;
                        const { x, y } = pos(e);
                        ctx.beginPath();
                        ctx.moveTo(x, y);
                        console.log(ctx);
                    });

                    canvas.addEventListener('mousemove', (e) => {
                        if (!drawing) return;
                        const { x, y } = pos(e);
                        ctx.lineTo(x, y);
                        ctx.stroke();
                    });

                    canvas.addEventListener('mouseup', () => { 
                        drawing = false; 
                    });
                    canvas.addEventListener('mouseleave', () => { drawing = false; });
                    "#))
                }
            }
        }
    }
}
