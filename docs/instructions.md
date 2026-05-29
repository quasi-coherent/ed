# Sentience Engineering Take Home -- Tone & Style

## Overview

Congrats on making it this far. We are excited about the potential to work with you and want to share this project as a way to
evaluate your skills and abilities.  You will have 48 hours from receipt of this page to submit your work.

### Philosophy

Use all the AI you want. In fact, we will strongly evaluate your ability to build with AI. One of the deliverables, in addition
to the project and code, is a writeup on how you approached the problem and how you used AI. Claude Code, Cursor, etc. is
encouraged and basically required.

### What We Are Looking For

#### Two things:

1. *Feature Complete Version*. Deliver on all of the requirements.
2. *A spike*. Equally or more importantly, we want you to show off a spike. A spike is something you are uniquely excited, passionate,
and skilled about. Show off what you are 99th percentile at in your take home.

### Deliverables

Please reply to the email thread you received this on with:

1. A web link to a deployed version of the project.
2. A link to the GitHub repo of the code you write.
3. In the repo, include a markdown file writeup about your approach to the problem, how you built it, and any considerations. Make sure to
specify your spike.

## Project - Build a Tone and Style Simulator

### Overview

Given a corpus of someone's real writing (emails, Slack messages, texts), build a web app that can generate new text that authentically
matches that person's tone, style, and voice.

### Requirements

#### Core Functionality

* Accept a corpus of text as input (upload a file or paste raw text). The corpus will contain a mix of emails, messages, and casual texts
from a single person.
* Analyze the corpus and extract identifiable style dimensions (e.g., formality level, sentence structure, vocabulary patterns, sign-off habits,
punctuation tendencies, emoji usage).
* Given a prompt or context (e.g., "write a follow-up email to a recruiter," "reply to this Slack message"), generate text that matches the voice
of the person in the corpus.
* Make it easy for a human to judge how well the output matches the original voice.

#### Evaluation

* Demonstrate that your tool works across different writing contexts (e.g., formal email, casual Slack message, short text reply). How you measure
and present output quality is up to you.
* Ideas: You have freedom — these are some examples of what you could do:
  - Build some form of scoring or confidence metric that indicates how well the generated text matches the corpus style. Explain your methodology.
* Include test cases with different prompt types (formal email, casual Slack message, short text reply) and show the output quality for each.

#### Interactive UI

* A clean web frontend where a user can: upload/paste a corpus, see the extracted style profile, input a writing prompt with context, and view the
generated output with the confidence score.
* The UI should make it easy to iterate: tweak the prompt, regenerate, compare outputs.

#### Hosting

* The app must be deployed to a publicly accessible URL. Include the live link in your submission.

#### Persistence

* Store generated outputs and their scores so a user can revisit previous generations.
* Store the extracted style profiles so a corpus doesn't need to be re-analyzed every session.

#### Considerations

* What happens when the corpus is small (under 20 messages)? How do you handle low-data scenarios?
* How do you distinguish between someone's style vs. the content/topic they write about?
* How would this scale if the corpus was 10,000+ messages?
* What are the privacy implications of ingesting someone's real communications?

### Corpus

Generate or source your own corpus of real human writing to test and build against. The corpus should contain a mix of emails, messages, and casual
texts from a single person.

Provide the corpus when you send your project to us so we can assess how closely you replicated your tone and style.
