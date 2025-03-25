import { defineBoot } from '#q-app/wrappers'
import vue3GoogleLogin from 'vue3-google-login'

export default defineBoot(({ app }) => {

  app.use(vue3GoogleLogin, {
    clientId: process.env.VITE_GOOGLE_CLIENT_ID,
  } as never);
});
