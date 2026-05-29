[@mel.module "./StyleProfile.module.css"]
external styles: Js.t({..}) = "default";

let pct = (f: float): string =>
  Js.Float.toFixed(f *. 100.0, ~digits=1) ++ "%";

let num = (f: float): string => Js.Float.toFixed(f, ~digits=2);

let stat = (~label: string, ~value: string, ~gloss: string="", ()) =>
  <div className={styles##statRow} key=label>
    <div>
      <div className={styles##statLabel}> {React.string(label)} </div>
      {String.length(gloss) > 0
         ? <div className={styles##statGloss}> {React.string(gloss)} </div>
         : React.null}
    </div>
    <div className={styles##statValue}> {React.string(value)} </div>
  </div>;

let chip = (s: string) =>
  <span className={styles##chip} key=s> {React.string(s)} </span>;

let chipList = (~items: array(string)) =>
  if (Array.length(items) == 0) {
    <div className={styles##empty}> {React.string("(none)")} </div>;
  } else {
    <div className={styles##chips}>
      {items |> Array.map(chip) |> React.array}
    </div>;
  };

[@react.component]
let make = () => {
  let corpus = CorpusContext.useCorpus();
  let body =
    switch (corpus.status) {
    | CorpusContext.Loading =>
      <div className={styles##loading}>
        {React.string("Loading style profile...")}
      </div>
    | CorpusContext.Errored(msg) =>
      <div className={styles##error}> {React.string(msg)} </div>
    | CorpusContext.Ready(None) =>
      <div className={styles##empty}>
        {React.string("No corpus yet — upload some text first.")}
      </div>
    | CorpusContext.Ready(Some(fp)) =>
      let formalityPct = max(0.0, min(1.0, fp.formalityScore)) *. 100.0;
      <>
        <section className={styles##section}>
          <h2> {React.string("Formality")} </h2>
          <div className={styles##scale}>
            <div
              className={styles##scaleMarker}
              style={ReactDOM.Style.make(
                ~left=Js.Float.toString(formalityPct) ++ "%",
                (),
              )}
            />
          </div>
          <div className={styles##scaleLabels}>
            <span> {React.string("Casual")} </span>
            <span> {React.string(pct(fp.formalityScore))} </span>
            <span> {React.string("Formal")} </span>
          </div>
        </section>
        <section className={styles##section}>
          <h2> {React.string("Sentence structure")} </h2>
          {stat(
             ~label="Average sentence length",
             ~value=num(fp.avgSentenceLength) ++ " words",
             (),
           )}
          {stat(
             ~label="Length variance",
             ~value=num(fp.sentenceLengthVariance),
             ~gloss="Higher means more rhythmic variety",
             (),
           )}
        </section>
        <section className={styles##section}>
          <h2> {React.string("Rhythmic markers")} </h2>
          {stat(
             ~label="Exclamations",
             ~value=pct(fp.exclamationRatio),
             ~gloss="Exclamation marks per sentence",
             (),
           )}
          {stat(
             ~label="Ellipses",
             ~value=pct(fp.ellipsisRatio),
             ~gloss="Trailing ... per sentence",
             (),
           )}
          {stat(
             ~label="Contractions",
             ~value=pct(fp.contractionRatio),
             ~gloss="Share of words that are contractions (don't, I'm, etc.)",
             (),
           )}
          {stat(
             ~label="Hedging",
             ~value=pct(fp.hedgingRatio),
             ~gloss=
               "Sentences containing softening language (just, maybe, I think)",
             (),
           )}
          {stat(
             ~label="Emoji",
             ~value=num(fp.emojiFrequency) ++ " / msg",
             (),
           )}
        </section>
        <section className={styles##section}>
          <h2> {React.string("Common openers")} </h2>
          {chipList(~items=fp.commonOpeners)}
        </section>
        <section className={styles##section}>
          <h2> {React.string("Common closers")} </h2>
          {chipList(~items=fp.commonClosers)}
        </section>
        <div className={styles##subtitle}>
          {React.string(
             "Derived from " ++ string_of_int(fp.messageCount) ++ " messages.",
           )}
        </div>
      </>;
    };

  <div className={styles##container}>
    <h1 className={styles##title}> {React.string("Style profile")} </h1>
    body
  </div>;
};
