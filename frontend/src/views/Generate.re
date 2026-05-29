[@mel.module "./Generate.module.css"] external styles: Js.t({..}) = "default";

type submitState =
  | Idle
  | Running
  | Failed(string)
  | Ready(Api.generateResponse);

[@react.component]
let make = () => {
  let corpus = CorpusContext.useCorpus();
  let (prompt, setPrompt) = React.useState(() => "");
  let (audience, setAudience) = React.useState(() => "email");
  let (nudge, setNudge) = React.useState(() => "");
  let (state, setState) = React.useState(() => Idle);
  let (tweakMode, setTweakMode) = React.useState(() => false);

  let corpusFingerprint =
    switch (corpus.status) {
    | Ready(Some(fp)) => Some(fp)
    | _ => None
    };

  let run = () =>
    if (String.length(String.trim(prompt)) == 0) {
      setState(_ => Failed("Enter a prompt first."));
    } else {
      setState(_ => Running);
      let req: Api.generateRequest = {
        prompt,
        audience: Api.audienceOfString(audience),
        retrievalCount: Some(4),
        nudge: String.length(nudge) > 0 ? Some(nudge) : None,
      };
      let _ =
        Client.postGenerate(req)
        |> Js.Promise.then_(res => {
             switch (res) {
             | Ok(r) => setState(_ => Ready(r))
             | Error(e) => setState(_ => Failed(Client.errorToString(e)))
             };
             Js.Promise.resolve();
           });
      ();
    };

  let onSubmit = _ => run();

  let onRegenerate = () => {
    setTweakMode(_ => false);
    run();
  };

  let onTweak = () => setTweakMode(_ => true);

  let showForm =
    switch (state) {
    | Idle
    | Failed(_) => true
    | Running => true
    | Ready(_) => tweakMode
    };

  let formDisabled =
    switch (state) {
    | Running => true
    | _ => false
    };

  <div className={styles##container}>
    <h1 className={styles##title}> {React.string("Generate")} </h1>
    {switch (corpus.status) {
     | Ready(None) =>
       <div className={styles##empty}>
         {React.string("Upload a corpus before generating.")}
       </div>
     | _ =>
       <>
         {showForm
            ? <div className={styles##form}>
                {tweakMode
                   ? <div className={styles##tweakNotice}>
                       {React.string(
                          "Tweak the prompt or nudge, then regenerate.",
                        )}
                     </div>
                   : React.null}
                <div className={styles##field}>
                  <label className={styles##label}>
                    {React.string("Prompt")}
                  </label>
                  <textarea
                    className={styles##textarea}
                    value=prompt
                    placeholder="e.g. write a follow-up email after a first-round interview"
                    onChange={e => {
                      let v = React.Event.Form.target(e)##value;
                      setPrompt(_ => v);
                    }}
                  />
                </div>
                <div className={styles##row}>
                  <div className={styles##field}>
                    <label className={styles##label}>
                      {React.string("Audience")}
                    </label>
                    <select
                      className={styles##select}
                      value=audience
                      onChange={e => {
                        let v = React.Event.Form.target(e)##value;
                        setAudience(_ => v);
                      }}>
                      <option value="email"> {React.string("Email")} </option>
                      <option value="slack"> {React.string("Slack")} </option>
                      <option value="sms"> {React.string("SMS")} </option>
                    </select>
                  </div>
                  <div className={styles##field}>
                    <label className={styles##label}>
                      {React.string("Nudge (optional)")}
                    </label>
                    <input
                      className={styles##input}
                      type_="text"
                      value=nudge
                      placeholder="make it shorter, less formal..."
                      onChange={e => {
                        let v = React.Event.Form.target(e)##value;
                        setNudge(_ => v);
                      }}
                    />
                  </div>
                </div>
                <button
                  className={styles##submit}
                  disabled=formDisabled
                  onClick=onSubmit>
                  {React.string(
                     switch (state) {
                     | Running => "Generating..."
                     | _ => tweakMode ? "Regenerate" : "Generate"
                     },
                   )}
                </button>
              </div>
            : React.null}
         {switch (state) {
          | Failed(msg) =>
            <div className={styles##error}> {React.string(msg)} </div>
          | Ready(resp) =>
            <ResultView
              response=resp
              corpusFingerprint
              onRegenerate
              regenerating={
                switch (state) {
                | Running => true
                | _ => false
                }
              }
              onTweak
            />
          | _ => React.null
          }}
       </>
     }}
  </div>;
};
