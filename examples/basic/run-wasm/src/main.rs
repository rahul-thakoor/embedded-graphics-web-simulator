fn main() {
    cargo_run_wasm::run_wasm_with_css(
        r#"
    body {
      display: flex;
      justify-content: center;
      flex-wrap: wrap;
      flex-direction: row;
      color: white;
      font-family: "Raleway Dots", cursive;
    }

    canvas {
      padding: 0;
      margin: auto;
      border-style: solid;
      border-color: #605e63;
      border-width: 2px;
      border-top-width: 20px;
      margin-bottom: 20px;
    }

    body {
      background-color: #171414;
    }

    header {
      margin-bottom: 20px;
      font-size: 40px;

      width: 100%;
      text-align: center;
    }

    div#custom-container {
      border: 1px solid #5ea1b0;
    }

    footer {
      display: block;
      position: fixed;
      bottom: 10px;
      left: 10px;
    }

    a {
      color: gainsboro;
    }
    "#,
    );
}
