import { defineBoot } from '#q-app/wrappers'
import * as Sentry from '@sentry/vue'


export default defineBoot(({ app, router }) => {

  Sentry.init({
    app,
    dsn: process.env.VITE_SENTRY_DSN,
    integrations: [
      Sentry.browserTracingIntegration({ router }),
      Sentry.replayIntegration(),
    ],
    tracePropagationTargets: ['http://localhost:9000', 'https://review-processor-frontend.onrender.com'],
    tracesSampleRate: 0.5,
    replaysSessionSampleRate: 0.1,
    replaysOnErrorSampleRate: 1.0,
    allowUrls: ['http://localhost:9000', 'https://review-processor-frontend.onrender.com']
  } as never);
});
