[@mel.module "./App.module.css"] external styles: Js.t({..}) = "default";

[@react.component]
let make = () => {
  let url = ReasonReactRouter.useUrl();
  let route = Router.ofUrl(url);
  let corpus = CorpusContext.useCorpus();

  let showCorpusNotice =
    switch (corpus.status, route) {
    | (CorpusContext.Ready(None), r) when r != Upload && r != NotFound =>
      true
    | _ => false
    };

  let body =
    switch (route) {
    | Login => <Login />
    | Upload => <Upload />
    | Profile => <StyleProfile />
    | Messages => <Messages />
    | Generate => <Generate />
    | History => <History />
    | NotFound => <Upload />
    };

  <div className={styles##app}>
    <Nav current=route />
    <main className={styles##main}>
      {showCorpusNotice
         ? <div className={styles##notice}>
             {React.string(
                "No corpus has been ingested yet. Head to Upload first.",
              )}
           </div>
         : React.null}
      body
    </main>
  </div>;
};
