fn main() {
    cargo_run_wasm::run_wasm_with_css(
        r#"
        body {
          margin:0;
          padding:0;
          font-family: Arial, Helvetica, sans-serif;
          width: 100%;
          text-align: center;
          background-color: indigo;
          color:magenta;
        }

        p#text {
          background-color: black;
          padding: 0.5rem;
          border-bottom: 1px solid blueviolet;
        }
        div#graphics {
          filter: drop-shadow(0 0 0.75rem black);
        }
        
    "#,
    );
}
