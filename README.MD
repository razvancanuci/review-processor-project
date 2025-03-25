<span style="font-size: 20px;">**Review Processor Project**</span>

<div style="display:flex; justify-content:space-between">
    <img src="assets/vue.png" alt="Vue" width = 100 height = 100 />
    <img src ="assets/quasar.png" alt="Quasar" width = 100 height = 100 />
        <img src ="assets/tinymce.png" alt="TinyMCE" width = 100 height = 100 />
    <img src ="assets/rust.png" alt="Rust" width = 100 height = 100 />
    <img src ="assets/postgres.png" alt="Postgres" width = 100 height = 100 />
    <img src ="assets/sentry_logo.svg" alt="Sentry" width = 100 height = 100 />
    <img src ="assets/mistral.png" alt="Mistral" width = 100 height = 100 />
    <img src ="assets/make_logo.png" alt="Make" width = 100 height = 100 />
    <img src ="assets/docker.png" alt="Docker" width = 100 height = 100 />
    <img src ="assets/jira.png" alt="Jira" width = 100 height = 100 />
</div>

<br />
<br />
A web application that allows users (depends on the domain specified in backend's env vars) to submit reviews and suggestions. Suggestions are stored in a database and automatically processed using Mistral LLM and converted into Jira epics from Make.com.

🛠️ Technologies Used

Frontend: Vue.js, Quasar, TinyMCE

Backend: Rust

Database: PostgreSQL on Supabase

AI: Mistral (LLM) for task generation

Automation: Make.com

Project Management: Jira

Deployment: Docker on Render

Monitoring: Sentry.io


📝 How It Works

1. Users submit a review and suggestions.

2. If the review contains a suggestion, that suggestion is stored in a postgresql database.

3. If the review contains a suggestion, a Make.com automated scheduler triggers a pipeline.

4. The pipeline asks Mistral to create a title and a description for an epic.

5. The epic is created in Jira using the Jira API.


📺 Demo

Below there is the youtube video with the demo:

[![Watch the demo](https://img.youtube.com/vi/KDska7ylGl0/0.jpg)](https://www.youtube.com/watch?v=KDska7ylGl0)


Also the link from the app present on the demo (works only with @gmail.com domain):

https://review-processor-frontend.onrender.com/

**The host enters in sleep mode if it is not used, it should be accessed more times before it starts working. Also there is some posibility to not work anymore**


🎨 Design 

![Diagram](assets/review-processor-diagram.jpg)


📊 Monitoring

For the errors there are logs on Sentry.io:

![Sentry](assets/sentry_logs.png)
