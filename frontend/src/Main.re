[@mel.module "react-dom/client"]
external createRoot: Dom.element => ReactDOM.Client.root = "createRoot";

let rootElement =
  switch (ReactDOM.querySelector("#root")) {
  | Some(el) => el
  | None => raise(Failure("no #root element in document"))
  };

let root = ReactDOM.Client.createRoot(rootElement);

ReactDOM.Client.render(root, <CorpusContext> <App /> </CorpusContext>);
